//! Auto-generated module for `ProductCategory`.
//!
//! Do not edit manually. Regenerate with: `python3 codegen.py <yaml>`

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductCategory {
    pub i18ns: serde_json::Value,
    pub id: i64,
    pub import_code: Option<String>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub name: String,
    pub ordering: Option<i64>,
    pub parent: Option<serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProductCategoryListResponse {
    pub data: Vec<ProductCategory>,
    pub meta: Option<super::ListMeta>,
}

