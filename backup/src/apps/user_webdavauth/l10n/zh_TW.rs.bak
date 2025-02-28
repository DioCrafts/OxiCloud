use std::collections::HashMap;
use rust_i18n::i18n;

i18n!("zh_TW");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translations() {
        let translations = zh_tw_translations();
        assert_eq!(translations.get("WebDAV Authentication"), Some(&"WebDAV 認證".to_string()));
        assert_eq!(translations.get("Address: "), Some(&"位址:".to_string()));
    }
}

pub fn zh_tw_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    translations.insert("WebDAV Authentication".to_string(), "WebDAV 認證".to_string());
    translations.insert("Address: ".to_string(), "位址:".to_string());
    translations.insert(
        "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.".to_string(),
        "使用者憑證將會被傳送到此位址。此外掛程式將會檢查回應，HTTP狀態碼 401與403將會被理解為無效憑證，而所有其他的回應將會被理解為有效憑證。".to_string()
    );
    translations
}

pub fn get_plural_form() -> &'static str {
    "nplurals=1; plural=0;"
}