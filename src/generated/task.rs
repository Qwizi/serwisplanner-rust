//! Auto-generated module for `Task`.
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
pub struct Task {
    pub account_companies: Option<serde_json::Value>,
    pub account_users: Option<serde_json::Value>,
    pub begin_date: String,
    pub calendar_data: Option<String>,
    pub completion_date: Option<String>,
    pub component_type: Option<String>,
    pub created_at: Option<String>,
    pub creator_user: Option<serde_json::Value>,
    pub end_date: String,
    pub etag: Option<String>,
    pub event: Option<serde_json::Value>,
    pub first_task: Option<serde_json::Value>,
    pub id: i64,
    pub import_code: Option<String>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub is_completed: Option<bool>,
    pub modification_date: Option<String>,
    pub recurrence_active: bool,
    pub recurrence_ex_dates: Option<String>,
    pub recurrence_interval: Option<i64>,
    pub recurrence_parent_begin_date: Option<String>,
    pub recurrence_pattern: Option<i64>,
    pub recurrence_until: Option<String>,
    pub recurrence_uuid: Option<String>,
    pub remind_date: Option<String>,
    pub reminder_before: Option<i64>,
    pub reminder_read: Option<bool>,
    pub size: Option<i64>,
    pub task_description: Option<String>,
    pub task_name: Option<String>,
    #[serde(rename = "type")]
    pub r#type: serde_json::Value,
    pub type_id: Option<i64>,
    pub uri: Option<String>,
    pub user: Option<serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskListResponse {
    pub data: Vec<Task>,
    pub meta: Option<super::ListMeta>,
}

/// Resource accessor for `/api/tasks`.
pub struct TaskResource {
    resource: Resource,
}

impl TaskResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/tasks"),
        }
    }

    /// List resources.
    pub async fn list(&self, params: Option<&QueryParams>) -> Result<TaskListResponse> {
        let value = self.resource.list(params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Retrieve a single resource by ID.
    pub async fn retrieve(&self, id: u64, params: Option<&QueryParams>) -> Result<Task> {
        let value = self.resource.retrieve(id, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Create a new resource.
    pub async fn create(&self, data: &Value, params: Option<&QueryParams>) -> Result<Task> {
        let value = self.resource.create(data, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Update a resource.
    pub async fn update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<Task> {
        let value = self.resource.update(id, data, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Partial update a resource.
    pub async fn partial_update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<Task> {
        let value = self.resource.partial_update(id, data, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Delete a resource.
    pub async fn delete(&self, id: u64, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.delete(id, params).await
    }

    /// Fetch all pages automatically.
    pub async fn all(&self, params: Option<QueryParams>) -> Result<Vec<Task>> {
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
