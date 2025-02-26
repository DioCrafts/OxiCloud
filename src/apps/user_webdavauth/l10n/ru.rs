use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert(
            "WebDAV Authentication",
            "Идентификация WebDAV"
        );
        m.insert(
            "Address: ",
            "Адрес:"
        );
        m.insert(
            "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.",
            "Учётные данные пользователя будут отправлены на этот адрес. Плагин проверит ответ и будет рассматривать HTTP коды 401 и 403 как неверные учётные данные, при любом другом ответе - учётные данные пользователя верны."
        );
        m
    };
}

pub const PLURAL_FORMS: &str = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";