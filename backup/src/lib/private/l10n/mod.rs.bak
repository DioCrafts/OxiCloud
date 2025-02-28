// Módulos generados automáticamente

pub mod factory;
pub mod string;

// Contenido fusionado desde src/lib/private/l10n.rs
//! ownCloud
//!
//! Originally by Frank Karlitschek, Jakob Sack
//! Copyright 2012 Frank Karlitschek frank@owncloud.org
//! Copyright 2013 Jakob Sack
//!
//! This library is free software; you can redistribute it and/or
//! modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
//! License as published by the Free Software Foundation; either
//! version 3 of the License, or any later version.
//!
//! This library is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//! GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//!
//! You should have received a copy of the GNU Affero General Public
//! License along with this library. If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Local, TimeZone};
use lazy_static::lazy_static;
use regex::Regex;
use log::warn;

/// Trait for localization functionality
pub trait IL10N {
    fn t(&self, text: &str, parameters: Vec<String>) -> L10NString;
    fn n(&self, text_singular: &str, text_plural: &str, count: i64, parameters: Vec<String>) -> L10NString;
    fn get_translations(&self) -> HashMap<String, String>;
    fn get_plural_form_string(&self) -> String;
    fn get_localizations(&self) -> HashMap<String, String>;
    fn l(&self, type_: &str, data: &str) -> Option<String>;
}

lazy_static! {
    static ref CACHE: RwLock<HashMap<String, L10NCache>> = RwLock::new(HashMap::new());
    static ref LANGUAGE: RwLock<String> = RwLock::new(String::new());
}

#[derive(Clone)]
struct L10NCache {
    translations: HashMap<String, String>,
    localizations: HashMap<String, String>,
}

pub struct L10NString {
    l10n: L10N,
    text: String,
    parameters: Vec<String>,
    count: Option<i64>,
}

impl L10NString {
    pub fn new(l10n: L10N, text: &str, parameters: Vec<String>, count: Option<i64>) -> Self {
        Self {
            l10n,
            text: text.to_string(),
            parameters,
            count,
        }
    }
    
    pub fn to_string(&self) -> String {
        let mut text = self.text.clone();
        
        if let Some(count) = self.count {
            text = text.replace("%n", &count.to_string());
        }
        
        if self.parameters.is_empty() {
            return text;
        }
        
        // Simple sprintf-like replacement
        let mut result = text;
        for (i, param) in self.parameters.iter().enumerate() {
            result = result.replace(&format!("%{}", i+1), param);
        }
        result
    }
}

impl std::fmt::Display for L10NString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Clone)]
pub struct L10N {
    app: String,
    lang: Option<String>,
    translations: HashMap<String, String>,
    plural_form_string: String,
    localizations: HashMap<String, String>,
    initialized: bool,
}

impl L10N {
    /// Get an L10N instance
    pub fn get(app: &str, lang: Option<&str>) -> Self {
        if lang.is_none() {
            // In PHP this would call OC::$server->getL10N($app)
            // For now, we'll just create a new instance
            Self::new(app, None)
        } else {
            Self::new(app, lang.map(String::from))
        }
    }

    /// Create a new L10N instance
    pub fn new(app: &str, lang: Option<String>) -> Self {
        let mut localizations = HashMap::new();
        localizations.insert("jsdate".to_string(), "dd.mm.yy".to_string());
        localizations.insert("date".to_string(), "%d.%m.%Y".to_string());
        localizations.insert("datetime".to_string(), "%d.%m.%Y %H:%M:%S".to_string());
        localizations.insert("time".to_string(), "%H:%M:%S".to_string());
        localizations.insert("firstday".to_string(), "0".to_string());

        Self {
            app: app.to_string(),
            lang,
            translations: HashMap::new(),
            plural_form_string: "nplurals=2; plural=(n != 1);".to_string(),
            localizations,
            initialized: false,
        }
    }

    /// Load translations from a file
    pub fn load(&mut self, trans_file: &str) -> Result<(), std::io::Error> {
        // In Rust we can't just include PHP files dynamically
        // Instead, we'd need to parse the file or use a different format
        // For this example, we'll assume we have a function to parse PHP translation files
        if let Ok((translations, plural_forms)) = self.parse_translation_file(trans_file) {
            self.translations = translations;
            if !plural_forms.is_empty() {
                self.plural_form_string = plural_forms;
            }
        }
        Ok(())
    }

    // Mock method to parse PHP translation files
    fn parse_translation_file(&self, file_path: &str) -> Result<(HashMap<String, String>, String), std::io::Error> {
        // This would actually parse a PHP file with translations
        // For now, we'll just return empty structures
        Ok((HashMap::new(), String::new()))
    }

    fn init(&mut self) {
        if self.initialized {
            return;
        }

        let app = self.app.clone();
        if app == "true" {
            self.initialized = true;
            return;
        }

        // Find the right language
        let lang = match &self.lang {
            Some(l) if !l.is_empty() => l.clone(),
            _ => Self::find_language(Some(&app)),
        };

        // Use cache if possible
        let cache_key = format!("{}::{}", app, lang);
        let cache_read = CACHE.read().unwrap();
        
        if let Some(cached) = cache_read.get(&cache_key) {
            self.translations = cached.translations.clone();
            self.localizations = cached.localizations.clone();
            self.initialized = true;
            return;
        }
        drop(cache_read); // Release lock

        // Find i18n directory and load translations
        let i18n_dir = Self::find_i18n_dir(&app);
        let trans_file_path = format!("{}{}.php", i18n_dir, lang);
        
        if Path::new(&trans_file_path).exists() {
            if let Ok((translations, plural_forms)) = self.parse_translation_file(&trans_file_path) {
                self.translations = translations;
                if !plural_forms.is_empty() {
                    self.plural_form_string = plural_forms;
                }
                
                // TODO: Add theme merging here similar to PHP
                // if let Some(theme) = OC_Config::get_value("theme") {
                //    let theme_file = format!("{}/themes/{}", OC::$SERVERROOT, theme...);
                //    // parse and merge translations
                // }
            }
        }

        // Load core localizations
        let core_l10n_path = format!("{}/core/l10n/l10n-{}.php", "SERVERROOT", lang);
        if Path::new(&core_l10n_path).exists() {
            // Parse and merge localizations
            // For now, we'll keep the default ones
        }

        // Save to cache
        let mut cache_write = CACHE.write().unwrap();
        cache_write.insert(cache_key, L10NCache {
            translations: self.translations.clone(),
            localizations: self.localizations.clone(),
        });

        self.initialized = true;
    }

    /// Find plural form function (simplified in Rust)
    fn get_plural_form(&self, n: i64) -> usize {
        // For simplicity, we'll implement just the English plural rule
        // In a real implementation, this would parse the plural form string
        if n == 1 { 0 } else { 1 }
    }

    /// Find the i18n directory for an app
    fn find_i18n_dir(app: &str) -> String {
        if app.is_empty() {
            return "SERVERROOT/core/l10n/".to_string();
        }
        
        // Check if app is in app folder
        let app_path = format!("APPPATH/{}/l10n/", app);
        if Path::new(&app_path).exists() {
            return app_path;
        }
        
        format!("SERVERROOT/{}/l10n/", app)
    }

    /// Find the best language
    pub fn find_language(app: Option<&str>) -> String {
        // Check if we have cached language
        {
            let language_read = LANGUAGE.read().unwrap();
            if !language_read.is_empty() && app.is_none() {
                return language_read.clone();
            }
        }

        // Check user preferences
        // TODO: Implement user preferences check
        // if OC_User::getUser() && OC_Preferences::getValue() { ... }

        // Check default language setting
        // TODO: Implement config check
        // let default_language = OC_Config::getValue("default_language", false);
        // if default_language != false { return default_language; }

        // Check HTTP_ACCEPT_LANGUAGE
        if let Ok(accept_language) = std::env::var("HTTP_ACCEPT_LANGUAGE") {
            let available = match app {
                Some(a) => Self::find_available_languages(Some(a)),
                None => Self::find_available_languages(None),
            };
            
            let preferences: Vec<&str> = accept_language.to_lowercase().split(',').map(|s| s.trim()).collect();
            
            for preference in preferences {
                let preferred_language = preference.split(';').next().unwrap_or("").replace('-', "_");
                
                // Exact match
                for available_language in &available {
                    if preferred_language == available_language.to_lowercase() {
                        if app.is_none() {
                            let mut language_write = LANGUAGE.write().unwrap();
                            *language_write = available_language.clone();
                        }
                        return available_language.clone();
                    }
                }
                
                // Language code match (en_US -> en)
                for available_language in &available {
                    if preferred_language.len() >= 2 && 
                       preferred_language[0..2] == available_language.to_lowercase() {
                        if app.is_none() {
                            let mut language_write = LANGUAGE.write().unwrap();
                            *language_write = available_language.clone();
                        }
                        return available_language.clone();
                    }
                }
            }
        }

        // Default to English
        "en".to_string()
    }

    /// Find all available languages for an app
    pub fn find_available_languages(app: Option<&str>) -> Vec<String> {
        let mut available = vec!["en".to_string()]; // English is always available
        let dir = Self::find_i18n_dir(app.unwrap_or(""));
        
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    if file_name.ends_with(".php") && !file_name.starts_with("l10n") {
                        if let Some(lang) = file_name.strip_suffix(".php") {
                            available.push(lang.to_string());
                        }
                    }
                }
            }
        }
        
        available
    }

    /// Check if a language exists for an app
    pub fn language_exists(app: &str, lang: &str) -> bool {
        if lang == "en" {
            return true; // English is always available
        }
        
        let dir = Self::find_i18n_dir(app);
        let file_path = format!("{}{}.php", dir, lang);
        Path::new(&file_path).exists()
    }

    /// Choose a language from a map of texts
    pub fn select_language(texts: HashMap<&str, &str>) -> String {
        let lang = Self::find_language(Some(texts.keys().cloned().collect::<Vec<&str>>().as_slice()));
        texts.get(lang.as_str()).unwrap_or(&"").to_string()
    }
}

impl IL10N for L10N {
    fn t(&self, text: &str, parameters: Vec<String>) -> L10NString {
        L10NString::new(self.clone(), text, parameters, None)
    }

    fn n(&self, text_singular: &str, text_plural: &str, count: i64, parameters: Vec<String>) -> L10NString {
        let mut l10n = self.clone();
        l10n.init();
        
        let identifier = format!("_{}__{}_{}", text_singular, text_plural, count);
        
        if l10n.translations.contains_key(&identifier) {
            return L10NString::new(l10n, &identifier, parameters, Some(count));
        } else {
            if count == 1 {
                return L10NString::new(l10n, text_singular, parameters, Some(count));
            } else {
                return L10NString::new(l10n, text_plural, parameters, Some(count));
            }
        }
    }

    fn get_translations(&self) -> HashMap<String, String> {
        let mut l10n = self.clone();
        l10n.init();
        l10n.translations
    }

    fn get_plural_form_string(&self) -> String {
        let mut l10n = self.clone();
        l10n.init();
        l10n.plural_form_string
    }

    fn get_localizations(&self) -> HashMap<String, String> {
        let mut l10n = self.clone();
        l10n.init();
        l10n.localizations
    }

    fn l(&self, type_: &str, data: &str) -> Option<String> {
        let mut l10n = self.clone();
        l10n.init();
        
        match type_ {
            "date" | "datetime" | "time" => {
                let timestamp = if data.parse::<i64>().is_ok() {
                    data.parse::<i64>().unwrap()
                } else {
                    // Try to parse as a date string
                    match DateTime::parse_from_rfc3339(data) {
                        Ok(dt) => dt.timestamp(),
                        Err(_) => {
                            return None;
                        }
                    }
                };
                
                let dt = Local.timestamp_opt(timestamp, 0).single()?;
                let format = l10n.localizations.get(type_)?;
                
                // This is a simplified version - would need a proper strftime implementation
                Some(dt.format(format).to_string())
            },
            "firstday" | "jsdate" => {
                l10n.localizations.get(type_).cloned()
            },
            _ => None
        }
    }
}