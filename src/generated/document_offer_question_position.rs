//! Auto-generated module for `DocumentOfferQuestionPosition`.
//!
//! Do not edit manually. Regenerate with: `python3 codegen.py <yaml>`

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DocumentOfferQuestionPosition {
    pub base_price_brutto: Option<f64>,
    pub base_price_netto: Option<f64>,
    pub base_rabat: Option<f64>,
    pub base_value_brutto: Option<f64>,
    pub base_value_netto: Option<f64>,
    pub document_offer_question: Option<DocumentOfferQuestionRel>,
    pub id: Option<i64>,
    pub import_code: Option<String>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub measure_unit: Option<MeasureUnitRel>,
    pub measure_unit_name: Option<String>,
    pub ordering: Option<i64>,
    pub position_type: Option<i64>,
    pub price_brutto: Option<f64>,
    pub price_netto: Option<f64>,
    pub price_rabat: Option<f64>,
    pub price_rabat_brutto: Option<f64>,
    pub product: Option<ProductRel>,
    pub product_code: Option<String>,
    pub product_complete: Option<ProductCompleteRel>,
    pub product_freetype: Option<ProductFreetypeRel>,
    pub product_name: Option<String>,
    pub quantity: Option<f64>,
    pub rabat: Option<f64>,
    pub value_brutto: Option<f64>,
    pub value_brutto_after_discounts: Option<f64>,
    pub value_netto: Option<f64>,
    pub value_netto_after_discounts: Option<f64>,
    pub vat: Option<VatRel>,
    pub vat_name: Option<String>,
    pub vat_value: Option<f64>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DocumentOfferQuestionRel {
    pub account_user: Option<serde_json::Value>,
    pub company: Option<serde_json::Value>,
    pub company_name: Option<String>,
    pub created_at: Option<String>,
    pub creator_account_user: Option<serde_json::Value>,
    pub creator_user: Option<serde_json::Value>,
    pub currency_code: Option<String>,
    pub currency_ratio: Option<f64>,
    pub default_address: Option<String>,
    pub delivery_place: Option<serde_json::Value>,
    pub delivery_type: Option<serde_json::Value>,
    pub document_header: Option<String>,
    pub document_series_definition: Option<serde_json::Value>,
    pub document_status: Option<i64>,
    pub document_subtype: Option<String>,
    pub exchange: Option<String>,
    pub exchange_history: Option<serde_json::Value>,
    pub exchange_ratio: Option<f64>,
    pub id: Option<i64>,
    pub import_code: Option<String>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub manual_status_changed: Option<bool>,
    pub modification_date: Option<String>,
    pub nip: Option<String>,
    pub notes: Option<String>,
    pub order_deadline: Option<String>,
    pub order_time: Option<String>,
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
pub struct MeasureUnitRel {
    pub import_code: Option<String>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub measure_unit_id: Option<i64>,
    pub name: Option<String>,
    pub short_name: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ProductCompleteRel {
    pub child: Option<serde_json::Value>,
    pub id: Option<i64>,
    pub parent: Option<serde_json::Value>,
    pub price_netto: Option<f64>,
    pub quantity: Option<f64>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ProductFreetypeRel {
    pub code_corrected: Option<String>,
    pub creation_date: Option<String>,
    pub id: Option<i64>,
    pub measure_unit: Option<serde_json::Value>,
    pub name_corrected: Option<String>,
    pub product_description: Option<String>,
    pub product_name: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ProductRel {
    pub a_currency_code: Option<String>,
    pub a_price_brutto: Option<f64>,
    pub a_price_netto: Option<f64>,
    pub a_vat_value: Option<f64>,
    pub b_currency_code: Option<String>,
    pub b_price_brutto: Option<f64>,
    pub b_price_netto: Option<f64>,
    pub b_vat_value: Option<f64>,
    pub bar_code: Option<String>,
    pub c_currency_code: Option<String>,
    pub c_price_brutto: Option<f64>,
    pub c_price_netto: Option<f64>,
    pub c_vat_value: Option<f64>,
    pub category: Option<serde_json::Value>,
    pub code: Option<String>,
    pub company_prices: Option<serde_json::Value>,
    pub d_currency_code: Option<String>,
    pub d_price_brutto: Option<f64>,
    pub d_price_netto: Option<f64>,
    pub d_vat_value: Option<f64>,
    pub default_image: Option<serde_json::Value>,
    pub description: Option<String>,
    pub e_currency_code: Option<String>,
    pub e_price_brutto: Option<f64>,
    pub e_price_netto: Option<f64>,
    pub e_vat_value: Option<f64>,
    pub ean: Option<String>,
    pub estimated_quantity: Option<String>,
    pub external_marker: Option<serde_json::Value>,
    pub external_markers: Option<serde_json::Value>,
    pub f_currency_code: Option<String>,
    pub f_price_brutto: Option<f64>,
    pub f_price_netto: Option<f64>,
    pub f_vat_value: Option<f64>,
    pub g_currency_code: Option<String>,
    pub g_price_brutto: Option<f64>,
    pub g_price_netto: Option<f64>,
    pub g_vat_value: Option<f64>,
    pub h_currency_code: Option<String>,
    pub h_price_brutto: Option<f64>,
    pub h_price_netto: Option<f64>,
    pub h_vat_value: Option<f64>,
    pub i18ns: Option<serde_json::Value>,
    pub id: Option<i64>,
    pub import_code: Option<String>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub is_complete: Option<bool>,
    pub is_main_product: Option<bool>,
    pub is_service: Option<bool>,
    pub measure_unit: Option<serde_json::Value>,
    pub modification_date: Option<String>,
    pub name: Option<String>,
    pub price_individuals: Option<serde_json::Value>,
    pub product_group: Option<serde_json::Value>,
    pub quantity: Option<f64>,
    pub quantity_precision: Option<i64>,
    pub quantity_raw: Option<f64>,
    pub quantity_reserved: Option<f64>,
    pub srp_price_type: Option<String>,
    pub translation_i18ns: Option<serde_json::Value>,
    pub vat: Option<serde_json::Value>,
    pub vat_value: Option<f64>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct VatRel {
    pub id: Option<i64>,
    pub import_code: Option<String>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub vat_description: Option<String>,
    pub vat_name: Option<String>,
    pub vat_value: Option<f64>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DocumentOfferQuestionPositionListResponse {
    pub data: Vec<DocumentOfferQuestionPosition>,
    pub meta: Option<super::ListMeta>,
}

