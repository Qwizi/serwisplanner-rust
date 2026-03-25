//! Auto-generated module for `OdbcReport`.
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
pub struct OdbcReport {
    pub autoformat_numbers: Option<bool>,
    pub available_parameters: Option<String>,
    pub config_format: Option<String>,
    pub defaults: Option<String>,
    pub edit_visibility: Option<bool>,
    pub enable_autofilters: Option<bool>,
    pub enable_grouping: Option<bool>,
    pub enable_sorting: Option<bool>,
    pub export_sql: Option<String>,
    pub force_defaults: Option<String>,
    pub hide_from_list: Option<bool>,
    pub hide_settings: Option<bool>,
    pub is_active: Option<bool>,
    pub last_modification: Option<String>,
    pub odbc_database_encoding: Option<String>,
    pub odbc_database_type: Option<String>,
    pub odbc_report_id: Option<i64>,
    pub odbc_reports_availabilities: Option<OdbcReportAvailabilityRel>,
    pub odbc_reports_credentials: Option<OdbcReportCredentialRel>,
    pub ordering: Option<i64>,
    pub pagination: Option<bool>,
    pub partial_summary: Option<bool>,
    pub partial_summary_sql: Option<String>,
    pub pdf_page_rows: Option<i64>,
    pub query_sql: Option<String>,
    pub report_columns: Option<String>,
    pub report_columns_date: Option<String>,
    pub report_configuration: Option<String>,
    pub report_description: Option<String>,
    pub report_licenses: Option<String>,
    pub report_name: Option<String>,
    pub report_output: Option<String>,
    pub report_type: Option<i64>,
    pub report_uuid: Option<String>,
    pub show_filters: Option<bool>,
    pub show_report_name: Option<bool>,
    pub show_summary: Option<bool>,
    pub static_header: Option<bool>,
    pub xml_template: Option<ProductTemplateRel>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct OdbcReportAvailabilityRel {
    pub availability_type: Option<i64>,
    pub odbc_report: Option<serde_json::Value>,
    pub odbc_report_availability_id: Option<i64>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct OdbcReportCredentialRel {
    pub credential_id: Option<i64>,
    pub odbc_report: Option<serde_json::Value>,
    pub user_profile: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ProductTemplateRel {
    pub custom_css: Option<String>,
    pub description: Option<String>,
    pub id: Option<i64>,
    pub is_default_for_type: Option<bool>,
    pub is_disallow_print_to_pdf: Option<bool>,
    pub language: Option<String>,
    pub name: Option<String>,
    pub options: Option<String>,
    pub pdf_prefix: Option<String>,
    pub plain: Option<bool>,
    pub priority: Option<i64>,
    pub read_only: Option<bool>,
    #[serde(rename = "type")]
    pub r#type: Option<i64>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OdbcReportListResponse {
    pub data: Vec<OdbcReport>,
    pub meta: Option<super::ListMeta>,
}

/// Resource accessor for `/api/odbc_reports`.
pub struct OdbcReportResource {
    resource: Resource,
}

impl OdbcReportResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/odbc_reports"),
        }
    }

    /// List resources.
    pub async fn list(&self, params: Option<&QueryParams>) -> Result<OdbcReportListResponse> {
        let value = self.resource.list(params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Retrieve a single resource by ID.
    pub async fn retrieve(&self, id: u64, params: Option<&QueryParams>) -> Result<OdbcReport> {
        let value = self.resource.retrieve(id, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Create a new resource.
    pub async fn create(&self, data: &Value, params: Option<&QueryParams>) -> Result<OdbcReport> {
        let value = self.resource.create(data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Update a resource.
    pub async fn update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<OdbcReport> {
        let value = self.resource.update(id, data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Partial update a resource.
    pub async fn partial_update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<OdbcReport> {
        let value = self.resource.partial_update(id, data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }

    /// Delete a resource.
    pub async fn delete(&self, id: u64, params: Option<&QueryParams>) -> Result<Value> {
        self.resource.delete(id, params).await
    }

    /// Fetch all pages automatically.
    pub async fn all(&self, params: Option<QueryParams>) -> Result<Vec<OdbcReport>> {
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
