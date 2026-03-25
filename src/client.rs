use std::sync::Arc;

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde_json::Value;
use tokio::sync::RwLock;

use crate::error::{raise_for_status, Result, SWError};
use crate::resources::{
    AccountResource, AuthResource, CommissionsResource, FilesResource, KanbansResource,
    PlacesResource, ProductsResource, ServicedProductsResource, UserProfilesResource,
    UsersResource,
};

pub(crate) struct ClientInner {
    pub http: reqwest::Client,
    pub base_url: String,
    pub token: RwLock<Option<String>>,
}

impl ClientInner {
    pub async fn request(
        &self,
        method: reqwest::Method,
        path: &str,
        json: Option<&Value>,
        query: Option<&[(String, String)]>,
    ) -> Result<Value> {
        let url = format!("{}{}", self.base_url, path);
        let mut builder = self.http.request(method, &url);

        // Auth header
        if let Some(token) = self.token.read().await.as_ref() {
            builder = builder.header(AUTHORIZATION, format!("Bearer {}", token));
        }

        // Default headers
        builder = builder
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json");

        if let Some(query) = query {
            builder = builder.query(query);
        }

        if let Some(body) = json {
            builder = builder.json(body);
        }

        let response = builder.send().await.map_err(|e| {
            if e.is_connect() || e.is_timeout() {
                SWError::Connection(e.to_string())
            } else {
                SWError::Connection(e.to_string())
            }
        })?;

        let status = response.status().as_u16();

        if status == 204 {
            return Ok(Value::Object(serde_json::Map::new()));
        }

        let data: Value = response
            .json()
            .await
            .unwrap_or(Value::Object(serde_json::Map::new()));

        if status >= 400 {
            return Err(raise_for_status(status, data));
        }

        Ok(data)
    }

    pub async fn request_multipart(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
    ) -> Result<Value> {
        let url = format!("{}{}", self.base_url, path);
        let mut builder = self.http.post(&url).header(ACCEPT, "application/json");

        if let Some(token) = self.token.read().await.as_ref() {
            builder = builder.header(AUTHORIZATION, format!("Bearer {}", token));
        }

        builder = builder.multipart(form);

        let response = builder
            .send()
            .await
            .map_err(|e| SWError::Connection(e.to_string()))?;

        let status = response.status().as_u16();
        let data: Value = response
            .json()
            .await
            .unwrap_or(Value::Object(serde_json::Map::new()));

        if status >= 400 {
            return Err(raise_for_status(status, data));
        }

        Ok(data)
    }

    pub async fn set_token(&self, token: String) {
        *self.token.write().await = Some(token);
    }
}

/// Async client for the Serwis Planner API.
///
/// # Examples
/// ```no_run
/// use serwis_planner::SerwisPlanner;
///
/// # async fn example() -> serwis_planner::error::Result<()> {
/// let client = SerwisPlanner::new("https://api.example.com");
/// client.auth().login("client_id", "auth_token", "login", "password").await?;
///
/// let me = client.me().await?;
/// let companies = client.account().companies().list(None).await?;
/// # Ok(())
/// # }
/// ```
pub struct SerwisPlanner {
    pub(crate) inner: Arc<ClientInner>,
}

impl SerwisPlanner {
    pub fn new(api_url: &str) -> Self {
        Self::with_options(api_url, None, None)
    }

    pub fn with_options(
        api_url: &str,
        timeout_secs: Option<u64>,
        user_agent: Option<&str>,
    ) -> Self {
        let mut headers = HeaderMap::new();
        let ua = user_agent.unwrap_or("SerwisPlanner/0.1.0 (Rust)");
        headers.insert(USER_AGENT, HeaderValue::from_str(ua).unwrap());

        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(timeout_secs.unwrap_or(30)))
            .default_headers(headers)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            inner: Arc::new(ClientInner {
                http,
                base_url: api_url.trim_end_matches('/').to_string(),
                token: RwLock::new(None),
            }),
        }
    }

    /// Set the Bearer token manually (alternative to login).
    pub async fn set_token(&self, token: &str) {
        self.inner.set_token(token.to_string()).await;
    }

    /// Get current user data (`GET /api/me`).
    pub async fn me(&self) -> Result<Value> {
        self.inner
            .request(reqwest::Method::GET, "/api/me", None, None)
            .await
    }

    /// Get mobile phases config (`GET /api/mobile_service_app_configs/1`).
    pub async fn mobile_phases_config(&self) -> Result<Value> {
        self.inner
            .request(
                reqwest::Method::GET,
                "/api/mobile_service_app_configs/1",
                None,
                None,
            )
            .await
    }

    /// Create user geolocations (`POST /api/user_geolocations`).
    pub async fn create_user_geolocations(&self, data: &Value) -> Result<Value> {
        self.inner
            .request(
                reqwest::Method::POST,
                "/api/user_geolocations",
                Some(data),
                None,
            )
            .await
    }

    // --- Resource accessors ---

    pub fn auth(&self) -> AuthResource {
        AuthResource::new(self.inner.clone())
    }

    pub fn account(&self) -> AccountResource {
        AccountResource::new(self.inner.clone())
    }

    pub fn commissions(&self) -> CommissionsResource {
        CommissionsResource::new(self.inner.clone())
    }

    pub fn files(&self) -> FilesResource {
        FilesResource::new(self.inner.clone())
    }

    pub fn kanbans(&self) -> KanbansResource {
        KanbansResource::new(self.inner.clone())
    }

    pub fn places(&self) -> PlacesResource {
        PlacesResource::new(self.inner.clone())
    }

    pub fn products(&self) -> ProductsResource {
        ProductsResource::new(self.inner.clone())
    }

    pub fn serviced_products(&self) -> ServicedProductsResource {
        ServicedProductsResource::new(self.inner.clone())
    }

    pub fn users(&self) -> UsersResource {
        UsersResource::new(self.inner.clone())
    }

    pub fn user_profiles(&self) -> UserProfilesResource {
        UserProfilesResource::new(self.inner.clone())
    }
}
