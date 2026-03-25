//! Auto-generated module for `CommissionAttribute`.
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
pub struct CommissionAttribute {
    pub additional_options: Option<String>,
    pub attribute_multis: Option<CommissionAttributeMultiRel>,
    pub chatbox: Option<bool>,
    pub display_priority: Option<i64>,
    pub document_series_definition: Option<String>,
    pub file: Option<serde_json::Value>,
    pub filter_as_tags: Option<bool>,
    pub id: Option<i64>,
    pub is_active: Option<bool>,
    pub is_for_position: Option<bool>,
    pub name: Option<String>,
    pub option: Option<String>,
    pub ordering: Option<i64>,
    pub parent: Option<CommissionAttributeRel>,
    pub register_event_onchange: Option<bool>,
    pub searching_for_options: Option<bool>,
    pub task_type: Option<TaskTypeRel>,
    pub tooltip: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<i64>,
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CommissionAttributeMultiRel {
    pub additional_options: Option<String>,
    pub attribute: Option<serde_json::Value>,
    pub attribute_option: Option<String>,
    pub id: Option<i64>,
    pub is_active: Option<bool>,
    pub name: Option<String>,
    pub ordering: Option<i64>,
    #[serde(rename = "type")]
    pub r#type: Option<i64>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CommissionAttributeRel {
    pub additional_options: Option<String>,
    pub attribute_multis: Option<serde_json::Value>,
    pub chatbox: Option<bool>,
    pub display_priority: Option<i64>,
    pub document_series_definition: Option<String>,
    pub file: Option<serde_json::Value>,
    pub filter_as_tags: Option<bool>,
    pub id: Option<i64>,
    pub is_active: Option<bool>,
    pub is_for_position: Option<bool>,
    pub name: Option<String>,
    pub option: Option<String>,
    pub ordering: Option<i64>,
    pub parent: Option<serde_json::Value>,
    pub register_event_onchange: Option<bool>,
    pub searching_for_options: Option<bool>,
    pub task_type: Option<serde_json::Value>,
    pub tooltip: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<i64>,
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct TaskTypeRel {
    pub default_event_date: Option<i64>,
    pub default_planned_date: Option<i64>,
    pub icon: Option<String>,
    pub icon_color: Option<String>,
    pub id: Option<i64>,
    pub is_automatic: Option<bool>,
    pub is_event: Option<bool>,
    pub is_hidden: Option<bool>,
    pub is_private: Option<bool>,
    pub is_time_extended: Option<bool>,
    pub notify_user: Option<bool>,
    pub required_fields: Option<String>,
    pub task_priority: Option<i64>,
    pub type_name: Option<String>,
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CommissionAttributeListResponse {
    pub data: Vec<CommissionAttribute>,
    pub meta: Option<super::ListMeta>,
}

/// Resource accessor for `/api/commission_attributes`.
pub struct CommissionAttributeResource {
    resource: Resource,
}

impl CommissionAttributeResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/commission_attributes"),
        }
    }

    /// List resources.
    pub async fn list(&self, params: Option<&QueryParams>) -> Result<CommissionAttributeListResponse> {
        let value = self.resource.list(params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Retrieve a single resource by ID.
    pub async fn retrieve(&self, id: u64, params: Option<&QueryParams>) -> Result<CommissionAttribute> {
        let value = self.resource.retrieve(id, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Create a new resource.
    pub async fn create(&self, data: &Value, params: Option<&QueryParams>) -> Result<CommissionAttribute> {
        let value = self.resource.create(data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Update a resource.
    pub async fn update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<CommissionAttribute> {
        let value = self.resource.update(id, data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Partial update a resource.
    pub async fn partial_update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<CommissionAttribute> {
        let value = self.resource.partial_update(id, data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Delete a resource.
    pub async fn delete(&self, id: u64, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.delete(id, params).await
    }

    /// Fetch all pages automatically.
    pub async fn all(&self, params: Option<QueryParams>) -> Result<Vec<CommissionAttribute>> {
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
