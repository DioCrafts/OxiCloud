use std::sync::Arc;

use crate::domain::services::i18n_service::{I18nResult, I18nService, Locale};

/// Service for i18n operations
pub struct I18nApplicationService {
    i18n_service: Arc<dyn I18nService>,
}

impl I18nApplicationService {
    /// Creates a dummy service for testing
    pub fn dummy() -> Self {
        struct DummyI18nService;

        #[async_trait::async_trait]
        impl I18nService for DummyI18nService {
            async fn translate(&self, _key: &str, _locale: Locale) -> I18nResult<String> {
                Ok("DUMMY_TRANSLATION".to_string())
            }

            async fn load_translations(&self, _locale: Locale) -> I18nResult<()> {
                Ok(())
            }

            async fn available_locales(&self) -> Vec<Locale> {
                vec![Locale::English, Locale::Spanish]
            }

            async fn is_supported(&self, _locale: Locale) -> bool {
                true
            }
        }

        Self {
            i18n_service: Arc::new(DummyI18nService),
        }
    }

    /// Creates a new i18n application service
    pub fn new(i18n_service: Arc<dyn I18nService>) -> Self {
        Self { i18n_service }
    }

    /// Get a translation for a key and locale
    pub async fn translate(&self, key: &str, locale: Option<Locale>) -> I18nResult<String> {
        let locale = locale.unwrap_or_default();
        self.i18n_service.translate(key, locale).await
    }

    /// Load translations for a locale
    pub async fn load_translations(&self, locale: Locale) -> I18nResult<()> {
        self.i18n_service.load_translations(locale).await
    }

    /// Load translations for all available locales
    pub async fn load_all_translations(&self) -> Vec<(Locale, I18nResult<()>)> {
        let locales = self.i18n_service.available_locales().await;
        let mut results = Vec::new();

        for locale in locales {
            let result = self.i18n_service.load_translations(locale).await;
            results.push((locale, result));
        }

        results
    }

    /// Get available locales
    pub async fn available_locales(&self) -> Vec<Locale> {
        self.i18n_service.available_locales().await
    }

    /// Check if a locale is supported
    pub async fn is_supported(&self, locale: Locale) -> bool {
        self.i18n_service.is_supported(locale).await
    }
}
