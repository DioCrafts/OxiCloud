use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("WebDAV Authentication", "WebDAV 認証");
        m.insert("Address: ", "アドレス:");
        m.insert(
            "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.",
            "ユーザーの権限情報をこのアドレスに送信します。このプラグインは応答をチェックし、HTTP状態コードが 401 と 403 の場合は無効な資格情報とし、他の応答はすべて有効な資格情報として処理します。"
        );
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}