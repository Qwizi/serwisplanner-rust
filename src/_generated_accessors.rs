// Auto-generated resource accessor methods.
// Do not edit manually. Regenerate with: python3 codegen.py <yaml>

impl SerwisPlanner {
    /// `/api/account_companies`
    pub fn account_company(&self) -> crate::generated::AccountCompanyResource {
        crate::generated::AccountCompanyResource::new(self.inner.clone())
    }

    /// `/api/account_company_attributes`
    pub fn account_company_attribute(&self) -> crate::generated::AccountCompanyAttributeResource {
        crate::generated::AccountCompanyAttributeResource::new(self.inner.clone())
    }

    /// `/api/account_company_histories`
    pub fn account_company_history(&self) -> crate::generated::AccountCompanyHistoryResource {
        crate::generated::AccountCompanyHistoryResource::new(self.inner.clone())
    }

    /// `/api/account_users`
    pub fn account_user(&self) -> crate::generated::AccountUserResource {
        crate::generated::AccountUserResource::new(self.inner.clone())
    }

    /// `/api/account_user_attributes`
    pub fn account_user_attribute(&self) -> crate::generated::AccountUserAttributeResource {
        crate::generated::AccountUserAttributeResource::new(self.inner.clone())
    }

    /// `/api/account_user_histories`
    pub fn account_user_history(&self) -> crate::generated::AccountUserHistoryResource {
        crate::generated::AccountUserHistoryResource::new(self.inner.clone())
    }

    /// `/api/additional_i18ns`
    pub fn additional_i18n(&self) -> crate::generated::AdditionalI18nResource {
        crate::generated::AdditionalI18nResource::new(self.inner.clone())
    }

    /// `/api/baskets`
    pub fn basket(&self) -> crate::generated::BasketResource {
        crate::generated::BasketResource::new(self.inner.clone())
    }

    /// `/api/basket_positions`
    pub fn basket_position(&self) -> crate::generated::BasketPositionResource {
        crate::generated::BasketPositionResource::new(self.inner.clone())
    }

    /// `/api/campaigns`
    pub fn campaign(&self) -> crate::generated::CampaignResource {
        crate::generated::CampaignResource::new(self.inner.clone())
    }

    /// `/api/campaign_attributes`
    pub fn campaign_attribute(&self) -> crate::generated::CampaignAttributeResource {
        crate::generated::CampaignAttributeResource::new(self.inner.clone())
    }

    /// `/api/campaign_opportunities`
    pub fn campaign_opportunity(&self) -> crate::generated::CampaignOpportunityResource {
        crate::generated::CampaignOpportunityResource::new(self.inner.clone())
    }

    /// `/api/campaign_phases`
    pub fn campaign_phase(&self) -> crate::generated::CampaignPhaseResource {
        crate::generated::CampaignPhaseResource::new(self.inner.clone())
    }

    /// `/api/commissions`
    pub fn commission(&self) -> crate::generated::CommissionResource {
        crate::generated::CommissionResource::new(self.inner.clone())
    }

    /// `/api/commission_attributes`
    pub fn commission_attribute(&self) -> crate::generated::CommissionAttributeResource {
        crate::generated::CommissionAttributeResource::new(self.inner.clone())
    }

    /// `/api/commission_attribute_criterias`
    pub fn commission_attribute_criteria(&self) -> crate::generated::CommissionAttributeCriteriaResource {
        crate::generated::CommissionAttributeCriteriaResource::new(self.inner.clone())
    }

    /// `/api/commission_attribute_relation_actions`
    pub fn commission_attribute_relation_actions(&self) -> crate::generated::CommissionAttributeRelationActionsResource {
        crate::generated::CommissionAttributeRelationActionsResource::new(self.inner.clone())
    }

    /// `/api/commission_attribute_relations`
    pub fn commission_attribute_relations(&self) -> crate::generated::CommissionAttributeRelationsResource {
        crate::generated::CommissionAttributeRelationsResource::new(self.inner.clone())
    }

    /// `/api/commission_histories`
    pub fn commission_history(&self) -> crate::generated::CommissionHistoryResource {
        crate::generated::CommissionHistoryResource::new(self.inner.clone())
    }

    /// `/api/commission_phases`
    pub fn commission_phase(&self) -> crate::generated::CommissionPhaseResource {
        crate::generated::CommissionPhaseResource::new(self.inner.clone())
    }

    /// `/api/commission_scope_types`
    pub fn commission_scope_type(&self) -> crate::generated::CommissionScopeTypeResource {
        crate::generated::CommissionScopeTypeResource::new(self.inner.clone())
    }

    /// `/api/commission_shortcuts`
    pub fn commission_shortcut(&self) -> crate::generated::CommissionShortcutResource {
        crate::generated::CommissionShortcutResource::new(self.inner.clone())
    }

    /// `/api/commission_templates`
    pub fn commission_template(&self) -> crate::generated::CommissionTemplateResource {
        crate::generated::CommissionTemplateResource::new(self.inner.clone())
    }

    /// `/api/commissions_user_userss`
    pub fn commissions_user_users(&self) -> crate::generated::CommissionsUserUsersResource {
        crate::generated::CommissionsUserUsersResource::new(self.inner.clone())
    }

    /// `/api/delivery_cost_definitions`
    pub fn delivery_cost_definition(&self) -> crate::generated::DeliveryCostDefinitionResource {
        crate::generated::DeliveryCostDefinitionResource::new(self.inner.clone())
    }

    /// `/api/delivery_types`
    pub fn delivery_type(&self) -> crate::generated::DeliveryTypeResource {
        crate::generated::DeliveryTypeResource::new(self.inner.clone())
    }

    /// `/api/discounts`
    pub fn discount(&self) -> crate::generated::DiscountResource {
        crate::generated::DiscountResource::new(self.inner.clone())
    }

    /// `/api/discount_codes`
    pub fn discount_code(&self) -> crate::generated::DiscountCodeResource {
        crate::generated::DiscountCodeResource::new(self.inner.clone())
    }

    /// `/api/document_attributes`
    pub fn document_attribute(&self) -> crate::generated::DocumentAttributeResource {
        crate::generated::DocumentAttributeResource::new(self.inner.clone())
    }

    /// `/api/document_invoices`
    pub fn document_invoice(&self) -> crate::generated::DocumentInvoiceResource {
        crate::generated::DocumentInvoiceResource::new(self.inner.clone())
    }

    /// `/api/document_offers`
    pub fn document_offer(&self) -> crate::generated::DocumentOfferResource {
        crate::generated::DocumentOfferResource::new(self.inner.clone())
    }

    /// `/api/document_offer_attributes`
    pub fn document_offer_attribute(&self) -> crate::generated::DocumentOfferAttributeResource {
        crate::generated::DocumentOfferAttributeResource::new(self.inner.clone())
    }

    /// `/api/document_offer_questions`
    pub fn document_offer_question(&self) -> crate::generated::DocumentOfferQuestionResource {
        crate::generated::DocumentOfferQuestionResource::new(self.inner.clone())
    }

    /// `/api/document_offer_question_attributes`
    pub fn document_offer_question_attribute(&self) -> crate::generated::DocumentOfferQuestionAttributeResource {
        crate::generated::DocumentOfferQuestionAttributeResource::new(self.inner.clone())
    }

    /// `/api/document_orders`
    pub fn document_order(&self) -> crate::generated::DocumentOrderResource {
        crate::generated::DocumentOrderResource::new(self.inner.clone())
    }

    /// `/api/document_series_definitions`
    pub fn document_series_definition(&self) -> crate::generated::DocumentSeriesDefinitionResource {
        crate::generated::DocumentSeriesDefinitionResource::new(self.inner.clone())
    }

    /// `/api/document_stores`
    pub fn document_store(&self) -> crate::generated::DocumentStoreResource {
        crate::generated::DocumentStoreResource::new(self.inner.clone())
    }

    /// `/api/email_accounts`
    pub fn email_account(&self) -> crate::generated::EmailAccountResource {
        crate::generated::EmailAccountResource::new(self.inner.clone())
    }

    /// `/api/email_messages`
    pub fn email_message(&self) -> crate::generated::EmailMessageResource {
        crate::generated::EmailMessageResource::new(self.inner.clone())
    }

    /// `/api/email_message_queues`
    pub fn email_message_queue(&self) -> crate::generated::EmailMessageQueueResource {
        crate::generated::EmailMessageQueueResource::new(self.inner.clone())
    }

    /// `/api/email_templates`
    pub fn email_template(&self) -> crate::generated::EmailTemplateResource {
        crate::generated::EmailTemplateResource::new(self.inner.clone())
    }

    /// `/api/estimated_quantities`
    pub fn estimated_quantity(&self) -> crate::generated::EstimatedQuantityResource {
        crate::generated::EstimatedQuantityResource::new(self.inner.clone())
    }

    /// `/api/exchanges`
    pub fn exchange(&self) -> crate::generated::ExchangeResource {
        crate::generated::ExchangeResource::new(self.inner.clone())
    }

    /// `/api/exchange_histories`
    pub fn exchange_history(&self) -> crate::generated::ExchangeHistoryResource {
        crate::generated::ExchangeHistoryResource::new(self.inner.clone())
    }

    /// `/api/external_markers`
    pub fn external_marker(&self) -> crate::generated::ExternalMarkerResource {
        crate::generated::ExternalMarkerResource::new(self.inner.clone())
    }

    /// `/api/favorites`
    pub fn favorite(&self) -> crate::generated::FavoriteResource {
        crate::generated::FavoriteResource::new(self.inner.clone())
    }

    /// `/api/favorite_positions`
    pub fn favorite_position(&self) -> crate::generated::FavoritePositionResource {
        crate::generated::FavoritePositionResource::new(self.inner.clone())
    }

    /// `/api/file_directories`
    pub fn file_directory(&self) -> crate::generated::FileDirectoryResource {
        crate::generated::FileDirectoryResource::new(self.inner.clone())
    }

    /// `/api/file_directory_credentials`
    pub fn file_directory_credential(&self) -> crate::generated::FileDirectoryCredentialResource {
        crate::generated::FileDirectoryCredentialResource::new(self.inner.clone())
    }

    /// `/api/gantts`
    pub fn gantt(&self) -> crate::generated::GanttResource {
        crate::generated::GanttResource::new(self.inner.clone())
    }

    /// `/api/holidaies`
    pub fn holiday(&self) -> crate::generated::HolidayResource {
        crate::generated::HolidayResource::new(self.inner.clone())
    }

    /// `/api/imap_folders`
    pub fn imap_folder(&self) -> crate::generated::ImapFolderResource {
        crate::generated::ImapFolderResource::new(self.inner.clone())
    }

    /// `/api/internal_document_orders`
    pub fn internal_document_order(&self) -> crate::generated::InternalDocumentOrderResource {
        crate::generated::InternalDocumentOrderResource::new(self.inner.clone())
    }

    /// `/api/kanbans`
    pub fn kanban(&self) -> crate::generated::KanbanResource {
        crate::generated::KanbanResource::new(self.inner.clone())
    }

    /// `/api/main_menus`
    pub fn main_menu(&self) -> crate::generated::MainMenuResource {
        crate::generated::MainMenuResource::new(self.inner.clone())
    }

    /// `/api/markers`
    pub fn markers(&self) -> crate::generated::MarkersResource {
        crate::generated::MarkersResource::new(self.inner.clone())
    }

    /// `/api/measure_units`
    pub fn measure_unit(&self) -> crate::generated::MeasureUnitResource {
        crate::generated::MeasureUnitResource::new(self.inner.clone())
    }

    /// `/api/messages`
    pub fn message(&self) -> crate::generated::MessageResource {
        crate::generated::MessageResource::new(self.inner.clone())
    }

    /// `/api/message_threads`
    pub fn message_thread(&self) -> crate::generated::MessageThreadResource {
        crate::generated::MessageThreadResource::new(self.inner.clone())
    }

    /// `/api/message_thread_users`
    pub fn message_thread_user(&self) -> crate::generated::MessageThreadUserResource {
        crate::generated::MessageThreadUserResource::new(self.inner.clone())
    }

    /// `/api/odbc_reports`
    pub fn odbc_report(&self) -> crate::generated::OdbcReportResource {
        crate::generated::OdbcReportResource::new(self.inner.clone())
    }

    /// `/api/payment_types`
    pub fn payment_type(&self) -> crate::generated::PaymentTypeResource {
        crate::generated::PaymentTypeResource::new(self.inner.clone())
    }

    /// `/api/place_attributes`
    pub fn place_attribute(&self) -> crate::generated::PlaceAttributeResource {
        crate::generated::PlaceAttributeResource::new(self.inner.clone())
    }

    /// `/api/places`
    pub fn places(&self) -> crate::generated::PlacesResource {
        crate::generated::PlacesResource::new(self.inner.clone())
    }

    /// `/api/price_individuals`
    pub fn price_individual(&self) -> crate::generated::PriceIndividualResource {
        crate::generated::PriceIndividualResource::new(self.inner.clone())
    }

    /// `/api/price_product_groups`
    pub fn price_product_group(&self) -> crate::generated::PriceProductGroupResource {
        crate::generated::PriceProductGroupResource::new(self.inner.clone())
    }

    /// `/api/price_types`
    pub fn price_types(&self) -> crate::generated::PriceTypesResource {
        crate::generated::PriceTypesResource::new(self.inner.clone())
    }

    /// `/api/products`
    pub fn product(&self) -> crate::generated::ProductResource {
        crate::generated::ProductResource::new(self.inner.clone())
    }

    /// `/api/product_attributes`
    pub fn product_attribute(&self) -> crate::generated::ProductAttributeResource {
        crate::generated::ProductAttributeResource::new(self.inner.clone())
    }

    /// `/api/product_categories`
    pub fn product_category(&self) -> crate::generated::ProductCategoryResource {
        crate::generated::ProductCategoryResource::new(self.inner.clone())
    }

    /// `/api/product_histories`
    pub fn product_history(&self) -> crate::generated::ProductHistoryResource {
        crate::generated::ProductHistoryResource::new(self.inner.clone())
    }

    /// `/api/product_templates`
    pub fn product_template(&self) -> crate::generated::ProductTemplateResource {
        crate::generated::ProductTemplateResource::new(self.inner.clone())
    }

    /// `/api/product_to_product_categories`
    pub fn product_to_product_category(&self) -> crate::generated::ProductToProductCategoryResource {
        crate::generated::ProductToProductCategoryResource::new(self.inner.clone())
    }

    /// `/api/profile_settings`
    pub fn profile_setting(&self) -> crate::generated::ProfileSettingResource {
        crate::generated::ProfileSettingResource::new(self.inner.clone())
    }

    /// `/api/serviced_products`
    pub fn serviced_product(&self) -> crate::generated::ServicedProductResource {
        crate::generated::ServicedProductResource::new(self.inner.clone())
    }

    /// `/api/serviced_product_attributes`
    pub fn serviced_product_attribute(&self) -> crate::generated::ServicedProductAttributeResource {
        crate::generated::ServicedProductAttributeResource::new(self.inner.clone())
    }

    /// `/api/serviced_product_histories`
    pub fn serviced_product_history(&self) -> crate::generated::ServicedProductHistoryResource {
        crate::generated::ServicedProductHistoryResource::new(self.inner.clone())
    }

    /// `/api/settings`
    pub fn setting(&self) -> crate::generated::SettingResource {
        crate::generated::SettingResource::new(self.inner.clone())
    }

    /// `/api/setting_histories`
    pub fn setting_history(&self) -> crate::generated::SettingHistoryResource {
        crate::generated::SettingHistoryResource::new(self.inner.clone())
    }

    /// `/api/storehouses`
    pub fn storehouse(&self) -> crate::generated::StorehouseResource {
        crate::generated::StorehouseResource::new(self.inner.clone())
    }

    /// `/api/targets`
    pub fn target(&self) -> crate::generated::TargetResource {
        crate::generated::TargetResource::new(self.inner.clone())
    }

    /// `/api/tasks`
    pub fn task(&self) -> crate::generated::TaskResource {
        crate::generated::TaskResource::new(self.inner.clone())
    }

    /// `/api/task_histories`
    pub fn task_history(&self) -> crate::generated::TaskHistoryResource {
        crate::generated::TaskHistoryResource::new(self.inner.clone())
    }

    /// `/api/task_types`
    pub fn task_type(&self) -> crate::generated::TaskTypeResource {
        crate::generated::TaskTypeResource::new(self.inner.clone())
    }

    /// `/api/user_geolocations`
    pub fn user_geolocation(&self) -> crate::generated::UserGeolocationResource {
        crate::generated::UserGeolocationResource::new(self.inner.clone())
    }

    /// `/api/user_profiles`
    pub fn user_profile(&self) -> crate::generated::UserProfileResource {
        crate::generated::UserProfileResource::new(self.inner.clone())
    }

    /// `/api/user_settings`
    pub fn user_setting(&self) -> crate::generated::UserSettingResource {
        crate::generated::UserSettingResource::new(self.inner.clone())
    }

    /// `/api/user_users`
    pub fn user_user(&self) -> crate::generated::UserUserResource {
        crate::generated::UserUserResource::new(self.inner.clone())
    }

    /// `/api/user_user_absences`
    pub fn user_user_absence(&self) -> crate::generated::UserUserAbsenceResource {
        crate::generated::UserUserAbsenceResource::new(self.inner.clone())
    }

    /// `/api/user_user_absence_limits`
    pub fn user_user_absence_limit(&self) -> crate::generated::UserUserAbsenceLimitResource {
        crate::generated::UserUserAbsenceLimitResource::new(self.inner.clone())
    }

    /// `/api/user_user_attributes`
    pub fn user_user_attribute(&self) -> crate::generated::UserUserAttributeResource {
        crate::generated::UserUserAttributeResource::new(self.inner.clone())
    }

    /// `/api/user_user_histories`
    pub fn user_user_history(&self) -> crate::generated::UserUserHistoryResource {
        crate::generated::UserUserHistoryResource::new(self.inner.clone())
    }

    /// `/api/vats`
    pub fn vat(&self) -> crate::generated::VatResource {
        crate::generated::VatResource::new(self.inner.clone())
    }

}
