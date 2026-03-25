use std::sync::Arc;

use reqwest::multipart;
use serde_json::{json, Value};

use crate::client::ClientInner;
use crate::error::Result;
use crate::params::QueryParams;
use crate::resources::base::Resource;

/// Files resource (`/api/files`) with upload support.
pub struct FilesResource {
    pub resource: Resource,
}

impl FilesResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/files"),
        }
    }

    pub async fn list(&self, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.list(params).await
    }

    pub async fn retrieve(&self, id: u64, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.retrieve(id, params).await
    }

    pub async fn create(&self, data: &Value, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.create(data, params).await
    }

    pub async fn update(
        &self,
        id: u64,
        data: &Value,
        params: Option<&QueryParams>,
    ) -> Result<Value> {
        self.resource.update(id, data, params).await
    }

    pub async fn delete(&self, id: u64, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.delete(id, params).await
    }

    pub async fn all(&self, params: Option<QueryParams>) -> Result<Vec<Value>> {
        self.resource.all(params).await
    }

    pub fn directories(&self) -> Resource {
        Resource::new(self.resource.inner.clone(), "/api/file_directories")
    }

    /// Upload a file via multipart form.
    ///
    /// # Arguments
    /// * `file_name` - Name of the file
    /// * `file_bytes` - File content as bytes
    /// * `mime_type` - MIME type (e.g., "application/pdf")
    /// * `mode` - Upload mode (default: 0)
    pub async fn upload(
        &self,
        file_name: &str,
        file_bytes: Vec<u8>,
        mime_type: &str,
        mode: Option<u32>,
    ) -> Result<Value> {
        let part = multipart::Part::bytes(file_bytes)
            .file_name(file_name.to_string())
            .mime_str(mime_type)
            .unwrap();

        let form = multipart::Form::new()
            .part("file", part)
            .text("mode", mode.unwrap_or(0).to_string());

        self.resource
            .inner
            .request_multipart("/api/files/upload", form)
            .await
    }

    /// Upload files from URLs.
    pub async fn upload_from_urls(&self, urls: &[&str]) -> Result<Value> {
        let body = json!(urls);
        self.resource
            .inner
            .request(
                reqwest::Method::POST,
                "/api/files/upload_from_urls",
                Some(&body),
                None,
            )
            .await
    }
}
