//! Auto-generated module for `DocumentOfferAttribute`.
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
pub struct DocumentOfferAttribute {
    pub attribute_multis: serde_json::Value,
    pub display_priority: Option<i64>,
    pub filter_as_tags: Option<bool>,
    pub id: i64,
    pub is_active: Option<bool>,
    pub is_for_position: Option<bool>,
    pub is_for_position_attribute_position: Option<bool>,
    pub name: String,
    pub option: Option<String>,
    pub ordering: Option<i64>,
    pub searching_for_options: Option<bool>,
    #[serde(rename = "type")]
    pub r#type: i64,
    pub visible_in_list: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DocumentOfferAttributeListResponse {
    pub data: Vec<DocumentOfferAttribute>,
    pub meta: Option<super::ListMeta>,
}

/// Resource accessor for `/api/document_offer_attributes`.
pub struct DocumentOfferAttributeResource {
    resource: Resource,
}

impl DocumentOfferAttributeResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/document_offer_attributes"),
        }
    }

    /// List resources.
    pub async fn list(&self, params: Option<&QueryParams>) -> Result<DocumentOfferAttributeListResponse> {
        let value = self.resource.list(params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Retrieve a single resource by ID.
    pub async fn retrieve(&self, id: u64, params: Option<&QueryParams>) -> Result<DocumentOfferAttribute> {
        let value = self.resource.retrieve(id, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Create a new resource.
    pub async fn create(&self, data: &Value, params: Option<&QueryParams>) -> Result<DocumentOfferAttribute> {
        let value = self.resource.create(data, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Update a resource.
    pub async fn update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<DocumentOfferAttribute> {
        let value = self.resource.update(id, data, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Partial update a resource.
    pub async fn partial_update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<DocumentOfferAttribute> {
        let value = self.resource.partial_update(id, data, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Delete a resource.
    pub async fn delete(&self, id: u64, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.delete(id, params).await
    }

    /// Fetch all pages automatically.
    pub async fn all(&self, params: Option<QueryParams>) -> Result<Vec<DocumentOfferAttribute>> {
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
