use std::sync::Arc;

use serde_json::Value;

use crate::client::ClientInner;
use crate::error::Result;
use crate::params::QueryParams;
use crate::resources::base::Resource;

/// Users resource (`/api/user_users`).
pub struct UsersResource {
    pub resource: Resource,
}

impl UsersResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/user_users"),
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
        Resource::new(self.resource.inner.clone(), "/api/user_attributes")
    }

    pub fn histories(&self) -> UserHistoriesResource {
        UserHistoriesResource::new(self.resource.inner.clone())
    }
}

pub struct UserHistoriesResource {
    inner: Arc<ClientInner>,
}

impl UserHistoriesResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    pub async fn list(&self, user_id: u64, params: Option<&QueryParams>) -> Result<Value> {
        let url = format!("/api/user_users/{}/histories", user_id);
        let query: Option<Vec<(String, String)>> =
            params.map(|p| p.build().into_iter().collect());
        self.inner
            .request(reqwest::Method::GET, &url, None, query.as_deref())
            .await
    }
}

/// User profiles resource (`/api/user_profiles`).
pub struct UserProfilesResource {
    pub resource: Resource,
}

impl UserProfilesResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/user_profiles"),
        }
    }
}

impl std::ops::Deref for UserProfilesResource {
    type Target = Resource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}
