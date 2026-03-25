//! Auto-generated module for `EmailAccount`.
//!
//! Do not edit manually. Regenerate with: `python3 codegen.py <yaml>`

use std::sync::Arc;
use serde_json::Value;
use crate::client::ClientInner;
use crate::error::Result;
use crate::params::QueryParams;
use crate::resources::base::Resource;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailAccount {
    pub append_to_send: Option<bool>,
    pub archival: Option<bool>,
    pub auth_method: Option<String>,
    pub auth_type: String,
    pub config_modification_date: Option<String>,
    pub email_account_name: Option<String>,
    pub email_address: String,
    pub full_synch_now: Option<bool>,
    pub id: i64,
    pub imap_login: Option<String>,
    pub imap_port: Option<i64>,
    pub imap_secure: Option<String>,
    pub imap_server: Option<String>,
    pub is_system_account: bool,
    pub last_inc_sync_date: Option<String>,
    pub sent_folder_name: Option<String>,
    pub smtp_auth_method: Option<String>,
    pub smtp_login: Option<String>,
    pub smtp_port: Option<i64>,
    pub smtp_secure: Option<String>,
    pub smtp_sender: Option<String>,
    pub smtp_server: Option<String>,
    pub unread_messages_counter: Option<i64>,
    pub user: Option<serde_json::Value>,
    pub validate_cert: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmailAccountListResponse {
    pub data: Vec<EmailAccount>,
    pub meta: Option<super::ListMeta>,
}

/// Resource accessor for `/api/email_accounts`.
pub struct EmailAccountResource {
    resource: Resource,
}

impl EmailAccountResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/email_accounts"),
        }
    }

    /// List resources.
    pub async fn list(&self, params: Option<&QueryParams>) -> Result<EmailAccountListResponse> {
        let value = self.resource.list(params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Retrieve a single resource by ID.
    pub async fn retrieve(&self, id: u64, params: Option<&QueryParams>) -> Result<EmailAccount> {
        let value = self.resource.retrieve(id, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Create a new resource.
    pub async fn create(&self, data: &Value, params: Option<&QueryParams>) -> Result<EmailAccount> {
        let value = self.resource.create(data, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Update a resource.
    pub async fn update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<EmailAccount> {
        let value = self.resource.update(id, data, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Partial update a resource.
    pub async fn partial_update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<EmailAccount> {
        let value = self.resource.partial_update(id, data, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Delete a resource.
    pub async fn delete(&self, id: u64, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.delete(id, params).await
    }

    /// Fetch all pages automatically.
    pub async fn all(&self, params: Option<QueryParams>) -> Result<Vec<EmailAccount>> {
        let items = self.resource.all(params).await?;
        items.into_iter()
            .map(|v| serde_json::from_value(v).map_err(|e| crate::error::SWError::Other(e.to_string())))
            .collect()
    }

    /// Get resource metadata.
    pub async fn meta(&self, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.meta(params).await
    }

    /// Autoselect.
    pub async fn autoselect(&self, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.autoselect(params).await
    }

    /// Access the untyped base resource.
    pub fn raw(&self) -> &Resource {
        &self.resource
    }
}
