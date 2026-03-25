use std::sync::Arc;

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde_json::Value;
use tokio::sync::RwLock;

use crate::error::{raise_for_status, Result, SWError};
use crate::resources::{AuthResource, FilesResource};

pub(crate) struct ClientInner {
    pub http: reqwest::Client,
    pub base_url: String,
    pub token: RwLock<Option<String>>,
}

impl ClientInner {
    pub async fn request(
        &self,
        method: reqwest::Method,
        path: &str,
        json: Option<&Value>,
        query: Option<&[(String, String)]>,
    ) -> Result<Value> {
        let url = format!("{}{}", self.base_url, path);
        let mut builder = self.http.request(method, &url);

        // Auth header
        if let Some(token) = self.token.read().await.as_ref() {
            builder = builder.header(AUTHORIZATION, format!("Bearer {}", token));
        }

        // Default headers
        builder = builder
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json");

        if let Some(query) = query {
            builder = builder.query(query);
        }

        if let Some(body) = json {
            builder = builder.json(body);
        }

        let response = builder.send().await.map_err(|e| {
            if e.is_connect() || e.is_timeout() {
                SWError::Connection(e.to_string())
            } else {
                SWError::Connection(e.to_string())
            }
        })?;

        let status = response.status().as_u16();

        if status == 204 {
            return Ok(Value::Object(serde_json::Map::new()));
        }

        let data: Value = response
            .json()
            .await
            .unwrap_or(Value::Object(serde_json::Map::new()));

        if status >= 400 {
            return Err(raise_for_status(status, data));
        }

        Ok(data)
    }

    pub async fn request_multipart(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
    ) -> Result<Value> {
        let url = format!("{}{}", self.base_url, path);
        let mut builder = self.http.post(&url).header(ACCEPT, "application/json");

        if let Some(token) = self.token.read().await.as_ref() {
            builder = builder.header(AUTHORIZATION, format!("Bearer {}", token));
        }

        builder = builder.multipart(form);

        let response = builder
            .send()
            .await
            .map_err(|e| SWError::Connection(e.to_string()))?;

        let status = response.status().as_u16();
        let data: Value = response
            .json()
            .await
            .unwrap_or(Value::Object(serde_json::Map::new()));

        if status >= 400 {
            return Err(raise_for_status(status, data));
        }

        Ok(data)
    }

    pub async fn set_token(&self, token: String) {
        *self.token.write().await = Some(token);
    }
}

/// Async client for the Serwis Planner API.
///
/// # Examples
/// ```no_run
/// use serwis_planner::SerwisPlanner;
///
/// # async fn example() -> serwis_planner::error::Result<()> {
/// let client = SerwisPlanner::new("https://api.example.com");
/// client.auth().login("client_id", "auth_token", "login", "password").await?;
///
/// let me = client.me().await?;
/// let companies = client.account().companies().list(None).await?;
/// # Ok(())
/// # }
/// ```
pub struct SerwisPlanner {
    pub(crate) inner: Arc<ClientInner>,
}

impl SerwisPlanner {
    pub fn new(api_url: &str) -> Self {
        Self::with_options(api_url, None, None)
    }

    pub fn with_options(
        api_url: &str,
        timeout_secs: Option<u64>,
        user_agent: Option<&str>,
    ) -> Self {
        let mut headers = HeaderMap::new();
        let ua = user_agent.unwrap_or("SerwisPlanner/0.1.0 (Rust)");
        headers.insert(USER_AGENT, HeaderValue::from_str(ua).unwrap());

        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(timeout_secs.unwrap_or(30)))
            .default_headers(headers)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            inner: Arc::new(ClientInner {
                http,
                base_url: api_url.trim_end_matches('/').to_string(),
                token: RwLock::new(None),
            }),
        }
    }

    /// Set the Bearer token manually (alternative to login).
    pub async fn set_token(&self, token: &str) {
        self.inner.set_token(token.to_string()).await;
    }

    /// Get current user data (`GET /api/me`).
    pub async fn me(&self) -> Result<Value> {
        self.inner
            .request(reqwest::Method::GET, "/api/me", None, None)
            .await
    }

    /// Get mobile phases config (`GET /api/mobile_service_app_configs/1`).
    pub async fn mobile_phases_config(&self) -> Result<Value> {
        self.inner
            .request(
                reqwest::Method::GET,
                "/api/mobile_service_app_configs/1",
                None,
                None,
            )
            .await
    }

    // --- Resource accessors ---

    pub fn auth(&self) -> AuthResource {
        AuthResource::new(self.inner.clone())
    }

    pub fn files(&self) -> FilesResource {
        FilesResource::new(self.inner.clone())
    }

    // --- Typed (auto-generated) resource accessors ---

    pub fn account_company_attribute(&self) -> crate::generated::AccountCompanyAttributeResource {
        crate::generated::AccountCompanyAttributeResource::new(self.inner.clone())
    }
    pub fn account_user(&self) -> crate::generated::AccountUserResource {
        crate::generated::AccountUserResource::new(self.inner.clone())
    }
    pub fn account_user_attribute(&self) -> crate::generated::AccountUserAttributeResource {
        crate::generated::AccountUserAttributeResource::new(self.inner.clone())
    }
    pub fn additional_i18n(&self) -> crate::generated::AdditionalI18nResource {
        crate::generated::AdditionalI18nResource::new(self.inner.clone())
    }
    pub fn basket(&self) -> crate::generated::BasketResource {
        crate::generated::BasketResource::new(self.inner.clone())
    }
    pub fn basket_position(&self) -> crate::generated::BasketPositionResource {
        crate::generated::BasketPositionResource::new(self.inner.clone())
    }
    pub fn campaign(&self) -> crate::generated::CampaignResource {
        crate::generated::CampaignResource::new(self.inner.clone())
    }
    pub fn campaign_attribute(&self) -> crate::generated::CampaignAttributeResource {
        crate::generated::CampaignAttributeResource::new(self.inner.clone())
    }
    pub fn campaign_phase(&self) -> crate::generated::CampaignPhaseResource {
        crate::generated::CampaignPhaseResource::new(self.inner.clone())
    }
    pub fn commission(&self) -> crate::generated::CommissionResource {
        crate::generated::CommissionResource::new(self.inner.clone())
    }
    pub fn commission_attribute(&self) -> crate::generated::CommissionAttributeResource {
        crate::generated::CommissionAttributeResource::new(self.inner.clone())
    }
    pub fn commission_attribute_criteria(&self) -> crate::generated::CommissionAttributeCriteriaResource {
        crate::generated::CommissionAttributeCriteriaResource::new(self.inner.clone())
    }
    pub fn commission_attribute_relation_actions(&self) -> crate::generated::CommissionAttributeRelationActionsResource {
        crate::generated::CommissionAttributeRelationActionsResource::new(self.inner.clone())
    }
    pub fn commission_attribute_relations(&self) -> crate::generated::CommissionAttributeRelationsResource {
        crate::generated::CommissionAttributeRelationsResource::new(self.inner.clone())
    }
    pub fn commission_phase(&self) -> crate::generated::CommissionPhaseResource {
        crate::generated::CommissionPhaseResource::new(self.inner.clone())
    }
    pub fn commission_scope_type(&self) -> crate::generated::CommissionScopeTypeResource {
        crate::generated::CommissionScopeTypeResource::new(self.inner.clone())
    }
    pub fn commission_shortcut(&self) -> crate::generated::CommissionShortcutResource {
        crate::generated::CommissionShortcutResource::new(self.inner.clone())
    }
    pub fn commission_template(&self) -> crate::generated::CommissionTemplateResource {
        crate::generated::CommissionTemplateResource::new(self.inner.clone())
    }
    pub fn commissions_user_users(&self) -> crate::generated::CommissionsUserUsersResource {
        crate::generated::CommissionsUserUsersResource::new(self.inner.clone())
    }
    pub fn delivery_cost_definition(&self) -> crate::generated::DeliveryCostDefinitionResource {
        crate::generated::DeliveryCostDefinitionResource::new(self.inner.clone())
    }
    pub fn delivery_type(&self) -> crate::generated::DeliveryTypeResource {
        crate::generated::DeliveryTypeResource::new(self.inner.clone())
    }
    pub fn discount(&self) -> crate::generated::DiscountResource {
        crate::generated::DiscountResource::new(self.inner.clone())
    }
    pub fn discount_code(&self) -> crate::generated::DiscountCodeResource {
        crate::generated::DiscountCodeResource::new(self.inner.clone())
    }
    pub fn document_attribute(&self) -> crate::generated::DocumentAttributeResource {
        crate::generated::DocumentAttributeResource::new(self.inner.clone())
    }
    pub fn document_invoice(&self) -> crate::generated::DocumentInvoiceResource {
        crate::generated::DocumentInvoiceResource::new(self.inner.clone())
    }
    pub fn document_offer(&self) -> crate::generated::DocumentOfferResource {
        crate::generated::DocumentOfferResource::new(self.inner.clone())
    }
    pub fn document_offer_attribute(&self) -> crate::generated::DocumentOfferAttributeResource {
        crate::generated::DocumentOfferAttributeResource::new(self.inner.clone())
    }
    pub fn document_offer_question(&self) -> crate::generated::DocumentOfferQuestionResource {
        crate::generated::DocumentOfferQuestionResource::new(self.inner.clone())
    }
    pub fn document_offer_question_attribute(&self) -> crate::generated::DocumentOfferQuestionAttributeResource {
        crate::generated::DocumentOfferQuestionAttributeResource::new(self.inner.clone())
    }
    pub fn document_order(&self) -> crate::generated::DocumentOrderResource {
        crate::generated::DocumentOrderResource::new(self.inner.clone())
    }
    pub fn document_series_definition(&self) -> crate::generated::DocumentSeriesDefinitionResource {
        crate::generated::DocumentSeriesDefinitionResource::new(self.inner.clone())
    }
    pub fn document_store(&self) -> crate::generated::DocumentStoreResource {
        crate::generated::DocumentStoreResource::new(self.inner.clone())
    }
    pub fn email_account(&self) -> crate::generated::EmailAccountResource {
        crate::generated::EmailAccountResource::new(self.inner.clone())
    }
    pub fn email_message(&self) -> crate::generated::EmailMessageResource {
        crate::generated::EmailMessageResource::new(self.inner.clone())
    }
    pub fn email_message_queue(&self) -> crate::generated::EmailMessageQueueResource {
        crate::generated::EmailMessageQueueResource::new(self.inner.clone())
    }
    pub fn email_template(&self) -> crate::generated::EmailTemplateResource {
        crate::generated::EmailTemplateResource::new(self.inner.clone())
    }
    pub fn exchange(&self) -> crate::generated::ExchangeResource {
        crate::generated::ExchangeResource::new(self.inner.clone())
    }
    pub fn external_marker(&self) -> crate::generated::ExternalMarkerResource {
        crate::generated::ExternalMarkerResource::new(self.inner.clone())
    }
    pub fn favorite(&self) -> crate::generated::FavoriteResource {
        crate::generated::FavoriteResource::new(self.inner.clone())
    }
    pub fn favorite_position(&self) -> crate::generated::FavoritePositionResource {
        crate::generated::FavoritePositionResource::new(self.inner.clone())
    }
    pub fn file_directory_credential(&self) -> crate::generated::FileDirectoryCredentialResource {
        crate::generated::FileDirectoryCredentialResource::new(self.inner.clone())
    }
    pub fn gantt(&self) -> crate::generated::GanttResource {
        crate::generated::GanttResource::new(self.inner.clone())
    }
    pub fn imap_folder(&self) -> crate::generated::ImapFolderResource {
        crate::generated::ImapFolderResource::new(self.inner.clone())
    }
    pub fn internal_document_order(&self) -> crate::generated::InternalDocumentOrderResource {
        crate::generated::InternalDocumentOrderResource::new(self.inner.clone())
    }
    pub fn kanban(&self) -> crate::generated::KanbanResource {
        crate::generated::KanbanResource::new(self.inner.clone())
    }
    pub fn main_menu(&self) -> crate::generated::MainMenuResource {
        crate::generated::MainMenuResource::new(self.inner.clone())
    }
    pub fn markers(&self) -> crate::generated::MarkersResource {
        crate::generated::MarkersResource::new(self.inner.clone())
    }
    pub fn measure_unit(&self) -> crate::generated::MeasureUnitResource {
        crate::generated::MeasureUnitResource::new(self.inner.clone())
    }
    pub fn message(&self) -> crate::generated::MessageResource {
        crate::generated::MessageResource::new(self.inner.clone())
    }
    pub fn message_thread(&self) -> crate::generated::MessageThreadResource {
        crate::generated::MessageThreadResource::new(self.inner.clone())
    }
    pub fn message_thread_user(&self) -> crate::generated::MessageThreadUserResource {
        crate::generated::MessageThreadUserResource::new(self.inner.clone())
    }
    pub fn odbc_report(&self) -> crate::generated::OdbcReportResource {
        crate::generated::OdbcReportResource::new(self.inner.clone())
    }
    pub fn payment_type(&self) -> crate::generated::PaymentTypeResource {
        crate::generated::PaymentTypeResource::new(self.inner.clone())
    }
    pub fn place_attribute(&self) -> crate::generated::PlaceAttributeResource {
        crate::generated::PlaceAttributeResource::new(self.inner.clone())
    }
    pub fn place(&self) -> crate::generated::PlacesResource {
        crate::generated::PlacesResource::new(self.inner.clone())
    }
    pub fn price_individual(&self) -> crate::generated::PriceIndividualResource {
        crate::generated::PriceIndividualResource::new(self.inner.clone())
    }
    pub fn price_product_group(&self) -> crate::generated::PriceProductGroupResource {
        crate::generated::PriceProductGroupResource::new(self.inner.clone())
    }
    pub fn price_types(&self) -> crate::generated::PriceTypesResource {
        crate::generated::PriceTypesResource::new(self.inner.clone())
    }
    pub fn product(&self) -> crate::generated::ProductResource {
        crate::generated::ProductResource::new(self.inner.clone())
    }
    pub fn product_attribute(&self) -> crate::generated::ProductAttributeResource {
        crate::generated::ProductAttributeResource::new(self.inner.clone())
    }
    pub fn product_template(&self) -> crate::generated::ProductTemplateResource {
        crate::generated::ProductTemplateResource::new(self.inner.clone())
    }
    pub fn profile_setting(&self) -> crate::generated::ProfileSettingResource {
        crate::generated::ProfileSettingResource::new(self.inner.clone())
    }
    pub fn serviced_product(&self) -> crate::generated::ServicedProductResource {
        crate::generated::ServicedProductResource::new(self.inner.clone())
    }
    pub fn serviced_product_attribute(&self) -> crate::generated::ServicedProductAttributeResource {
        crate::generated::ServicedProductAttributeResource::new(self.inner.clone())
    }
    pub fn setting(&self) -> crate::generated::SettingResource {
        crate::generated::SettingResource::new(self.inner.clone())
    }
    pub fn storehouse(&self) -> crate::generated::StorehouseResource {
        crate::generated::StorehouseResource::new(self.inner.clone())
    }
    pub fn target(&self) -> crate::generated::TargetResource {
        crate::generated::TargetResource::new(self.inner.clone())
    }
    pub fn task(&self) -> crate::generated::TaskResource {
        crate::generated::TaskResource::new(self.inner.clone())
    }
    pub fn task_type(&self) -> crate::generated::TaskTypeResource {
        crate::generated::TaskTypeResource::new(self.inner.clone())
    }
    pub fn user_geolocation(&self) -> crate::generated::UserGeolocationResource {
        crate::generated::UserGeolocationResource::new(self.inner.clone())
    }
    pub fn user_profile(&self) -> crate::generated::UserProfileResource {
        crate::generated::UserProfileResource::new(self.inner.clone())
    }
    pub fn user_setting(&self) -> crate::generated::UserSettingResource {
        crate::generated::UserSettingResource::new(self.inner.clone())
    }
    pub fn user_user(&self) -> crate::generated::UserUserResource {
        crate::generated::UserUserResource::new(self.inner.clone())
    }
    pub fn user_user_absence(&self) -> crate::generated::UserUserAbsenceResource {
        crate::generated::UserUserAbsenceResource::new(self.inner.clone())
    }
    pub fn user_user_absence_limit(&self) -> crate::generated::UserUserAbsenceLimitResource {
        crate::generated::UserUserAbsenceLimitResource::new(self.inner.clone())
    }
    pub fn user_user_attribute(&self) -> crate::generated::UserUserAttributeResource {
        crate::generated::UserUserAttributeResource::new(self.inner.clone())
    }
    pub fn vat(&self) -> crate::generated::VatResource {
        crate::generated::VatResource::new(self.inner.clone())
    }
}
