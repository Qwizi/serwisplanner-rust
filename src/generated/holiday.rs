//! Auto-generated module for `Holiday`.
//!
//! Do not edit manually. Regenerate with: `python3 codegen.py <yaml>`

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Holiday {
    pub holiday: String,
    pub id: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HolidayListResponse {
    pub data: Vec<Holiday>,
    pub meta: Option<super::ListMeta>,
}

