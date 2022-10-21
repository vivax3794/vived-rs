//! Ratelimiter and error handling client

use serde::Deserialize;
use std::sync::Arc;
use std::{future::Future, time::Duration};
use tokio::sync::{Notify, RwLock, Semaphore};

use log::{debug, error, info, trace, warn};

// Rate limits were hit at 40 req/30 secs, but not o 30 req/30 secs, so we will keep to that!
/// Number of allowed requests that can happen at once
const CONCURRENT_REQUEST: usize = 30;
/// How many seconds should the request permit be locked down after a request
const LOCK_HOLD_DURATION: u64 = 30;

/// What action should the ratelimiter code take based on the result of the api call
enum ApiResultAction<R> {
    /// Return the given value to the caller
    /// (This might actually either be a Ok() or Err())
    Return(R),
    /// Activate ratelimit lock and  retry after the specified seconds
    RetryAfter(u64),
    /// Active ratelimit lock and retry with exponential backoff
    RetryWithBackoff,
}

/// A error description
#[derive(Deserialize, Debug)]
pub struct GuildedError {
    /// Error code
    pub code: String,
    /// Message detailing the error
    pub message: String,
    /// this information is based on the specific error, and contains additional information
    pub meta: Option<serde_json::Value>,
}

/// An error that can be produced during the course of making a request
#[derive(Debug)]
pub enum ApiError {
    /// The library does not have a specific case for this error
    Unknown(reqwest::Error),
    /// And error occurred with parsing the returned json data
    /// This will also produce a `debug` log with the raw content data
    JsonError(serde_json::Error),
    /// A error occurred and guilded provided us with a nice explanation
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
/// You shouldn't need to implement this your self, you can if there are new routes that we don't support yet
/// But hopefully we should get to it soon enough
pub trait Endpoint<R> {
    /// Create the request that will be sent to api
    fn build(&self, client: &reqwest::Client) -> reqwest::RequestBuilder;
    /// Convert from the raw api response to the needed result
    ///
    /// # Errors
    /// errors if the raw string cant be parsed into the expected json structure.
    fn from_raw(raw: &str) -> Result<R, serde_json::Error>;
}

/// This client handles ratelimiter and errors.
/// This means that you could just do a while true loop and spam its methods and it will make sure you don't get ratelimited.
/// THO! sending 100 requests without triggering a ratelimit is gonna take around 90 seconds :P
/// so like don't if you don't actually need
#[derive(Debug)]
pub struct Client {
    /// The `reqwest` client to use
    client: RwLock<reqwest::Client>,
    /// This is used to keep the number of concurrent tasks within a specific amount
    sem: Arc<Semaphore>,
}

impl Client {
    /// Create a new api client using the provided token
    ///
    /// # Panics
    /// if provided token contains invalid chars
    ///
    /// or if there is an error constructing the reqwest client, which can happen
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
            "RATELIMITER SETTINGS: max concurrent requests: {}",
            CONCURRENT_REQUEST
        );
        info!(
            "RATELIMITER SETTINGS: lock hold time: {} seconds",
            LOCK_HOLD_DURATION
        );

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}")
                .parse()
                .expect("Invalid characters in provided token"),
        );

        let client = reqwest::Client::builder()
            .user_agent(user_agent)
            .default_headers(headers)
            .build()
            .expect("Error creating reqwest client");

        Self {
            sem: Arc::new(Semaphore::new(CONCURRENT_REQUEST)),
            client: RwLock::new(client),
        }
    }

    /// Handle ratelimits and retry logic
    /// operates on `ApiResultAction`
    async fn handle_ratelimit<C, F, R>(&self, closure: C) -> R
    where
        C: Fn() -> F,
        F: Future<Output = ApiResultAction<R>>,
    {
        let permit = Arc::clone(&self.sem)
            .acquire_owned()
            .await
            .expect("Ratelimiter semaphore has been closed unexpectedly");

        let mut backoff_amount: u64 = 20;

        let mut lockdown_permits = None;

        let result = loop {
            match closure().await {
                ApiResultAction::Return(value) => break value,
                ApiResultAction::RetryAfter(wait_amount) => {
                    warn!(
                        "Ratelimit hit, blocking all requests for {} seconds",
                        wait_amount
                    );

                    lockdown_permits = Some(
                        Arc::clone(&self.sem)
                            .acquire_many_owned(self.sem.available_permits() as u32)
                            .await
                            .expect("Ratelimiter semaphore has been closed unexpectedly"),
                    );

                    tokio::time::sleep(Duration::from_secs(wait_amount)).await;
                }
                ApiResultAction::RetryWithBackoff => {
                    warn!(
                        "Ratelimit hit, blocking all requests for {} seconds (BACKOFF MODE)",
                        backoff_amount
                    );

                    lockdown_permits = Some(
                        Arc::clone(&self.sem)
                            .acquire_many_owned(self.sem.available_permits() as u32)
                            .await
                            .expect("Ratelimiter semaphore has been closed unexpectedly")
                    );

                    tokio::time::sleep(Duration::from_secs(backoff_amount)).await;
                    backoff_amount *= 2;
                }
            }
        };

        if let Some(permits) = lockdown_permits {
            permits.forget();
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
    /// If a ratelimit is hit and the "Retry-After" header is malformed
    pub async fn make_request<'a, E, R>(&self, builder: E) -> Result<R, ApiError>
    where
        E: Endpoint<R>,
    {
        self.handle_ratelimit(|| async {
            let client = self.client.read().await;
            let request = builder.build(&client).build().expect("invalid request");

            debug!("making request");
            trace!("URL: {}", request.url());
            trace!("METHOD: {}", request.method());
            trace!("HEADERS: {:#?}", request.headers());

            if let Some(body) = request.body().and_then(reqwest::Body::as_bytes) {
                trace!("BODY: {}", String::from_utf8_lossy(body));
            } else {
                trace!("NO VALID BODY");
            }

            let res = client.execute(request).await;

            let res = match res {
                Ok(value) => value,
                Err(error) => return ApiResultAction::Return(Err(ApiError::Unknown(error))),
            };

            let status = res.status();

            if status.is_success() {
                let content = res.text().await.expect("response data was not valid text");
                E::from_raw(&content)
                    .map_err(|err| {
                        error!("RESPONSE BODY: {}", content);
                        err.into()
                    })
                    .into()
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
                // we could use the .json method, but we want access to the hole content in the event it isn't json
                // (or our json scheme just isn't valid)
                let content = res
                    .text()
                    .await
                    .expect("Expected response content to be text");
                ApiResultAction::Return(Err(match serde_json::from_str::<GuildedError>(&content) {
                    Ok(error) => ApiError::Guilded(error),
                    Err(error) => {
                        error!("RESPONSE BODY: {}", content);
                        ApiError::JsonError(error)
                    }
                }))
            }
        })
        .await
    }
}
