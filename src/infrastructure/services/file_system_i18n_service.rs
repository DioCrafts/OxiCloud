use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;
use async_trait::async_trait;
use serde_json::Value;
use tokio::fs;

use crate::domain::services::i18n_service::{I18nService, I18nError, I18nResult, Locale};

/// File system implementation of the I18nService
pub struct FileSystemI18nService {
    /// Base directory containing translation files
    translations_dir: PathBuf,
    
    /// Cached translations (locale code -> JSON data)
    cache: RwLock<HashMap<Locale, Value>>,
}

impl FileSystemI18nService {
    /// Create a dummy service for testing
    pub fn dummy() -> Self {
        Self {
            translations_dir: PathBuf::from("/tmp/dummy_translations"),
            cache: RwLock::new(HashMap::new()),
        }
    }

    /// Creates a new file system i18n service
    pub fn new(translations_dir: PathBuf) -> Self {
        Self {
            translations_dir,
            cache: RwLock::new(HashMap::new()),
        }
    }
    
    /// Get translation file path for a locale
    fn get_locale_file_path(&self, locale: Locale) -> PathBuf {
        self.translations_dir.join(format!("{}.json", locale.as_str()))
    }
    
    /// Get a nested key from JSON data
    fn get_nested_value(&self, data: &Value, key: &str) -> Option<String> {
        let parts: Vec<&str> = key.split('.').collect();
        let mut current = data;
        
        for part in &parts[0..parts.len() - 1] {
            if let Some(next) = current.get(part) {
                current = next;
            } else {
                return None;
            }
        }
        
        if let Some(last_part) = parts.last()
            && let Some(value) = current.get(last_part)
                && value.is_string() {
                    return value.as_str().map(|s| s.to_string());
                }
        
        None
    }
}

#[async_trait]
impl I18nService for FileSystemI18nService {
    async fn translate(&self, key: &str, locale: Locale) -> I18nResult<String> {
        // Check if translations are cached
        {
            let cache = self.cache.read().unwrap();
            if let Some(translations) = cache.get(&locale) {
                if let Some(value) = self.get_nested_value(translations, key) {
                    return Ok(value);
                }
                
                // Try to use English as fallback if we couldn't find the key
                if locale != Locale::English
                    && let Some(english_translations) = cache.get(&Locale::English)
                        && let Some(value) = self.get_nested_value(english_translations, key) {
                            return Ok(value);
                        }
                
                return Err(I18nError::KeyNotFound(key.to_string()));
            }
        }
        
        // If not cached, load translations and try again
        self.load_translations(locale).await?;
        
        {
            let cache = self.cache.read().unwrap();
            if let Some(translations) = cache.get(&locale) {
                if let Some(value) = self.get_nested_value(translations, key) {
                    return Ok(value);
                }
                
                // Try to use English as fallback
                if locale != Locale::English
                    && let Some(english_translations) = cache.get(&Locale::English)
                        && let Some(value) = self.get_nested_value(english_translations, key) {
                            return Ok(value);
                        }
            }
        }
        
        Err(I18nError::KeyNotFound(key.to_string()))
    }
    
    async fn load_translations(&self, locale: Locale) -> I18nResult<()> {
        let file_path = self.get_locale_file_path(locale);
        tracing::info!("Loading translations for locale {} from {:?}", locale.as_str(), file_path);
        
        // Check if file exists
        if !file_path.exists() {
            return Err(I18nError::InvalidLocale(locale.as_str().to_string()));
        }
        
        // Read and parse file
        let content = fs::read_to_string(&file_path)
            .await
            .map_err(|e| I18nError::LoadError(format!("Failed to read translation file: {}", e)))?;
            
        let translations: Value = serde_json::from_str(&content)
            .map_err(|e| I18nError::LoadError(format!("Failed to parse translation file: {}", e)))?;
            
        // Update cache
        {
            let mut cache = self.cache.write().unwrap();
            cache.insert(locale, translations);
        }
        
        tracing::info!("Translations loaded for locale {}", locale.as_str());
        Ok(())
    }
    
    async fn available_locales(&self) -> Vec<Locale> {
        vec![Locale::English, Locale::Spanish, Locale::French, Locale::German, Locale::Portuguese]
    }
    
    async fn is_supported(&self, locale: Locale) -> bool {
        let file_path = self.get_locale_file_path(locale);
        file_path.exists()
    }
}