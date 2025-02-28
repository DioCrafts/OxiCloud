/**
 * Copyright (c) 2012 Bart Visscher <bartv@thisnet.nl>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use std::collections::HashMap;
use std::fmt;

/// Type for plural form functions that determine which translation to use based on count
type PluralFormFn = fn(u64) -> usize;

/// The localization provider trait that must be implemented by any struct
/// that wants to provide translations to the L10nString
pub trait L10nProvider {
    /// Returns all available translations as a map
    fn get_translations(&self) -> &HashMap<String, Vec<String>>;
    
    /// Returns the function that determines which plural form to use
    fn get_plural_form_function(&self) -> PluralFormFn;
}

/// Represents a localizable string with parameters and count for pluralization
pub struct L10nString<'a, T: L10nProvider> {
    /// Reference to the localization provider
    l10n: &'a T,
    
    /// The text key to be translated
    text: String,
    
    /// The parameters to be inserted into the translated string
    parameters: Vec<String>,
    
    /// The count for pluralization
    count: u64,
}

impl<'a, T: L10nProvider> L10nString<'a, T> {
    /// Creates a new localizable string with the given parameters
    pub fn new(l10n: &'a T, text: String, parameters: Vec<String>, count: u64) -> Self {
        Self {
            l10n,
            text,
            parameters,
            count,
        }
    }
}

impl<'a, T: L10nProvider> fmt::Display for L10nString<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let translations = self.l10n.get_translations();
        
        let mut text = self.text.clone();
        if let Some(translation) = translations.get(&self.text) {
            if translation.len() > 1 {
                // Handle plural forms
                let plural_fn = self.l10n.get_plural_form_function();
                let id = plural_fn(self.count);
                if let Some(plural_text) = translation.get(id) {
                    text = plural_text.clone();
                }
            } else if let Some(singular_text) = translation.first() {
                text = singular_text.clone();
            }
        }
        
        // Replace %n with count (equivalent to PHP's str_replace)
        text = text.replace("%n", &self.count.to_string());
        
        // Format the string with parameters (equivalent to PHP's vsprintf)
        // Simplified implementation assuming {0}, {1}, etc. format
        let mut result = text;
        for (i, param) in self.parameters.iter().enumerate() {
            result = result.replace(&format!("{{{}}}", i), param);
        }
        
        write!(f, "{}", result)
    }
}