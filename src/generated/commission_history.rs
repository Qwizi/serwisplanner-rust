//! Auto-generated module for `CommissionHistory`.
//!
//! Do not edit manually. Regenerate with: `python3 codegen.py <yaml>`

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommissionHistory {
    pub account_user: Option<serde_json::Value>,
    pub attribute: Option<serde_json::Value>,
    pub commission: Option<serde_json::Value>,
    pub commission_history_id: i64,
    pub field_name: Option<String>,
    pub field_type: i64,
    pub modification_date: Option<String>,
    pub new_value: Option<String>,
    pub old_value: Option<String>,
    pub user_user: Option<serde_json::Value>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommissionHistoryListResponse {
    pub data: Vec<CommissionHistory>,
    pub meta: Option<super::ListMeta>,
}

