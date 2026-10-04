//! The main Plaid API client.

use crate::middleware::{Next, Request, ResponseBody};
use crate::{Config, PlaidError};
use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::value::RawValue;
use std::borrow::Cow;
use std::sync::Arc;

/// The main Plaid API client.
///
/// Clone is cheap — `PlaidClient` uses an `Arc` internally.
#[derive(Debug, Clone)]
pub struct PlaidClient {
    inner: Arc<PlaidClientInner>,
}

#[derive(Debug)]
pub(crate) struct PlaidClientInner {
    http: Client,
    config: Config,
}

/// Plaid's error response body, returned on non-2xx responses.
#[derive(Debug, Deserialize)]
struct PlaidErrorBody {
    error_type: String,
    error_code: String,
    error_message: String,
    request_id: String,
}

/// The terminal send at the end of the middleware chain: performs the
/// actual HTTP POST and maps the response.
pub(crate) async fn send(
    inner: &PlaidClientInner,
    request: &Request,
) -> Result<ResponseBody, PlaidError> {
    let url = format!("{}{}", inner.base_url().trim_end_matches('/'), request.path);
    // Transport errors only; response decoding is mapped separately so
    // retry logic can tell "never reached the server" apart from
    // "server replied with a malformed body".
    let response = inner.http.post(&url).json(&request.body).send().await?;

    if response.status().is_success() {
        let text = response.text().await.map_err(PlaidError::Decode)?;
        Ok(RawValue::from_string(text)?)
    } else {
        let status = response.status();
        let text = response.text().await.map_err(PlaidError::Decode)?;
        match serde_json::from_str::<PlaidErrorBody>(&text) {
            Ok(body) => Err(PlaidError::Api {
                status: status.as_u16(),
                error_type: body.error_type,
                error_code: body.error_code,
                error_message: body.error_message,
                request_id: body.request_id,
            }),
            Err(_) => Err(PlaidError::UnexpectedStatus {
                status: status.as_u16(),
                body: text,
            }),
        }
    }
}

impl PlaidClientInner {
    fn base_url(&self) -> Cow<'_, str> {
        match &self.config.base_url {
            Some(url) => Cow::Borrowed(url.as_str()),
            None => match self.config.environment {
                crate::Environment::Sandbox => Cow::Borrowed("https://sandbox.plaid.com"),
                crate::Environment::Production => Cow::Borrowed("https://production.plaid.com"),
            },
        }
    }
}

impl PlaidClient {
    /// Create a new client with the given configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying HTTP client cannot be built.
    pub fn new(config: Config) -> Result<Self, PlaidError> {
        let http = Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(PlaidError::HttpClient)?;

        let inner = Arc::new(PlaidClientInner { http, config });

        Ok(Self { inner })
    }

    /// Returns the base URL for the configured environment, or the
    /// `Config::base_url` override when set.
    #[must_use]
    pub fn base_url(&self) -> Cow<'_, str> {
        self.inner.base_url()
    }

    /// POST a JSON body to a Plaid endpoint, merging `client_id` and
    /// `secret` into the request body as Plaid expects.
    ///
    /// The request runs through the middleware chain configured on
    /// [`Config::middleware`]. On 2xx the response is deserialized into
    /// `R`. On non-2xx the Plaid error body is mapped to
    /// [`PlaidError::Api`], falling back to [`PlaidError::Unknown`] when
    /// the body is not a Plaid error.
    pub(crate) async fn post<B, R>(&self, path: &str, body: &B) -> Result<R, PlaidError>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        let mut value = serde_json::to_value(body)?;
        let map = value
            .as_object_mut()
            .ok_or_else(|| PlaidError::Unknown("request body must be a JSON object".into()))?;
        map.insert("client_id".into(), self.client_id().expose_secret().into());
        map.insert("secret".into(), self.secret().expose_secret().into());

        let request = Request {
            path: path.to_owned(),
            body: value,
        };
        let next = Next {
            chain: &self.inner.config.middleware,
            client: &self.inner,
        };
        let body = next.run(&request).await?;
        Ok(serde_json::from_str(body.get())?)
    }

    /// Returns the client ID.
    pub(crate) fn client_id(&self) -> &SecretString {
        &self.inner.config.client_id
    }

    /// Returns the secret.
    pub(crate) fn secret(&self) -> &SecretString {
        &self.inner.config.secret
    }
}
