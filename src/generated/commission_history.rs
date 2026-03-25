//! Auto-generated module for `CommissionHistory`.
//!
//! Do not edit manually. Regenerate with: `python3 codegen.py <yaml>`

use std::sync::Arc;
use serde_json::Value;
use crate::client::ClientInner;
use crate::error::Result;
use crate::params::QueryParams;
use crate::resources::base::Resource;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CommissionHistory {
    pub account_user: Option<AccountUserRel>,
    pub attribute: Option<CommissionAttributeRel>,
    pub commission: Option<CommissionRel>,
    pub commission_history_id: Option<i64>,
    pub field_name: Option<String>,
    pub field_type: Option<i64>,
    pub modification_date: Option<String>,
    pub new_value: Option<String>,
    pub old_value: Option<String>,
    pub user_user: Option<UserUserRel>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AccountUserRel {
    pub active_currency: Option<String>,
    pub additional_contacts: Option<serde_json::Value>,
    pub authorized_to_make_orders: Option<bool>,
    pub available_currencies: Option<String>,
    pub avatar: Option<String>,
    pub avatar_file: Option<serde_json::Value>,
    pub company: Option<serde_json::Value>,
    pub consent_processing_personal_data: Option<bool>,
    pub default_currency: Option<String>,
    pub default_language: Option<String>,
    pub email: Option<String>,
    pub failed_login_counter: Option<i64>,
    pub first_name: Option<String>,
    pub id: Option<i64>,
    pub import_code: Option<String>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub is2fa_active: Option<bool>,
    pub is_invoice_user: Option<bool>,
    pub language: Option<String>,
    pub last_failed_login: Option<String>,
    pub last_failed_login_ip: Option<String>,
    pub last_login: Option<String>,
    pub last_login_ip: Option<String>,
    pub last_name: Option<String>,
    pub method2fa: Option<String>,
    pub mobile_number: Option<String>,
    pub modification_date: Option<String>,
    pub newsletter_active: Option<bool>,
    pub phone_number: Option<String>,
    pub place: Option<serde_json::Value>,
    pub scoring: Option<i64>,
    pub system_theme: Option<String>,
    pub user_profile: Option<serde_json::Value>,
    pub username: Option<String>,
    pub verified: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CommissionAttributeRel {
    pub additional_options: Option<String>,
    pub attribute_multis: Option<serde_json::Value>,
    pub chatbox: Option<bool>,
    pub display_priority: Option<i64>,
    pub document_series_definition: Option<String>,
    pub file: Option<serde_json::Value>,
    pub filter_as_tags: Option<bool>,
    pub id: Option<i64>,
    pub is_active: Option<bool>,
    pub is_for_position: Option<bool>,
    pub name: Option<String>,
    pub option: Option<String>,
    pub ordering: Option<i64>,
    pub parent: Option<serde_json::Value>,
    pub register_event_onchange: Option<bool>,
    pub searching_for_options: Option<bool>,
    pub task_type: Option<serde_json::Value>,
    pub tooltip: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<i64>,
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CommissionRel {
    pub account_user: Option<serde_json::Value>,
    pub account_user_name: Option<String>,
    pub commission_address: Option<String>,
    pub commission_number: Option<String>,
    pub commission_phase: Option<serde_json::Value>,
    pub commission_phase_id: Option<i64>,
    pub commissions_user_users: Option<serde_json::Value>,
    pub company: Option<serde_json::Value>,
    pub company_name: Option<String>,
    pub created_at: Option<String>,
    pub creator_account_user: Option<serde_json::Value>,
    pub creator_user: Option<serde_json::Value>,
    pub document_series_definition: Option<serde_json::Value>,
    pub end_date: Option<String>,
    pub id: Option<i64>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub modification_date: Option<String>,
    pub name: Option<String>,
    pub phase_modification_date: Option<String>,
    pub place: Option<serde_json::Value>,
    pub positions: Option<serde_json::Value>,
    pub start_date: Option<String>,
    pub user: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct UserUserRel {
    pub active_currency: Option<String>,
    pub available_currencies: Option<String>,
    pub avatar: Option<String>,
    pub avatar_file: Option<serde_json::Value>,
    pub default_callendar_color: Option<String>,
    pub default_css_file: Option<serde_json::Value>,
    pub default_currency: Option<String>,
    pub email: Option<String>,
    pub failed_login_counter: Option<i64>,
    pub first_name: Option<String>,
    pub id: Option<i64>,
    pub import_code: Option<String>,
    pub import_id: Option<i64>,
    pub import_typ: Option<String>,
    pub is2fa_active: Option<bool>,
    pub is_external_login: Option<bool>,
    pub language: Option<String>,
    pub last_action: Option<String>,
    pub last_failed_login: Option<String>,
    pub last_login_date: Option<String>,
    pub last_logout_date: Option<String>,
    pub last_name: Option<String>,
    pub logged_in: Option<bool>,
    pub message_on_email: Option<bool>,
    pub method2fa: Option<String>,
    pub mobile_number: Option<String>,
    pub modification_date: Option<String>,
    pub msexchange: Option<bool>,
    pub phone_number: Option<String>,
    pub reset_link_expires: Option<String>,
    pub session_id: Option<String>,
    pub subordinates: Option<serde_json::Value>,
    pub superior: Option<serde_json::Value>,
    pub system_theme: Option<String>,
    pub uri: Option<String>,
    pub user_profile: Option<serde_json::Value>,
    pub username: Option<String>,
    pub verified: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CommissionHistoryListResponse {
    pub data: Vec<CommissionHistory>,
    pub meta: Option<super::ListMeta>,
}

/// Resource accessor for `/api/commission_histories`.
pub struct CommissionHistoryResource {
    resource: Resource,
}

impl CommissionHistoryResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/commission_histories"),
        }
    }

    /// List resources.
    pub async fn list(&self, params: Option<&QueryParams>) -> Result<CommissionHistoryListResponse> {
        let value = self.resource.list(params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Retrieve a single resource by ID.
    pub async fn retrieve(&self, id: u64, params: Option<&QueryParams>) -> Result<CommissionHistory> {
        let value = self.resource.retrieve(id, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Create a new resource.
    pub async fn create(&self, data: &Value, params: Option<&QueryParams>) -> Result<CommissionHistory> {
        let value = self.resource.create(data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Update a resource.
    pub async fn update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<CommissionHistory> {
        let value = self.resource.update(id, data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Partial update a resource.
    pub async fn partial_update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<CommissionHistory> {
        let value = self.resource.partial_update(id, data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Delete a resource.
    pub async fn delete(&self, id: u64, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.delete(id, params).await
    }

    /// Fetch all pages automatically.
    pub async fn all(&self, params: Option<QueryParams>) -> Result<Vec<CommissionHistory>> {
        let items = self.resource.all(params).await?;
        items.into_iter()
            .map(|v| serde_json::from_value(v).map_err(|e| crate::error::SWError::Other(e.to_string())))
            .collect()
    }

    /// Get resource metadata.
    pub async fn meta(&self, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.meta(params).await
    }

    /// Autoselect.
    pub async fn autoselect(&self, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.autoselect(params).await
    }

    /// Access the untyped base resource.
    pub fn raw(&self) -> &Resource {
        &self.resource
    }
}
