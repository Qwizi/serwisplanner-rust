use std::sync::Arc;

use serde_json::Value;

use crate::client::ClientInner;
use crate::error::Result;
use crate::params::QueryParams;
use crate::resources::base::Resource;

/// Serviced products resource (`/api/serviced_products`).
pub struct ServicedProductsResource {
    pub resource: Resource,
}

impl ServicedProductsResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/serviced_products"),
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

    pub async fn partial_update(
        &self,
        id: u64,
        data: &Value,
        params: Option<&QueryParams>,
    ) -> Result<Value> {
        self.resource.partial_update(id, data, params).await
    }

    pub async fn delete(&self, id: u64, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.delete(id, params).await
    }

    pub async fn all(&self, params: Option<QueryParams>) -> Result<Vec<Value>> {
        self.resource.all(params).await
    }

    pub fn attributes(&self) -> Resource {
        Resource::new(
            self.resource.inner.clone(),
            "/api/serviced_product_attributes",
        )
    }

    /// Generate PDF (`GET /api/serviced_products/{id}/pdf`).
    pub async fn generate_pdf(
        &self,
        product_id: u64,
        template_id: Option<u64>,
    ) -> Result<Value> {
        let url = format!("/api/serviced_products/{}/pdf", product_id);
        let tid = template_id.unwrap_or(0).to_string();
        let query = vec![("template_id".to_string(), tid)];
        self.resource
            .inner
            .request(reqwest::Method::GET, &url, None, Some(&query))
            .await
    }
}
