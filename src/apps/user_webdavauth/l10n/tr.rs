use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("WebDAV Authentication", "WebDAV Kimlik doğrulaması");
        m.insert("Address: ", "Adres:");
        m.insert("The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.", "Kullanıcı kimlik bilgileri bu adrese gönderilecek. Bu eklenti yanıtı kontrol edecek ve 401 ile 403 HTTP durum kodlarını geçersiz kimlik bilgileri olarak, diğer yanıtları ise doğru kimlik bilgileri olarak algılayacaktır.");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n > 1);";
}