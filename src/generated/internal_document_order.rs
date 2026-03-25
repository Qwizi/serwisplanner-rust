//! Auto-generated module for `InternalDocumentOrder`.
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
pub struct InternalDocumentOrder {
    pub address_customer: Option<String>,
    pub address_delivery: Option<String>,
    pub address_owner: Option<String>,
    pub company_id: Option<i64>,
    pub company_name: Option<String>,
    pub currency_code: Option<String>,
    pub date_delivery: Option<String>,
    pub default_address: Option<String>,
    pub delivery_type: Option<serde_json::Value>,
    pub document_header: Option<String>,
    pub document_id: i64,
    pub document_series_definition: Option<serde_json::Value>,
    pub document_status: Option<i64>,
    pub document_status_by_ph: Option<i64>,
    pub document_value_brutto: Option<f64>,
    pub document_value_brutto_in_currency: Option<f64>,
    pub document_value_netto: Option<f64>,
    pub document_value_netto_in_currency: Option<f64>,
    pub document_value_rabat: Option<f64>,
    pub document_value_rabat_in_currency: Option<f64>,
    pub exchange_history: Option<serde_json::Value>,
    pub external_date: Option<String>,
    pub external_number: Option<String>,
    pub import_code: Option<String>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub manual_status_changed: Option<bool>,
    pub name_customer: Option<String>,
    pub name_delivery: Option<String>,
    pub name_owner: Option<String>,
    pub nip: Option<String>,
    pub notes: Option<String>,
    pub order_time: Option<String>,
    pub order_type: Option<i64>,
    pub payment_type: Option<serde_json::Value>,
    pub place: Option<serde_json::Value>,
    pub storehouse: Option<serde_json::Value>,
    pub user: Option<serde_json::Value>,
    pub user_ph_id: Option<i64>,
    pub user_ph_name: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InternalDocumentOrderListResponse {
    pub data: Vec<InternalDocumentOrder>,
    pub meta: Option<super::ListMeta>,
}

/// Resource accessor for `/api/internal_document_orders`.
pub struct InternalDocumentOrderResource {
    resource: Resource,
}

impl InternalDocumentOrderResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/internal_document_orders"),
        }
    }

    /// List resources.
    pub async fn list(&self, params: Option<&QueryParams>) -> Result<InternalDocumentOrderListResponse> {
        let value = self.resource.list(params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Retrieve a single resource by ID.
    pub async fn retrieve(&self, id: u64, params: Option<&QueryParams>) -> Result<InternalDocumentOrder> {
        let value = self.resource.retrieve(id, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Create a new resource.
    pub async fn create(&self, data: &Value, params: Option<&QueryParams>) -> Result<InternalDocumentOrder> {
        let value = self.resource.create(data, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Update a resource.
    pub async fn update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<InternalDocumentOrder> {
        let value = self.resource.update(id, data, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Partial update a resource.
    pub async fn partial_update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<InternalDocumentOrder> {
        let value = self.resource.partial_update(id, data, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Delete a resource.
    pub async fn delete(&self, id: u64, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.delete(id, params).await
    }

    /// Fetch all pages automatically.
    pub async fn all(&self, params: Option<QueryParams>) -> Result<Vec<InternalDocumentOrder>> {
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
