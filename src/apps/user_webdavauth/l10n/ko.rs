use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("WebDAV Authentication", "WebDAV 인증");
        m.insert("Address: ", "주소:");
        m.insert("The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.", "ownCloud에서 이 URL로 사용자 인증 정보를 보냅니다. 이 플러그인은 응답을 확인하여 HTTP 상태 코드 401이나 403이 돌아온 경우에 잘못된 인증 정보로 간주합니다. 다른 모든 상태 코드는 올바른 인증 정보로 간주합니다.");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}