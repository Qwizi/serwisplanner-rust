//! Auto-generated module for `DocumentStore`.
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
pub struct DocumentStore {
    pub address_customer: Option<String>,
    pub address_delivery: Option<String>,
    pub address_owner: Option<String>,
    pub company: Option<serde_json::Value>,
    pub company_name: Option<String>,
    pub connected_document_store: Option<serde_json::Value>,
    pub created_at: Option<String>,
    pub creator_user: Option<serde_json::Value>,
    pub default_address: Option<String>,
    pub document_date_deadline: String,
    pub document_date_sold: String,
    pub document_description: Option<String>,
    pub document_header: Option<String>,
    pub document_inventory: Option<serde_json::Value>,
    pub document_invoice: Option<serde_json::Value>,
    pub document_order: Option<serde_json::Value>,
    pub document_payment_details: Option<String>,
    pub document_series_definition: serde_json::Value,
    pub document_status: Option<i64>,
    pub document_subtype: Option<String>,
    pub external_number: Option<String>,
    pub id: i64,
    pub import_code: Option<String>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub income_quantity: Option<f64>,
    pub is_accounted: Option<bool>,
    pub is_exported: Option<bool>,
    pub modification_date: Option<String>,
    pub nip: Option<String>,
    pub outcome_quantity: Option<f64>,
    pub positions: serde_json::Value,
    pub storehouse: serde_json::Value,
    pub storehouse_code: Option<String>,
    pub to_document: Option<serde_json::Value>,
    pub to_storehouse: Option<serde_json::Value>,
    pub value_netto: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DocumentStoreListResponse {
    pub data: Vec<DocumentStore>,
    pub meta: Option<super::ListMeta>,
}

/// Resource accessor for `/api/document_stores`.
pub struct DocumentStoreResource {
    resource: Resource,
}

impl DocumentStoreResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/document_stores"),
        }
    }

    /// List resources.
    pub async fn list(&self, params: Option<&QueryParams>) -> Result<DocumentStoreListResponse> {
        let value = self.resource.list(params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Retrieve a single resource by ID.
    pub async fn retrieve(&self, id: u64, params: Option<&QueryParams>) -> Result<DocumentStore> {
        let value = self.resource.retrieve(id, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Create a new resource.
    pub async fn create(&self, data: &Value, params: Option<&QueryParams>) -> Result<DocumentStore> {
        let value = self.resource.create(data, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Update a resource.
    pub async fn update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<DocumentStore> {
        let value = self.resource.update(id, data, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Partial update a resource.
    pub async fn partial_update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<DocumentStore> {
        let value = self.resource.partial_update(id, data, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Delete a resource.
    pub async fn delete(&self, id: u64, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.delete(id, params).await
    }

    /// Fetch all pages automatically.
    pub async fn all(&self, params: Option<QueryParams>) -> Result<Vec<DocumentStore>> {
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
