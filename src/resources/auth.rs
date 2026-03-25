use std::sync::Arc;

use serde_json::json;

use crate::client::ClientInner;
use crate::error::{Result, SWError};

pub struct AuthResource {
    inner: Arc<ClientInner>,
}

impl AuthResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    /// Authenticate with the API.
    ///
    /// Sends credentials to `POST /_/security/login` and stores the returned
    /// Bearer token for all subsequent requests.
    pub async fn login(
        &self,
        client_id: &str,
        auth_token: &str,
        login: &str,
        password: &str,
    ) -> Result<serde_json::Value> {
        let body = json!({
            "clientId": client_id,
            "authToken": auth_token,
            "login": login,
            "password": password,
        });

        let response = self
            .inner
            .request(
                reqwest::Method::POST,
                "/_/security/login",
                Some(&body),
                None,
            )
            .await?;

        let token = response
            .get("token")
            .and_then(|t| t.as_str())
            .ok_or_else(|| SWError::Other("No token in login response".to_string()))?;

        self.inner.set_token(token.to_string()).await;

        Ok(response)
    }
}
