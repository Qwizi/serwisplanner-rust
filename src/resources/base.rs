use std::sync::Arc;

use serde_json::Value;

use crate::client::ClientInner;
use crate::error::Result;
use crate::params::QueryParams;

/// Base resource providing standard CRUD operations.
pub struct Resource {
    pub(crate) inner: Arc<ClientInner>,
    pub(crate) path: String,
}

impl Resource {
    pub(crate) fn new(inner: Arc<ClientInner>, path: &str) -> Self {
        Self {
            inner,
            path: path.to_string(),
        }
    }

    fn build_query(params: Option<&QueryParams>) -> Option<Vec<(String, String)>> {
        params.map(|p| p.build().into_iter().collect())
    }

    /// List resources (`GET /path`).
    pub async fn list(&self, params: Option<&QueryParams>) -> Result<Value> {
        let query = Self::build_query(params);
        let query_ref = query.as_deref();
        self.inner
            .request(reqwest::Method::GET, &self.path, None, query_ref)
            .await
    }

    /// Retrieve a single resource (`GET /path/{id}`).
    pub async fn retrieve(&self, id: u64, params: Option<&QueryParams>) -> Result<Value> {
        let url = format!("{}/{}", self.path, id);
        let query = Self::build_query(params);
        let query_ref = query.as_deref();
        self.inner
            .request(reqwest::Method::GET, &url, None, query_ref)
            .await
    }

    /// Create a resource (`POST /path`).
    pub async fn create(&self, data: &Value, params: Option<&QueryParams>) -> Result<Value> {
        let query = Self::build_query(params);
        let query_ref = query.as_deref();
        self.inner
            .request(reqwest::Method::POST, &self.path, Some(data), query_ref)
            .await
    }

    /// Update a resource (`PUT /path/{id}`).
    pub async fn update(
        &self,
        id: u64,
        data: &Value,
        params: Option<&QueryParams>,
    ) -> Result<Value> {
        let url = format!("{}/{}", self.path, id);
        let query = Self::build_query(params);
        let query_ref = query.as_deref();
        self.inner
            .request(reqwest::Method::PUT, &url, Some(data), query_ref)
            .await
    }

    /// Partial update (`PATCH /path/{id}`).
    pub async fn partial_update(
        &self,
        id: u64,
        data: &Value,
        params: Option<&QueryParams>,
    ) -> Result<Value> {
        let url = format!("{}/{}", self.path, id);
        let query = Self::build_query(params);
        let query_ref = query.as_deref();
        self.inner
            .request(reqwest::Method::PATCH, &url, Some(data), query_ref)
            .await
    }

    /// Delete a resource (`DELETE /path/{id}`).
    pub async fn delete(&self, id: u64, params: Option<&QueryParams>) -> Result<Value> {
        let url = format!("{}/{}", self.path, id);
        let query = Self::build_query(params);
        let query_ref = query.as_deref();
        self.inner
            .request(reqwest::Method::DELETE, &url, None, query_ref)
            .await
    }

    /// Bulk update (`PUT /path`).
    pub async fn bulk_update(&self, data: &Value, params: Option<&QueryParams>) -> Result<Value> {
        let query = Self::build_query(params);
        let query_ref = query.as_deref();
        self.inner
            .request(reqwest::Method::PUT, &self.path, Some(data), query_ref)
            .await
    }

    /// Bulk delete (`DELETE /path`).
    pub async fn bulk_delete(&self, params: Option<&QueryParams>) -> Result<Value> {
        let query = Self::build_query(params);
        let query_ref = query.as_deref();
        self.inner
            .request(reqwest::Method::DELETE, &self.path, None, query_ref)
            .await
    }

    /// Get resource metadata (`GET /path/meta`).
    pub async fn meta(&self, params: Option<&QueryParams>) -> Result<Value> {
        let url = format!("{}/meta", self.path);
        let query = Self::build_query(params);
        let query_ref = query.as_deref();
        self.inner
            .request(reqwest::Method::GET, &url, None, query_ref)
            .await
    }

    /// Autoselect (`GET /path/autoselect`).
    pub async fn autoselect(&self, params: Option<&QueryParams>) -> Result<Value> {
        let url = format!("{}/autoselect", self.path);
        let query = Self::build_query(params);
        let query_ref = query.as_deref();
        self.inner
            .request(reqwest::Method::GET, &url, None, query_ref)
            .await
    }

    /// Fetch all pages automatically.
    ///
    /// Expects responses in the format: `{"data": [...], "meta": {"total": N}}`.
    pub async fn all(&self, params: Option<QueryParams>) -> Result<Vec<Value>> {
        let mut params = params.unwrap_or_default();
        let limit = params.get_limit().unwrap_or(100);
        params = params.limit(limit);

        let mut all_items: Vec<Value> = Vec::new();
        let mut current_page = 1u64;

        loop {
            let page_params = params.clone().page(current_page);
            let response = self.list(Some(&page_params)).await?;

            let items = response
                .get("data")
                .and_then(|d| d.as_array())
                .cloned()
                .unwrap_or_default();

            if items.is_empty() {
                break;
            }

            all_items.extend(items);

            let total = response
                .get("meta")
                .and_then(|m| m.get("total"))
                .and_then(|t| t.as_u64())
                .unwrap_or(0);

            if all_items.len() as u64 >= total {
                break;
            }

            current_page += 1;
        }

        Ok(all_items)
    }
}
