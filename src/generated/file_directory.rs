//! Auto-generated module for `FileDirectory`.
//!
//! Do not edit manually. Regenerate with: `python3 codegen.py <yaml>`

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDirectory {
    pub id: i64,
    pub is_last_directory: Option<bool>,
    pub name: String,
    pub parent_directory: Option<serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileDirectoryListResponse {
    pub data: Vec<FileDirectory>,
    pub meta: Option<super::ListMeta>,
}

