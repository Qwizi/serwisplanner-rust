//! Auto-generated module for `Commission`.
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
pub struct Commission {
    pub account_user: Option<AccountUserRel>,
    pub account_user_name: Option<String>,
    pub commission_address: Option<String>,
    pub commission_number: Option<String>,
    pub commission_phase: Option<CommissionPhaseRel>,
    pub commission_phase_id: Option<i64>,
    pub commissions_user_users: Option<UserUserRel>,
    pub company: Option<AccountCompanyRel>,
    pub company_name: Option<String>,
    pub created_at: Option<String>,
    pub creator_account_user: Option<AccountUserRel>,
    pub creator_user: Option<UserUserRel>,
    pub document_series_definition: Option<DocumentSeriesDefinitionRel>,
    pub end_date: Option<String>,
    pub id: Option<i64>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub modification_date: Option<String>,
    pub name: Option<String>,
    pub phase_modification_date: Option<String>,
    pub place: Option<PlacesRel>,
    pub positions: Option<CommissionPositionRel>,
    pub start_date: Option<String>,
    pub user: Option<UserUserRel>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AccountCompanyRel {
    pub account_users: Option<serde_json::Value>,
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
    pub id: Option<i64>,
    pub import_code: Option<String>,
    pub import_id: Option<String>,
    pub import_typ: Option<String>,
    pub modification_date: Option<String>,
    pub name: Option<String>,
    pub nip: Option<String>,
    pub original_credit_limit: Option<f64>,
    pub ph: Option<serde_json::Value>,
    pub places: Option<serde_json::Value>,
    pub price_company_group: Option<serde_json::Value>,
    pub short_name: Option<String>,
    pub temporary_credit_limit: Option<f64>,
    pub vat_handling: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AccountUserRel {
    pub active_currency: Option<String>,
    pub additional_contacts: Option<serde_json::Value>,
    pub authorized_to_make_orders: Option<bool>,
    pub available_currencies: Option<String>,
    pub avatar: Option<String>,
    pub avatar_file: Option<serde_json::Value>,
    pub company: Option<serde_json::Value>,
    pub consent_processing_personal_data: Option<bool>,
    pub default_currency: Option<String>,
    pub default_language: Option<String>,
    pub email: Option<String>,
    pub failed_login_counter: Option<i64>,
    pub first_name: Option<String>,
    pub id: Option<i64>,
    pub import_code: Option<String>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub is2fa_active: Option<bool>,
    pub is_invoice_user: Option<bool>,
    pub language: Option<String>,
    pub last_failed_login: Option<String>,
    pub last_failed_login_ip: Option<String>,
    pub last_login: Option<String>,
    pub last_login_ip: Option<String>,
    pub last_name: Option<String>,
    pub method2fa: Option<String>,
    pub mobile_number: Option<String>,
    pub modification_date: Option<String>,
    pub newsletter_active: Option<bool>,
    pub phone_number: Option<String>,
    pub place: Option<serde_json::Value>,
    pub scoring: Option<i64>,
    pub system_theme: Option<String>,
    pub user_profile: Option<serde_json::Value>,
    pub username: Option<String>,
    pub verified: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CommissionPhaseRel {
    pub color: Option<String>,
    pub commission_phase_id: Option<i64>,
    pub document_series_definition: Option<String>,
    pub is_first_phase: Option<bool>,
    pub kanban_columns: Option<serde_json::Value>,
    pub phase_name: Option<String>,
    pub product_required: Option<i64>,
    pub serial_code_required: Option<i64>,
    pub stage: Option<i64>,
    pub task_type: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CommissionPositionRel {
    pub commission: Option<serde_json::Value>,
    pub id: Option<i64>,
    pub price: Option<f64>,
    pub product: Option<serde_json::Value>,
    pub product_code: Option<String>,
    pub product_id: Option<i64>,
    pub product_name: Option<String>,
    pub quantity: Option<f64>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DocumentSeriesDefinitionRel {
    pub account: Option<i64>,
    pub additional_data: Option<String>,
    pub available_templates: Option<String>,
    pub create_connected_document_store: Option<i64>,
    pub currency: Option<String>,
    pub default_series: Option<bool>,
    pub document_subtype: Option<String>,
    pub document_type: Option<i64>,
    pub id: Option<i64>,
    pub name: Option<String>,
    pub numbering_schema: Option<String>,
    pub numbering_type: Option<i64>,
    pub payment_register: Option<serde_json::Value>,
    pub positions_active: Option<bool>,
    pub priority: Option<i64>,
    pub product_price_option: Option<i64>,
    pub serial_code_option: Option<i64>,
    pub stages: Option<i64>,
    pub storehouse: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct PlacesRel {
    pub city: Option<String>,
    pub community: Option<String>,
    pub company: Option<serde_json::Value>,
    pub coordinates_modification_date: Option<String>,
    pub country: Option<String>,
    pub county: Option<String>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub id: Option<i64>,
    pub import_code: Option<String>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub latitude: Option<f64>,
    pub local_number: Option<String>,
    pub longitude: Option<f64>,
    pub main_address: Option<bool>,
    pub modification_date: Option<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub postal_code: Option<String>,
    pub province: Option<String>,
    pub street_address: Option<String>,
    pub street_number: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct UserUserRel {
    pub active_currency: Option<String>,
    pub available_currencies: Option<String>,
    pub avatar: Option<String>,
    pub avatar_file: Option<serde_json::Value>,
    pub default_callendar_color: Option<String>,
    pub default_css_file: Option<serde_json::Value>,
    pub default_currency: Option<String>,
    pub email: Option<String>,
    pub failed_login_counter: Option<i64>,
    pub first_name: Option<String>,
    pub id: Option<i64>,
    pub import_code: Option<String>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub is2fa_active: Option<bool>,
    pub is_external_login: Option<bool>,
    pub language: Option<String>,
    pub last_action: Option<String>,
    pub last_failed_login: Option<String>,
    pub last_login_date: Option<String>,
    pub last_logout_date: Option<String>,
    pub last_name: Option<String>,
    pub logged_in: Option<bool>,
    pub message_on_email: Option<bool>,
    pub method2fa: Option<String>,
    pub mobile_number: Option<String>,
    pub modification_date: Option<String>,
    pub msexchange: Option<bool>,
    pub phone_number: Option<String>,
    pub reset_link_expires: Option<String>,
    pub session_id: Option<String>,
    pub subordinates: Option<serde_json::Value>,
    pub superior: Option<serde_json::Value>,
    pub system_theme: Option<String>,
    pub uri: Option<String>,
    pub user_profile: Option<serde_json::Value>,
    pub username: Option<String>,
    pub verified: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CommissionListResponse {
    pub data: Vec<Commission>,
    pub meta: Option<super::ListMeta>,
}

/// Resource accessor for `/api/commissions`.
pub struct CommissionResource {
    resource: Resource,
}

impl CommissionResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/commissions"),
        }
    }

    /// List resources.
    pub async fn list(&self, params: Option<&QueryParams>) -> Result<CommissionListResponse> {
        let value = self.resource.list(params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Retrieve a single resource by ID.
    pub async fn retrieve(&self, id: u64, params: Option<&QueryParams>) -> Result<Commission> {
        let value = self.resource.retrieve(id, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Create a new resource.
    pub async fn create(&self, data: &Value, params: Option<&QueryParams>) -> Result<Commission> {
        let value = self.resource.create(data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Update a resource.
    pub async fn update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<Commission> {
        let value = self.resource.update(id, data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Partial update a resource.
    pub async fn partial_update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<Commission> {
        let value = self.resource.partial_update(id, data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Delete a resource.
    pub async fn delete(&self, id: u64, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.delete(id, params).await
    }

    /// Fetch all pages automatically.
    pub async fn all(&self, params: Option<QueryParams>) -> Result<Vec<Commission>> {
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
