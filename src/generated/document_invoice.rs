//! Auto-generated module for `DocumentInvoice`.
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
pub struct DocumentInvoice {
    pub address: Option<String>,
    pub address_customer: Option<String>,
    pub address_delivery: Option<String>,
    pub address_owner: Option<String>,
    pub company: serde_json::Value,
    pub company_name: Option<String>,
    pub connected_email_status: Option<i64>,
    pub created_at: Option<String>,
    pub creator_user: Option<serde_json::Value>,
    pub currency_code: String,
    pub default_address: Option<String>,
    pub delivery_place: Option<serde_json::Value>,
    pub description: Option<String>,
    pub document_date: String,
    pub document_header: String,
    pub document_order: Option<serde_json::Value>,
    pub document_series_definition: serde_json::Value,
    pub document_status: i64,
    pub document_subtype: Option<String>,
    pub document_type: i64,
    pub exchange: Option<String>,
    pub exchange_ratio: Option<f64>,
    pub external_number: Option<String>,
    pub id: i64,
    pub import_code: Option<String>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub income_value: Option<f64>,
    pub income_value_in_currency: Option<f64>,
    pub is_accounted: Option<bool>,
    pub is_receipt: Option<bool>,
    pub modification_date: Option<String>,
    pub nip: Option<String>,
    pub outcome_value: Option<f64>,
    pub outcome_value_in_currency: Option<f64>,
    pub payment_date: String,
    pub payment_payed: Option<f64>,
    pub payment_payed_in_currency: Option<f64>,
    pub payment_to_pay: f64,
    pub payment_to_pay_in_currency: Option<f64>,
    pub payment_type: serde_json::Value,
    pub positions: serde_json::Value,
    pub receipt: Option<serde_json::Value>,
    pub storehouse: Option<serde_json::Value>,
    pub to_document: Option<serde_json::Value>,
    pub user_ph: Option<serde_json::Value>,
    pub user_ph_name: Option<String>,
    pub value_brutto: f64,
    pub value_brutto_in_currency: Option<f64>,
    pub value_netto: f64,
    pub value_netto_in_currency: Option<f64>,
    pub value_vat: f64,
    pub value_vat_in_currency: Option<f64>,
    pub vat_conversion_mode: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DocumentInvoiceListResponse {
    pub data: Vec<DocumentInvoice>,
    pub meta: Option<super::ListMeta>,
}

/// Resource accessor for `/api/document_invoices`.
pub struct DocumentInvoiceResource {
    resource: Resource,
}

impl DocumentInvoiceResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/document_invoices"),
        }
    }

    /// List resources.
    pub async fn list(&self, params: Option<&QueryParams>) -> Result<DocumentInvoiceListResponse> {
        let value = self.resource.list(params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Retrieve a single resource by ID.
    pub async fn retrieve(&self, id: u64, params: Option<&QueryParams>) -> Result<DocumentInvoice> {
        let value = self.resource.retrieve(id, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Create a new resource.
    pub async fn create(&self, data: &Value, params: Option<&QueryParams>) -> Result<DocumentInvoice> {
        let value = self.resource.create(data, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Update a resource.
    pub async fn update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<DocumentInvoice> {
        let value = self.resource.update(id, data, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Partial update a resource.
    pub async fn partial_update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<DocumentInvoice> {
        let value = self.resource.partial_update(id, data, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Delete a resource.
    pub async fn delete(&self, id: u64, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.delete(id, params).await
    }

    /// Fetch all pages automatically.
    pub async fn all(&self, params: Option<QueryParams>) -> Result<Vec<DocumentInvoice>> {
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
