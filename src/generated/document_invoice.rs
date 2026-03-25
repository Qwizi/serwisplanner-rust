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
    pub company: Option<AccountCompanyRel>,
    pub company_name: Option<String>,
    pub connected_email_status: Option<i64>,
    pub created_at: Option<String>,
    pub creator_user: Option<UserUserRel>,
    pub currency_code: Option<String>,
    pub default_address: Option<String>,
    pub delivery_place: Option<PlacesRel>,
    pub description: Option<String>,
    pub document_date: Option<String>,
    pub document_header: Option<String>,
    pub document_order: Option<DocumentOrderRel>,
    pub document_series_definition: Option<DocumentSeriesDefinitionRel>,
    pub document_status: Option<i64>,
    pub document_subtype: Option<String>,
    pub document_type: Option<i64>,
    pub exchange: Option<String>,
    pub exchange_ratio: Option<f64>,
    pub external_number: Option<String>,
    pub id: Option<i64>,
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
    pub payment_date: Option<String>,
    pub payment_payed: Option<f64>,
    pub payment_payed_in_currency: Option<f64>,
    pub payment_to_pay: Option<f64>,
    pub payment_to_pay_in_currency: Option<f64>,
    pub payment_type: Option<PaymentTypeRel>,
    pub positions: Option<DocumentInvoicePositionRel>,
    pub receipt: Option<DocumentInvoiceRel>,
    pub storehouse: Option<StorehouseRel>,
    pub to_document: Option<DocumentInvoiceRel>,
    pub user_ph: Option<UserUserRel>,
    pub user_ph_name: Option<String>,
    pub value_brutto: Option<f64>,
    pub value_brutto_in_currency: Option<f64>,
    pub value_netto: Option<f64>,
    pub value_netto_in_currency: Option<f64>,
    pub value_vat: Option<f64>,
    pub value_vat_in_currency: Option<f64>,
    pub vat_conversion_mode: Option<i64>,
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
pub struct DocumentInvoicePositionRel {
    pub base_price_brutto: Option<f64>,
    pub base_price_netto: Option<f64>,
    pub base_rabat: Option<f64>,
    pub base_value_brutto: Option<f64>,
    pub base_value_netto: Option<f64>,
    pub corrected_position: Option<serde_json::Value>,
    pub current_base_price_netto: Option<f64>,
    pub current_base_rabat: Option<f64>,
    pub current_price_netto: Option<f64>,
    pub current_price_rabat: Option<f64>,
    pub current_quantity: Option<f64>,
    pub current_rabat: Option<f64>,
    pub current_vat: Option<serde_json::Value>,
    pub document_invoice: Option<serde_json::Value>,
    pub id: Option<i64>,
    pub import_code: Option<String>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub is_complet: Option<bool>,
    pub measure_unit: Option<serde_json::Value>,
    pub measure_unit_name: Option<String>,
    pub modification_date: Option<String>,
    pub order_position: Option<serde_json::Value>,
    pub ordering: Option<i64>,
    pub price_brutto: Option<f64>,
    pub price_netto: Option<f64>,
    pub price_rabat: Option<f64>,
    pub price_rabat_brutto: Option<f64>,
    pub product: Option<serde_json::Value>,
    pub product_code: Option<String>,
    pub product_name: Option<String>,
    pub quantity: Option<f64>,
    pub rabat: Option<f64>,
    pub value_brutto: Option<f64>,
    pub value_brutto_after_discounts: Option<f64>,
    pub value_netto: Option<f64>,
    pub value_netto_after_discounts: Option<f64>,
    pub value_vat: Option<f64>,
    pub vat: Option<serde_json::Value>,
    pub vat_name: Option<String>,
    pub vat_percentage: Option<f64>,
    pub vat_value: Option<f64>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DocumentInvoiceRel {
    pub address: Option<String>,
    pub address_customer: Option<String>,
    pub address_delivery: Option<String>,
    pub address_owner: Option<String>,
    pub company: Option<serde_json::Value>,
    pub company_name: Option<String>,
    pub connected_email_status: Option<i64>,
    pub created_at: Option<String>,
    pub creator_user: Option<serde_json::Value>,
    pub currency_code: Option<String>,
    pub default_address: Option<String>,
    pub delivery_place: Option<serde_json::Value>,
    pub description: Option<String>,
    pub document_date: Option<String>,
    pub document_header: Option<String>,
    pub document_order: Option<serde_json::Value>,
    pub document_series_definition: Option<serde_json::Value>,
    pub document_status: Option<i64>,
    pub document_subtype: Option<String>,
    pub document_type: Option<i64>,
    pub exchange: Option<String>,
    pub exchange_ratio: Option<f64>,
    pub external_number: Option<String>,
    pub id: Option<i64>,
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
    pub payment_date: Option<String>,
    pub payment_payed: Option<f64>,
    pub payment_payed_in_currency: Option<f64>,
    pub payment_to_pay: Option<f64>,
    pub payment_to_pay_in_currency: Option<f64>,
    pub payment_type: Option<serde_json::Value>,
    pub positions: Option<serde_json::Value>,
    pub receipt: Option<serde_json::Value>,
    pub storehouse: Option<serde_json::Value>,
    pub to_document: Option<serde_json::Value>,
    pub user_ph: Option<serde_json::Value>,
    pub user_ph_name: Option<String>,
    pub value_brutto: Option<f64>,
    pub value_brutto_in_currency: Option<f64>,
    pub value_netto: Option<f64>,
    pub value_netto_in_currency: Option<f64>,
    pub value_vat: Option<f64>,
    pub value_vat_in_currency: Option<f64>,
    pub vat_conversion_mode: Option<i64>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DocumentOrderRel {
    pub account_user: Option<serde_json::Value>,
    pub address_customer: Option<String>,
    pub address_delivery: Option<String>,
    pub company: Option<serde_json::Value>,
    pub company_name: Option<String>,
    pub created_at: Option<String>,
    pub creator_account_user: Option<serde_json::Value>,
    pub creator_user: Option<serde_json::Value>,
    pub currency_code: Option<String>,
    pub date_delivery: Option<String>,
    pub default_address: Option<String>,
    pub delivery_place: Option<serde_json::Value>,
    pub delivery_type: Option<serde_json::Value>,
    pub discount_code: Option<serde_json::Value>,
    pub document_header: Option<String>,
    pub document_offer_question: Option<serde_json::Value>,
    pub document_series_definition: Option<serde_json::Value>,
    pub document_status: Option<i64>,
    pub exchange: Option<String>,
    pub exchange_history: Option<serde_json::Value>,
    pub exchange_ratio: Option<f64>,
    pub external_date: Option<String>,
    pub external_number: Option<String>,
    pub id: Option<i64>,
    pub import_code: Option<String>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub modification_date: Option<String>,
    pub nip: Option<String>,
    pub notes: Option<String>,
    pub order_type: Option<i64>,
    pub payment_type: Option<serde_json::Value>,
    pub positions: Option<serde_json::Value>,
    pub user_ph: Option<serde_json::Value>,
    pub user_ph_name: Option<String>,
    pub value_brutto: Option<f64>,
    pub value_netto: Option<f64>,
    pub value_netto_before_discount: Option<f64>,
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
pub struct PaymentTypeRel {
    pub check_company_credit_limit: Option<bool>,
    pub description: Option<String>,
    pub id: Option<i64>,
    pub import_code: Option<String>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub is_default: Option<bool>,
    pub name: Option<String>,
    pub online_payment: Option<bool>,
    pub payment_subtype: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub value: Option<f64>,
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
pub struct StorehouseRel {
    pub account: Option<i64>,
    pub description: Option<String>,
    pub id: Option<i64>,
    pub import_code: Option<String>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub is_active: Option<bool>,
    pub is_default: Option<bool>,
    pub name: Option<String>,
    pub short_name: Option<String>,
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
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Create a new resource.
    pub async fn create(&self, data: &Value, params: Option<&QueryParams>) -> Result<DocumentInvoice> {
        let value = self.resource.create(data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Update a resource.
    pub async fn update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<DocumentInvoice> {
        let value = self.resource.update(id, data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Partial update a resource.
    pub async fn partial_update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<DocumentInvoice> {
        let value = self.resource.partial_update(id, data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
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
