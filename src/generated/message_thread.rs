//! Auto-generated module for `MessageThread`.
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
pub struct MessageThread {
    pub id: Option<i64>,
    pub is_archival: Option<bool>,
    pub last_message: Option<MessageRel>,
    pub last_message_content: Option<String>,
    pub last_message_date: Option<String>,
    pub last_sender_avatar: Option<String>,
    pub last_sender_name: Option<String>,
    pub messages: Option<MessageRel>,
    pub user_count: Option<i64>,
    pub users: Option<MessageThreadUserRel>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct MessageRel {
    pub content: Option<String>,
    pub date_sent: Option<String>,
    pub id: Option<i64>,
    pub is_system_message: Option<bool>,
    pub sender_account_user: Option<serde_json::Value>,
    pub sender_name: Option<String>,
    pub sender_user: Option<serde_json::Value>,
    pub thread: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct MessageThreadUserRel {
    pub account_user: Option<serde_json::Value>,
    pub date_added: Option<String>,
    pub id: Option<i64>,
    pub is_active: Option<bool>,
    pub last_read_message: Option<serde_json::Value>,
    pub message_thread: Option<serde_json::Value>,
    pub user_type: Option<i64>,
    pub user_user: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MessageThreadListResponse {
    pub data: Vec<MessageThread>,
    pub meta: Option<super::ListMeta>,
}

/// Resource accessor for `/api/message_threads`.
pub struct MessageThreadResource {
    resource: Resource,
}

impl MessageThreadResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/message_threads"),
        }
    }

    /// List resources.
    pub async fn list(&self, params: Option<&QueryParams>) -> Result<MessageThreadListResponse> {
        let value = self.resource.list(params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Retrieve a single resource by ID.
    pub async fn retrieve(&self, id: u64, params: Option<&QueryParams>) -> Result<MessageThread> {
        let value = self.resource.retrieve(id, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Create a new resource.
    pub async fn create(&self, data: &Value, params: Option<&QueryParams>) -> Result<MessageThread> {
        let value = self.resource.create(data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Update a resource.
    pub async fn update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<MessageThread> {
        let value = self.resource.update(id, data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Partial update a resource.
    pub async fn partial_update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<MessageThread> {
        let value = self.resource.partial_update(id, data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Delete a resource.
    pub async fn delete(&self, id: u64, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.delete(id, params).await
    }

    /// Fetch all pages automatically.
    pub async fn all(&self, params: Option<QueryParams>) -> Result<Vec<MessageThread>> {
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
