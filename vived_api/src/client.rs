use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::sync::Arc;
use std::{future::Future, time::Duration};
use tokio::sync::{Notify, RwLock, Semaphore};

use log::{trace, info, warn, debug};

/// 5 seems like a nice number
const CONCURRENT_REQUEST: usize = 5;
const LOCK_HOLD_DURATION: u64 = 30;

enum ApiResultAction<R> {
    Return(R),
    RetryAfter(u64),
    RetryWithBackoff,
}

#[derive(Deserialize, Debug)]
pub struct GuildedError {
    pub code: String,
    pub message: String,
    pub meta: Option<serde_json::Value>,
}

#[derive(Debug)]
pub enum ApiError {
    Unknown(reqwest::Error),
    JsonError(serde_json::Error),
    Guilded(GuildedError),
}

impl<R> From<R> for ApiResultAction<R> {
    fn from(v: R) -> Self {
        Self::Return(v)
    }
}

pub trait Endpoint<R> {
    fn build(&self, client: &reqwest::Client) -> reqwest::RequestBuilder;
}

pub struct ApiClient {
    sem: Arc<Semaphore>,
    client: Arc<RwLock<reqwest::Client>>,
    ratelimit_lock_active: RwLock<bool>,
    ratelimit_unlocked_notification: Arc<Notify>,
}

impl ApiClient {
    #[must_use]
    pub fn new(token: &str) -> Self {
        let user_agent = format!(
                "library: vived, version: {}, rustc version: {}",
                version::version!(),
                rustc_version_runtime::version()
            );
        info!("using User-Agent: {}", user_agent);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {token}").parse().unwrap());

        let client = reqwest::Client::builder()
            .user_agent(user_agent)
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            sem: Arc::new(Semaphore::new(CONCURRENT_REQUEST)),
            client: Arc::new(RwLock::new(client)),
            ratelimit_lock_active: RwLock::new(false),
            ratelimit_unlocked_notification: Arc::new(Notify::new()),
        }
    }

    async fn handle_ratelimit<C, F, R>(&self, closure: C) -> R
    where
        C: Fn() -> F,
        F: Future<Output = ApiResultAction<R>>,
    {
        // if ratelimit lock is in effect wait until it is released
        if *self.ratelimit_lock_active.read().await {
            self.ratelimit_unlocked_notification
                .clone()
                .notified()
                .await;
        }

        trace!("getting permit to make request");
        let permit = self.sem.clone().acquire_owned().await.unwrap();

        let mut backoff_amount: u64 = 20;
        let mut ratelimit_lock_activated_by_us: bool = false;

        let result = loop {
            match closure().await {
                ApiResultAction::Return(value) => break value,
                ApiResultAction::RetryAfter(wait_amount) => {
                    warn!(
                        "Ratelimit hit, blocking all requests for {} seconds",
                        wait_amount
                    );

                    *self.ratelimit_lock_active.write().await = true;
                    ratelimit_lock_activated_by_us = true;

                    tokio::time::sleep(Duration::from_secs(wait_amount)).await;
                }
                ApiResultAction::RetryWithBackoff => {
                    warn!(
                        "Ratelimit hit, blocking all requests for {} seconds (BACKOFF MODE)",
                        backoff_amount
                    );

                    *self.ratelimit_lock_active.write().await = true;
                    ratelimit_lock_activated_by_us = true;

                    tokio::time::sleep(Duration::from_secs(backoff_amount)).await;
                    backoff_amount *= 2;
                }
            }
        };
        // if this task has activated the ratelimit lock it needs to release it
        if ratelimit_lock_activated_by_us {
            *self.ratelimit_lock_active.write().await = false;
            self.ratelimit_unlocked_notification.notify_waiters();
        }

        // Make permit last longer than the call so we don't get requests too quickly
        tokio::spawn(async move {
            trace!("holding permit for {LOCK_HOLD_DURATION} seconds");
            tokio::time::sleep(Duration::from_secs(LOCK_HOLD_DURATION)).await;
            drop(permit);
            trace!("dropped permit");
        });

        result
    }

    // Api?
    // * client.make_request(SomeBuilder::new().with_thing(abc)).await?;
    // ? what happens if a user calls build / send, themselves?
    // * they cant, because builder should need `Client` access, and we don't provide that
    // * if they do that manually they are just asking to mess up :P

    pub async fn make_request<'a, B, R>(&self, builder: B) -> Result<R, ApiError>
    where
        B: Endpoint<R>,
        R: DeserializeOwned,
    {
        self.handle_ratelimit(|| async {
            let request = builder
                .build(&*self.client.clone().read_owned().await);
            
            trace!("making request: {:?}", request);

            let res = request.send().await;

            let res = match res {
                Ok(value) => value,
                Err(error) => return ApiResultAction::Return(Err(ApiError::Unknown(error))),
            };

            let status = res.status();

            if status.is_success() {
                let content = res.text().await.unwrap();

                ApiResultAction::Return(match serde_json::from_str(&content) {
                    Ok(res) => Ok(res),
                    Err(error) => {
                        debug!("RESPONSE BODY: {}", content);
                        Err(ApiError::JsonError(error))
                    }
                })
            } else if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                if let Some(wait_amount) = res.headers().get("Retry-After") {
                    let wait_amount = wait_amount.to_str().unwrap().parse().unwrap();
                    ApiResultAction::RetryAfter(wait_amount)
                } else {
                    ApiResultAction::RetryWithBackoff
                }
            } else {
                // we could use the .json method, but we want access to the hole content in the event it isnt json
                // (or our json scheme just isn't valid)
                let content = res.text().await.unwrap();
                ApiResultAction::Return(Err(match serde_json::from_str::<GuildedError>(&content) {
                    Ok(error) => ApiError::Guilded(error),
                    Err(error) => {
                        debug!("RESPONSE BODY: {}", content);
                        ApiError::JsonError(error)
                    },
                }))
            }
        })
        .await
    }
}
