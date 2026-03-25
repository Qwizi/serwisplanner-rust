use std::sync::Arc;

use serde_json::Value;

use crate::client::ClientInner;
use crate::error::Result;
use crate::params::QueryParams;
use crate::resources::base::Resource;

/// Places resource (`/api/places`) with attributes sub-resource.
pub struct PlacesResource {
    pub resource: Resource,
}

impl PlacesResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/places"),
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

    pub fn attributes(&self) -> Resource {
        Resource::new(self.resource.inner.clone(), "/api/place_attributes")
    }
}
