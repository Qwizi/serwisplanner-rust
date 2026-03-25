use std::sync::Arc;

use serde_json::Value;

use crate::client::ClientInner;
use crate::error::Result;
use crate::params::QueryParams;
use crate::resources::base::Resource;

/// Commissions resource (`/api/commissions`) with sub-resources.
pub struct CommissionsResource {
    pub resource: Resource,
}

impl CommissionsResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/commissions"),
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

    pub async fn meta(&self, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.meta(params).await
    }

    pub fn attributes(&self) -> CommissionAttributesResource {
        CommissionAttributesResource::new(self.resource.inner.clone())
    }

    pub fn phases(&self) -> Resource {
        Resource::new(self.resource.inner.clone(), "/api/commission_phases")
    }

    pub fn scope_types(&self) -> Resource {
        Resource::new(self.resource.inner.clone(), "/api/commission_scope_types")
    }

    pub fn shortcuts(&self) -> Resource {
        Resource::new(self.resource.inner.clone(), "/api/commission_shortcuts")
    }

    pub fn users(&self) -> Resource {
        Resource::new(self.resource.inner.clone(), "/api/commission_users")
    }
}

pub struct CommissionAttributesResource {
    pub resource: Resource,
}

impl CommissionAttributesResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/commission_attributes"),
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

    pub fn criterias(&self) -> Resource {
        Resource::new(
            self.resource.inner.clone(),
            "/api/commission_attribute_criterias",
        )
    }

    pub fn relations(&self) -> Resource {
        Resource::new(
            self.resource.inner.clone(),
            "/api/commission_attribute_relations",
        )
    }
}
