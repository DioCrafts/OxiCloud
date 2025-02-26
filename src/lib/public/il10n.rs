//! Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
//! This file is licensed under the Affero General Public License version 3 or
//! later.
//! See the COPYING-README file.

/// Public interface of ownCloud for apps to use.
/// L10n interface

/// TODO: Description
pub trait IL10N {
    /// Translating
    ///
    /// Returns the translation. If no translation is found, `text` will be
    /// returned.
    ///
    /// # Arguments
    /// * `text` - The text we need a translation for
    /// * `parameters` - Parameters for sprintf
    fn t(&self, text: &str, parameters: Vec<String>) -> String;

    /// Translating
    ///
    /// Returns the translation. If no translation is found, `text` will be
    /// returned. %n will be replaced with the number of objects.
    ///
    /// The correct plural is determined by the plural_forms-function
    /// provided by the po file.
    ///
    /// # Arguments
    /// * `text_singular` - The string to translate for exactly one object
    /// * `text_plural` - The string to translate for n objects
    /// * `count` - Number of objects
    /// * `parameters` - Parameters for sprintf
    fn n(&self, text_singular: &str, text_plural: &str, count: i32, parameters: Vec<String>) -> String;

    /// Localization
    ///
    /// Returns the localized data.
    ///
    /// Implemented types:
    ///  - date
    ///    - Creates a date
    ///    - l10n-field: date
    ///    - params: timestamp (int/string)
    ///  - datetime
    ///    - Creates date and time
    ///    - l10n-field: datetime
    ///    - params: timestamp (int/string)
    ///  - time
    ///    - Creates a time
    ///    - l10n-field: time
    ///    - params: timestamp (int/string)
    ///
    /// # Arguments
    /// * `type_` - Type of localization
    /// * `data` - Parameters for this localization
    fn l(&self, type_: &str, data: &str) -> Option<String>;
}