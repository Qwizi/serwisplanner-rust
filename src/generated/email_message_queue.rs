//! Auto-generated module for `EmailMessageQueue`.
//!
//! Do not edit manually. Regenerate with: `python3 codegen.py <yaml>`

use std::sync::Arc;
use serde_json::Value;
use crate::client::ClientInner;
use crate::error::Result;
use crate::params::QueryParams;
use crate::resources::base::Resource;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct EmailMessageQueue {
    pub content_html: Option<String>,
    pub created_at: Option<String>,
    pub email_account: Option<EmailAccountRel>,
    pub email_message: Option<EmailMessageRel>,
    pub files: Option<String>,
    pub headers: Option<String>,
    pub id: Option<i64>,
    pub send_attempts: Option<i64>,
    pub smtp_sent: Option<bool>,
    pub subject: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct EmailAccountRel {
    pub append_to_send: Option<bool>,
    pub archival: Option<bool>,
    pub auth_method: Option<String>,
    pub auth_type: Option<String>,
    pub config_modification_date: Option<String>,
    pub email_account_name: Option<String>,
    pub email_address: Option<String>,
    pub full_synch_now: Option<bool>,
    pub id: Option<i64>,
    pub imap_login: Option<String>,
    pub imap_port: Option<i64>,
    pub imap_secure: Option<String>,
    pub imap_server: Option<String>,
    pub is_system_account: Option<bool>,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct EmailMessageRel {
    pub attachments: Option<String>,
    pub content: Option<String>,
    pub content_html: Option<String>,
    pub delivery_date: Option<String>,
    pub email_account: Option<serde_json::Value>,
    pub has_attachments: Option<bool>,
    pub has_been_read: Option<bool>,
    pub headers: Option<String>,
    pub id: Option<i64>,
    pub imap_folder: Option<String>,
    pub imap_id: Option<i64>,
    pub received_date: Option<String>,
    pub send_date: Option<String>,
    pub server_message_id: Option<String>,
    pub subject: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EmailMessageQueueListResponse {
    pub data: Vec<EmailMessageQueue>,
    pub meta: Option<super::ListMeta>,
}

/// Resource accessor for `/api/email_message_queues`.
pub struct EmailMessageQueueResource {
    resource: Resource,
}

impl EmailMessageQueueResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/email_message_queues"),
        }
    }

    /// List resources.
    pub async fn list(&self, params: Option<&QueryParams>) -> Result<EmailMessageQueueListResponse> {
        let value = self.resource.list(params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Retrieve a single resource by ID.
    pub async fn retrieve(&self, id: u64, params: Option<&QueryParams>) -> Result<EmailMessageQueue> {
        let value = self.resource.retrieve(id, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Create a new resource.
    pub async fn create(&self, data: &Value, params: Option<&QueryParams>) -> Result<EmailMessageQueue> {
        let value = self.resource.create(data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Update a resource.
    pub async fn update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<EmailMessageQueue> {
        let value = self.resource.update(id, data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Partial update a resource.
    pub async fn partial_update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<EmailMessageQueue> {
        let value = self.resource.partial_update(id, data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Delete a resource.
    pub async fn delete(&self, id: u64, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.delete(id, params).await
    }

    /// Fetch all pages automatically.
    pub async fn all(&self, params: Option<QueryParams>) -> Result<Vec<EmailMessageQueue>> {
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
