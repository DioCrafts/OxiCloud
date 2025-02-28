use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("WebDAV Authentication", "WebDAV 认证");
        m.insert("Address: ", "地址：");
        m.insert(
            "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.",
            "用户的身份将会被发送到此 URL。这个插件检查返回值并且将 HTTP 状态编码 401 和 403 解释为非法身份，其他所有返回值为合法身份。"
        );
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}