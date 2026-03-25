//! Auto-generated module for `TaskHistory`.
//!
//! Do not edit manually. Regenerate with: `python3 codegen.py <yaml>`

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskHistory {
    pub attribute: Option<serde_json::Value>,
    pub field_name: Option<String>,
    pub field_type: Option<i64>,
    pub id: i64,
    pub modification_date: Option<String>,
    pub new_value: Option<String>,
    pub old_value: Option<String>,
    pub task: Option<serde_json::Value>,
    pub user_user: Option<serde_json::Value>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskHistoryListResponse {
    pub data: Vec<TaskHistory>,
    pub meta: Option<super::ListMeta>,
}

