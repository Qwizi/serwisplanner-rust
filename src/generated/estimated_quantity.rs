//! Auto-generated module for `EstimatedQuantity`.
//!
//! Do not edit manually. Regenerate with: `python3 codegen.py <yaml>`

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimatedQuantity {
    pub category: Option<serde_json::Value>,
    pub description: Option<String>,
    pub file: Option<serde_json::Value>,
    pub id: i64,
    pub product: Option<serde_json::Value>,
    pub quantity_from: Option<f64>,
    pub quantity_to: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EstimatedQuantityListResponse {
    pub data: Vec<EstimatedQuantity>,
    pub meta: Option<super::ListMeta>,
}

