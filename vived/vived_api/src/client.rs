//! Ratelimiting and error handling client

use serde::Deserialize;
use std::sync::Arc;
use std::{future::Future, time::Duration};
use tokio::sync::{Notify, RwLock, Semaphore};

use log::{debug, info, trace, warn};

// Rate limits were hit at 40 req/30 secs, but not o 30 req/30 secs, so we will keep to that!
/// Number of allowed requests that can happen at once
const CONCURRENT_REQUEST: usize = 30;
/// How many seconds should the request permit be locked down after a request
const LOCK_HOLD_DURATION: u64 = 30;

/// What action should the ratelimiting code take based on the result of the api call
enum ApiResultAction<R> {
    /// Return the given value to the caller
    /// (This might actually either be a Ok() or Err())
    Return(R),
    /// Activate ratelimit lock and  retry after the specified seconds
    RetryAfter(u64),
    /// Active ratelimit lock and rety with exponential backoff
    RetryWithBackoff,
}

/// A error description
#[derive(Deserialize, Debug)]
pub struct GuildedError {
    /// Error code
    pub code: String,
    /// Message detalinig the error
    pub message: String,
    /// this information is based on the specific error, and contains additional information
    pub meta: Option<serde_json::Value>,
}

/// An error that can be produced during the course of making a request
#[derive(Debug)]
pub enum ApiError {
    /// The library does not have a secific case for this error
    Unknown(reqwest::Error),
    /// And error occured with parsing the returned json data
    /// This will also produce a `debug` log with the raw content data
    JsonError(serde_json::Error),
    /// A error occured and guilded provided us with a nice explanation
    Guilded(GuildedError),
}

impl From<GuildedError> for ApiError {
    fn from(v: GuildedError) -> Self {
        Self::Guilded(v)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(v: serde_json::Error) -> Self {
        Self::JsonError(v)
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(v: reqwest::Error) -> Self {
        Self::Unknown(v)
    }
}

impl<R> From<R> for ApiResultAction<R> {
    fn from(v: R) -> Self {
        Self::Return(v)
    }
}

/// An endpoint details to the client how to perform an action
/// # Note
/// You shouldnt need to implement this your self, you can if there are new routes that we dont support yet
/// But hopeflly we should get to it soon enough
pub trait Endpoint<R> {
    /// Create the request that will be sent to api
    fn build(&self, client: &reqwest::Client) -> reqwest::RequestBuilder;
    /// Convert from the raw api response to the needed result
    /// 
    /// # Errors
    /// errors if the raw string cant be parsed into the expected json structure.
    fn from_raw(raw: String) -> Result<R, serde_json::Error>;
}


/// This client handles ratelimiting and errors.
/// This means that you could just do a while true loop and spam its methods and it will make sure you dont get ratelimited.
/// THO! sending 100 requests without trigring a ratelimit is gonna take around 90 seconds :P 
/// so like dont if you dont actually need
#[derive(Debug, Clone)]
pub struct Client {
    /// The `reqwest` client to use
    client: Arc<RwLock<reqwest::Client>>,
    /// This is used to keep the number of concurrent tasks within a specific amount
    sem: Arc<Semaphore>,
    /// Has a ratelimit been hit?
    /// if it has wait!
    ratelimit_lock_active: Arc<RwLock<bool>>,
    /// Listen for this notify when a ratelimit is in effect
    /// its activation signals the all clear to continue requests
    ratelimit_unlocked_notification: Arc<Notify>,
}

impl Client {
    /// Create a new api client using the provided token
    /// 
    /// # Panics
    /// if provided token contains invalid chars
    /// 
    /// or if there is an error constructing the reqwest clinet, which can happen 
    /// when there is no resolver or tls backend found on the system. 
    #[must_use]
    pub fn new(token: &str) -> Self {
        let user_agent = format!(
            "library: vived, version: {}, rustc version: {}",
            version::version!(),
            rustc_version_runtime::version()
        );

        info!("using User-Agent: {}", user_agent);
        info!(
            "RATELIMITING SETTINGS: max concurrent requests: {}",
            CONCURRENT_REQUEST
        );
        info!(
            "RATELIMITING SETTINGS: lock hold time: {} seconds",
            LOCK_HOLD_DURATION
        );

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse().expect("Invalid characthers in provided token"),
        );

        let client = reqwest::Client::builder()
            .user_agent(user_agent)
            .default_headers(headers)
            .build()
            .expect("Error creating reqwest client");

        Self {
            sem: Arc::new(Semaphore::new(CONCURRENT_REQUEST)),
            client: Arc::new(RwLock::new(client)),
            ratelimit_lock_active: Arc::new(RwLock::new(false)),
            ratelimit_unlocked_notification: Arc::new(Notify::new()),
        }
    }

    /// Handle ratelimits and retry logic
    /// operates on `ApiResultAction`
    async fn handle_ratelimit<C, F, R>(&self, closure: C) -> R
    where
        C: Fn() -> F,
        F: Future<Output = ApiResultAction<R>>,
    {
        // if ratelimit lock is in effect wait until it is released
        if *self.ratelimit_lock_active.read().await {
            warn!("RATELIMITING LOCK IS IN EFFECT, blocking request until ratelimit is released");
            Arc::clone(&self.ratelimit_unlocked_notification)
                .notified()
                .await;
        }

        trace!("getting permit to make request");
        if self.sem.available_permits() == 0 {
            warn!("All permits used, will now wait for other requests to exit lock period");
        }
        let permit = Arc::clone(&self.sem)
            .acquire_owned()
            .await
            .expect("Ratelimiting semapore has been closed unexpectdly");

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

    /// Make a request to the guilded api using the provided endpoint builder
    ///
    /// # Errors
    /// If there is a connection error or an error parsing the return json data
    ///
    /// # Panics
    /// If a ratelimit is hit and the "Rety-After" header is mallformed
    pub async fn make_request<'a, E, R>(&self, builder: E) -> Result<R, ApiError>
    where
        E: Endpoint<R>,
    {
        self.handle_ratelimit(|| async {
            let request = builder.build(&*Arc::clone(&self.client).read_owned().await);

            trace!("making request: {:?}", request);

            let res = request.send().await;

            let res = match res {
                Ok(value) => value,
                Err(error) => return ApiResultAction::Return(Err(ApiError::Unknown(error))),
            };

            let status = res.status();

            if status.is_success() {
                let content = res.text().await.expect("response data was not valid text");
                E::from_raw(content).map_err(ApiError::from).into()
            } else if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                if let Some(wait_amount) = res.headers().get("Retry-After") {
                    let wait_amount = wait_amount
                        .to_str()
                        .expect("Retry-After header was not valid text")
                        .parse()
                        .expect("Retry-After header value was not a valid number");
                    ApiResultAction::RetryAfter(wait_amount)
                } else {
                    ApiResultAction::RetryWithBackoff
                }
            } else {
                // we could use the .json method, but we want access to the hole content in the event it isnt json
                // (or our json scheme just isn't valid)
                let content = res
                    .text()
                    .await
                    .expect("Expected response content to be text");
                ApiResultAction::Return(Err(match serde_json::from_str::<GuildedError>(&content) {
                    Ok(error) => ApiError::Guilded(error),
                    Err(error) => {
                        debug!("RESPONSE BODY: {}", content);
                        ApiError::JsonError(error)
                    }
                }))
            }
        })
        .await
    }
}
