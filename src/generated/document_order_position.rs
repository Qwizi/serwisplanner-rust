//! Auto-generated module for `DocumentOrderPosition`.
//!
//! Do not edit manually. Regenerate with: `python3 codegen.py <yaml>`

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DocumentOrderPosition {
    pub base_price_brutto: Option<f64>,
    pub base_price_netto: Option<f64>,
    pub base_rabat: Option<f64>,
    pub base_value_brutto: Option<f64>,
    pub base_value_netto: Option<f64>,
    pub basket_position: Option<serde_json::Value>,
    pub document_order: Option<serde_json::Value>,
    pub id: i64,
    pub import_code: Option<String>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub measure_unit: Option<serde_json::Value>,
    pub measure_unit_name: Option<String>,
    pub ordering: Option<i64>,
    pub position_type: i64,
    pub price_brutto: Option<f64>,
    pub price_netto: Option<f64>,
    pub price_rabat: Option<f64>,
    pub price_rabat_brutto: Option<f64>,
    pub product: serde_json::Value,
    pub product_code: Option<String>,
    pub product_complete: Option<serde_json::Value>,
    pub product_freetype: Option<serde_json::Value>,
    pub product_name: Option<String>,
    pub quantity: f64,
    pub rabat: Option<f64>,
    pub value_brutto: Option<f64>,
    pub value_brutto_after_discounts: Option<f64>,
    pub value_netto: Option<f64>,
    pub value_netto_after_discounts: Option<f64>,
    pub vat: Option<serde_json::Value>,
    pub vat_name: Option<String>,
    pub vat_value: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DocumentOrderPositionListResponse {
    pub data: Vec<DocumentOrderPosition>,
    pub meta: Option<super::ListMeta>,
}

