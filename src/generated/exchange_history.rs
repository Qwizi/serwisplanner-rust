//! Auto-generated module for `ExchangeHistory`.
//!
//! Do not edit manually. Regenerate with: `python3 codegen.py <yaml>`

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeHistory {
    pub currency_code_from: Option<String>,
    pub currency_code_to: Option<String>,
    pub exchange_date: Option<String>,
    pub exchange_ratio: Option<f64>,
    pub exchange_source: Option<String>,
    pub exchange_type: Option<i64>,
    pub id: i64,
    pub import_code: Option<String>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub pair: Option<String>,
    pub reverse_ratio: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExchangeHistoryListResponse {
    pub data: Vec<ExchangeHistory>,
    pub meta: Option<super::ListMeta>,
}

