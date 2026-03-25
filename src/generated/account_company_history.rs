//! Auto-generated module for `AccountCompanyHistory`.
//!
//! Do not edit manually. Regenerate with: `python3 codegen.py <yaml>`

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountCompanyHistory {
    pub account_company: Option<serde_json::Value>,
    pub attribute: Option<serde_json::Value>,
    pub field_name: Option<String>,
    pub field_type: i64,
    pub id: i64,
    pub modification_date: Option<String>,
    pub new_value: Option<String>,
    pub old_value: Option<String>,
    pub user_user: Option<serde_json::Value>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccountCompanyHistoryListResponse {
    pub data: Vec<AccountCompanyHistory>,
    pub meta: Option<super::ListMeta>,
}

