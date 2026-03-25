//! Auto-generated module for `CampaignOpportunity`.
//!
//! Do not edit manually. Regenerate with: `python3 codegen.py <yaml>`

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignOpportunity {
    pub account_user: Option<serde_json::Value>,
    pub campaign: serde_json::Value,
    pub campaign_phase: Option<serde_json::Value>,
    pub company: Option<serde_json::Value>,
    pub currency: Option<String>,
    pub description: Option<String>,
    pub estimated_close_date: Option<String>,
    pub id: i64,
    pub import_code: Option<String>,
    pub import_id: Option<String>,
    pub import_typ: Option<String>,
    pub lost_caused_by: Option<String>,
    pub lost_description: Option<String>,
    pub modification_date: Option<String>,
    pub ordering: Option<i64>,
    pub real_close_date: Option<String>,
    pub start_date: Option<String>,
    pub status: Option<String>,
    pub title: String,
    pub user: Option<serde_json::Value>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CampaignOpportunityListResponse {
    pub data: Vec<CampaignOpportunity>,
    pub meta: Option<super::ListMeta>,
}

