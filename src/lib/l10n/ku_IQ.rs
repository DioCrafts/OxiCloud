use rust_fluent::types::{FluentNumber, FluentValue};
use rust_fluent::FluentBundle;
use std::collections::HashMap;

// L10n support for Kurdish (IQ) - ku_IQ
pub struct KuIq;

impl KuIq {
    pub fn get_translations() -> HashMap<String, String> {
        let mut translations = HashMap::new();
        translations.insert("Help".to_string(), "یارمەتی".to_string());
        translations.insert("Settings".to_string(), "ده‌ستكاری".to_string());
        translations.insert("Users".to_string(), "به‌كارهێنه‌ر".to_string());
        translations.insert("Admin".to_string(), "به‌ڕێوه‌به‌ری سه‌ره‌كی".to_string());
        translations.insert("web services under your control".to_string(), "ڕاژه‌ی وێب له‌ژێر چاودێریت دایه".to_string());
        translations
    }

    pub fn get_plural_forms() -> &'static str {
        "nplurals=2; plural=(n != 1);"
    }

    pub fn setup_time_ago_messages(bundle: &mut FluentBundle) -> Result<(), rust_fluent::FluentError> {
        // Define the minute ago plurals
        bundle.add_resource(
            r#"
            minute-ago = { $n ->
                [1] minute ago
                *[other] { $n } minutes ago
            }
            "#,
        )?;

        // Define the hour ago plurals
        bundle.add_resource(
            r#"
            hour-ago = { $n ->
                [1] hour ago
                *[other] { $n } hours ago
            }
            "#,
        )?;

        // Define the day ago plurals
        bundle.add_resource(
            r#"
            day-ago = { $n ->
                [1] day ago
                *[other] { $n } days ago
            }
            "#,
        )?;

        // Define the month ago plurals
        bundle.add_resource(
            r#"
            month-ago = { $n ->
                [1] month ago
                *[other] { $n } months ago
            }
            "#,
        )?;

        Ok(())
    }

    pub fn format_time_ago(bundle: &FluentBundle, id: &str, n: i64) -> String {
        let mut args = HashMap::new();
        args.insert("n".to_string(), FluentValue::Number(FluentNumber::new(n as f64)));
        
        match bundle.format_pattern(bundle.get_message(id).unwrap().value(), Some(&args), &mut vec![]) {
            Ok(value) => value.to_string(),
            Err(_) => format!("{} {}", n, id),
        }
    }
}