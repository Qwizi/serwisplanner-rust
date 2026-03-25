//! Auto-generated module for `DocumentInvoicePosition`.
//!
//! Do not edit manually. Regenerate with: `python3 codegen.py <yaml>`

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DocumentInvoicePosition {
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
    pub id: i64,
    pub import_code: Option<String>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub is_complet: Option<bool>,
    pub measure_unit: Option<serde_json::Value>,
    pub measure_unit_name: String,
    pub modification_date: Option<String>,
    pub order_position: Option<serde_json::Value>,
    pub ordering: Option<i64>,
    pub price_brutto: Option<f64>,
    pub price_netto: Option<f64>,
    pub price_rabat: Option<f64>,
    pub price_rabat_brutto: Option<f64>,
    pub product: serde_json::Value,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DocumentInvoicePositionListResponse {
    pub data: Vec<DocumentInvoicePosition>,
    pub meta: Option<super::ListMeta>,
}

