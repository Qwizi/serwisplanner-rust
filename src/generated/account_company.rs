//! Auto-generated module for `AccountCompany`.
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
pub struct AccountCompany {
    pub account_users: serde_json::Value,
    pub additional_phs: Option<serde_json::Value>,
    pub address_city: Option<String>,
    pub address_community: Option<String>,
    pub address_country: Option<String>,
    pub address_county: Option<String>,
    pub address_local_number: Option<String>,
    pub address_postal_code: Option<String>,
    pub address_province: Option<String>,
    pub address_street: Option<String>,
    pub address_street_number: Option<String>,
    pub block_order: Option<bool>,
    pub created_at: Option<String>,
    pub creation_type: Option<i64>,
    pub credit_limit_date: Option<String>,
    pub credit_limit_free: Option<f64>,
    pub default_delivery_type: Option<serde_json::Value>,
    pub default_language: Option<String>,
    pub default_order_series: Option<serde_json::Value>,
    pub default_payment_type: Option<serde_json::Value>,
    pub default_place: Option<serde_json::Value>,
    pub default_price_type: Option<i64>,
    pub default_user: Option<serde_json::Value>,
    pub id: i64,
    pub import_code: Option<String>,
    pub import_id: Option<String>,
    pub import_typ: Option<String>,
    pub modification_date: Option<String>,
    pub name: Option<String>,
    pub nip: Option<String>,
    pub original_credit_limit: Option<f64>,
    pub ph: Option<serde_json::Value>,
    pub places: serde_json::Value,
    pub price_company_group: Option<serde_json::Value>,
    pub short_name: Option<String>,
    pub temporary_credit_limit: Option<f64>,
    pub vat_handling: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccountCompanyListResponse {
    pub data: Vec<AccountCompany>,
    pub meta: Option<super::ListMeta>,
}

/// Resource accessor for `/api/account_companies`.
pub struct AccountCompanyResource {
    resource: Resource,
}

impl AccountCompanyResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/account_companies"),
        }
    }

    /// List resources.
    pub async fn list(&self, params: Option<&QueryParams>) -> Result<AccountCompanyListResponse> {
        let value = self.resource.list(params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Retrieve a single resource by ID.
    pub async fn retrieve(&self, id: u64, params: Option<&QueryParams>) -> Result<AccountCompany> {
        let value = self.resource.retrieve(id, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Create a new resource.
    pub async fn create(&self, data: &Value, params: Option<&QueryParams>) -> Result<AccountCompany> {
        let value = self.resource.create(data, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Update a resource.
    pub async fn update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<AccountCompany> {
        let value = self.resource.update(id, data, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Partial update a resource.
    pub async fn partial_update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<AccountCompany> {
        let value = self.resource.partial_update(id, data, params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Delete a resource.
    pub async fn delete(&self, id: u64, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.delete(id, params).await
    }

    /// Fetch all pages automatically.
    pub async fn all(&self, params: Option<QueryParams>) -> Result<Vec<AccountCompany>> {
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
