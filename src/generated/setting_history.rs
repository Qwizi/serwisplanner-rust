//! Auto-generated module for `SettingHistory`.
//!
//! Do not edit manually. Regenerate with: `python3 codegen.py <yaml>`

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingHistory {
    pub id: i64,
    pub modification_date: Option<String>,
    pub new_value: Option<String>,
    pub old_value: Option<String>,
    pub setting: Option<serde_json::Value>,
    pub setting_name: Option<String>,
    pub user_user: Option<serde_json::Value>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SettingHistoryListResponse {
    pub data: Vec<SettingHistory>,
    pub meta: Option<super::ListMeta>,
}

