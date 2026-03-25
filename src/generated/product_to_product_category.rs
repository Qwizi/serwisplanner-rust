//! Auto-generated module for `ProductToProductCategory`.
//!
//! Do not edit manually. Regenerate with: `python3 codegen.py <yaml>`

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductToProductCategory {
    pub id: i64,
    pub last_element: Option<bool>,
    pub product: Option<serde_json::Value>,
    pub product_category: Option<serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProductToProductCategoryListResponse {
    pub data: Vec<ProductToProductCategory>,
    pub meta: Option<super::ListMeta>,
}

