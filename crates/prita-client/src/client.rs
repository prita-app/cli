//! Transport over the Prita GraphQL endpoint.

use cynic::{GraphQlResponse, Operation};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::error::Error;

const USER_AGENT: &str = concat!("prita-cli/", env!("CARGO_PKG_VERSION"));

/// A client for the Prita GraphQL API.
///
/// Sends `POST <endpoint>` with `Authorization: Bearer <token>`.
pub struct PritaClient {
    http: reqwest::Client,
    endpoint: String,
    token: String,
}

impl PritaClient {
    /// Build a client for `endpoint`, authenticating with `token`.
    pub fn new(token: impl Into<String>, endpoint: impl Into<String>) -> Result<Self, Error> {
        let http = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
        Ok(Self {
            http,
            endpoint: endpoint.into(),
            token: token.into(),
        })
    }

    /// Run a typed cynic operation and return its response data.
    ///
    /// Maps a 401 to [`Error::Unauthenticated`] and a non-empty `errors` array
    /// to [`Error::GraphQl`].
    pub async fn run<T, V>(&self, operation: Operation<T, V>) -> Result<T, Error>
    where
        T: DeserializeOwned,
        V: Serialize,
    {
        let resp = self
            .http
            .post(&self.endpoint)
            .bearer_auth(&self.token)
            .json(&operation)
            .send()
            .await?;

        let status = resp.status();
        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(Error::Unauthenticated);
        }

        let response: GraphQlResponse<T> = resp.json().await?;

        if let Some(errors) = response.errors
            && !errors.is_empty()
        {
            let message = errors
                .into_iter()
                .map(|e| e.message)
                .collect::<Vec<_>>()
                .join("; ");
            return Err(Error::GraphQl(message));
        }

        response.data.ok_or_else(|| {
            if status.is_success() {
                Error::GraphQl("server returned no data".into())
            } else {
                Error::GraphQl(format!("HTTP {status}"))
            }
        })
    }
}
