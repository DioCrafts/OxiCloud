use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

pub struct L10n {
    translations: HashMap<String, String>,
    plural_forms: String,
}

impl L10n {
    pub fn new() -> Self {
        let mut translations = HashMap::new();
        translations.insert("Deletion failed".to_string(), "삭제 실패".to_string());
        translations.insert("Keep settings?".to_string(), "설정을 유지합니까?".to_string());
        translations.insert("Error".to_string(), "오류".to_string());
        translations.insert("Select groups".to_string(), "그룹 선택".to_string());
        translations.insert("Connection test succeeded".to_string(), "연결 시험 성공".to_string());
        translations.insert("Connection test failed".to_string(), "연결 시험 실패".to_string());
        translations.insert("_%s group found_::_%s groups found_".to_string(), "".to_string());
        translations.insert("_%s user found_::_%s users found_".to_string(), "".to_string());
        translations.insert("Save".to_string(), "저장".to_string());
        translations.insert("Help".to_string(), "도움말".to_string());
        translations.insert("Host".to_string(), "호스트".to_string());
        translations.insert("You can omit the protocol, except you require SSL. Then start with ldaps://".to_string(), "SSL을 사용하는 경우가 아니라면 프로토콜을 입력하지 않아도 됩니다. SSL을 사용하려면 ldaps://를 입력하십시오.".to_string());
        translations.insert("Port".to_string(), "포트".to_string());
        translations.insert("User DN".to_string(), "사용자 DN".to_string());
        translations.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.".to_string(), "바인딩 작업을 수행할 클라이언트 사용자 DN입니다. 예를 들어서 uid=agent,dc=example,dc=com입니다. 익명 접근을 허용하려면 DN과 암호를 비워 두십시오.".to_string());
        translations.insert("Password".to_string(), "암호".to_string());
        translations.insert("For anonymous access, leave DN and Password empty.".to_string(), "익명 접근을 허용하려면 DN과 암호를 비워 두십시오.".to_string());
        translations.insert("One Base DN per line".to_string(), "기본 DN을 한 줄에 하나씩 입력하십시오".to_string());
        translations.insert("You can specify Base DN for users and groups in the Advanced tab".to_string(), "고급 탭에서 사용자 및 그룹에 대한 기본 DN을 지정할 수 있습니다.".to_string());
        translations.insert("Back".to_string(), "뒤로".to_string());
        translations.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.".to_string(), "<b>경고:</b> PHP LDAP 모듈이 비활성화되어 있거나 설치되어 있지 않습니다. 백엔드를 사용할 수 없습니다. 시스템 관리자에게 설치를 요청하십시오.".to_string());
        translations.insert("Connection Settings".to_string(), "연결 설정".to_string());
        translations.insert("Configuration Active".to_string(), "구성 활성화".to_string());
        translations.insert("User Login Filter".to_string(), "사용자 로그인 필터".to_string());
        translations.insert("Backup (Replica) Host".to_string(), "백업 (복제) 포트".to_string());
        translations.insert("Backup (Replica) Port".to_string(), "백업 (복제) 포트".to_string());
        translations.insert("Disable Main Server".to_string(), "주 서버 비활성화".to_string());
        translations.insert("Case insensitve LDAP server (Windows)".to_string(), "서버에서 대소문자를 구분하지 않음 (Windows)".to_string());
        translations.insert("Turn off SSL certificate validation.".to_string(), "SSL 인증서 유효성 검사를 해제합니다.".to_string());
        translations.insert("in seconds. A change empties the cache.".to_string(), "초. 항목 변경 시 캐시가 갱신됩니다.".to_string());
        translations.insert("Directory Settings".to_string(), "디렉토리 설정".to_string());
        translations.insert("User Display Name Field".to_string(), "사용자의 표시 이름 필드".to_string());
        translations.insert("Base User Tree".to_string(), "기본 사용자 트리".to_string());
        translations.insert("One User Base DN per line".to_string(), "사용자 DN을 한 줄에 하나씩 입력하십시오".to_string());
        translations.insert("User Search Attributes".to_string(), "사용자 검색 속성".to_string());
        translations.insert("Group Display Name Field".to_string(), "그룹의 표시 이름 필드".to_string());
        translations.insert("Base Group Tree".to_string(), "기본 그룹 트리".to_string());
        translations.insert("One Group Base DN per line".to_string(), "그룹 기본 DN을 한 줄에 하나씩 입력하십시오".to_string());
        translations.insert("Group Search Attributes".to_string(), "그룹 검색 속성".to_string());
        translations.insert("Group-Member association".to_string(), "그룹-회원 연결".to_string());
        translations.insert("in bytes".to_string(), "바이트".to_string());
        translations.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.".to_string(), "사용자 이름을 사용하려면 비워 두십시오(기본값). 기타 경우 LDAP/AD 속성을 지정하십시오.".to_string());

        L10n {
            translations,
            plural_forms: "nplurals=1; plural=0;".to_string(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.translations.get(key)
    }

    pub fn plural_forms(&self) -> &str {
        &self.plural_forms
    }
}

impl fmt::Debug for L10n {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("L10n")
            .field("translations", &self.translations)
            .field("plural_forms", &self.plural_forms)
            .finish()
    }
}

// Factory function to create a shared instance
pub fn create_ko() -> Arc<L10n> {
    Arc::new(L10n::new())
}

// Singleton pattern implementation
lazy_static::lazy_static! {
    static ref KO_INSTANCE: Arc<L10n> = create_ko();
}

pub fn get() -> Arc<L10n> {
    KO_INSTANCE.clone()
}