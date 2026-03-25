use std::sync::Arc;

use serde_json::Value;

use crate::client::ClientInner;
use crate::error::Result;
use crate::params::QueryParams;
use crate::resources::base::Resource;

/// Container resource for account-related endpoints.
pub struct AccountResource {
    inner: Arc<ClientInner>,
}

impl AccountResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    pub fn companies(&self) -> AccountCompaniesResource {
        AccountCompaniesResource::new(self.inner.clone())
    }

    pub fn users(&self) -> AccountUsersResource {
        AccountUsersResource::new(self.inner.clone())
    }
}

// --- Account Companies ---

pub struct AccountCompaniesResource {
    pub resource: Resource,
}

impl AccountCompaniesResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/account_companies"),
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

    pub async fn autoselect(&self, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.autoselect(params).await
    }

    pub fn attributes(&self) -> Resource {
        Resource::new(
            self.resource.inner.clone(),
            "/api/account_company_attributes",
        )
    }

    pub fn histories(&self) -> AccountCompanyHistoriesResource {
        AccountCompanyHistoriesResource::new(self.resource.inner.clone())
    }

    /// GUS update (`PUT /api/account_companies/{id}/gus`).
    pub async fn gus_update(&self, company_id: u64, data: &Value) -> Result<Value> {
        let url = format!("/api/account_companies/{}/gus", company_id);
        self.resource
            .inner
            .request(reqwest::Method::PUT, &url, Some(data), None)
            .await
    }

    /// ODBC reports (`GET /api/account_companies/{id}/odbc_reports`).
    pub async fn odbc_reports(
        &self,
        company_id: u64,
        params: Option<&QueryParams>,
    ) -> Result<Value> {
        let url = format!("/api/account_companies/{}/odbc_reports", company_id);
        let query: Option<Vec<(String, String)>> =
            params.map(|p| p.build().into_iter().collect());
        self.resource
            .inner
            .request(reqwest::Method::GET, &url, None, query.as_deref())
            .await
    }

    /// Email messages (`GET /api/account_companies/{id}/email_messages`).
    pub async fn email_messages(
        &self,
        company_id: u64,
        params: Option<&QueryParams>,
    ) -> Result<Value> {
        let url = format!("/api/account_companies/{}/email_messages", company_id);
        let query: Option<Vec<(String, String)>> =
            params.map(|p| p.build().into_iter().collect());
        self.resource
            .inner
            .request(reqwest::Method::GET, &url, None, query.as_deref())
            .await
    }
}

pub struct AccountCompanyHistoriesResource {
    inner: Arc<ClientInner>,
}

impl AccountCompanyHistoriesResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    pub async fn list(
        &self,
        company_id: u64,
        params: Option<&QueryParams>,
    ) -> Result<Value> {
        let url = format!("/api/account_companies/{}/histories", company_id);
        let query: Option<Vec<(String, String)>> =
            params.map(|p| p.build().into_iter().collect());
        self.inner
            .request(reqwest::Method::GET, &url, None, query.as_deref())
            .await
    }
}

// --- Account Users ---

pub struct AccountUsersResource {
    pub resource: Resource,
}

impl AccountUsersResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/account_users"),
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
            "/api/account_user_attributes",
        )
    }

    pub fn histories(&self) -> AccountUserHistoriesResource {
        AccountUserHistoriesResource::new(self.resource.inner.clone())
    }
}

pub struct AccountUserHistoriesResource {
    inner: Arc<ClientInner>,
}

impl AccountUserHistoriesResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    pub async fn list(
        &self,
        user_id: u64,
        params: Option<&QueryParams>,
    ) -> Result<Value> {
        let url = format!("/api/account_users/{}/histories", user_id);
        let query: Option<Vec<(String, String)>> =
            params.map(|p| p.build().into_iter().collect());
        self.inner
            .request(reqwest::Method::GET, &url, None, query.as_deref())
            .await
    }
}
