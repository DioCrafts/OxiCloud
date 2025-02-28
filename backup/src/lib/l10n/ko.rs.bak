use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("App \"%s\" can't be installed because it is not compatible with this version of ownCloud.", "현재 ownCloud 버전과 호환되지 않기 때문에 \"%s\" 앱을 설치할 수 없습니다.");
        m.insert("No app name specified", "앱 이름이 지정되지 않았습니다.");
        m.insert("Help", "도움말");
        m.insert("Personal", "개인");
        m.insert("Settings", "설정");
        m.insert("Users", "사용자");
        m.insert("Admin", "관리자");
        m.insert("Failed to upgrade \"%s\".", "\"%s\" 업그레이드에 실패했습니다.");
        m.insert("Unknown filetype", "알수없는 파일형식");
        m.insert("Invalid image", "잘못된 그림");
        m.insert("web services under your control", "내가 관리하는 웹 서비스");
        m.insert("cannot open \"%s\"", "\"%s\"을(를) 열 수 없습니다.");
        m.insert("ZIP download is turned off.", "ZIP 다운로드가 비활성화되었습니다.");
        m.insert("Files need to be downloaded one by one.", "파일을 개별적으로 다운로드해야 합니다.");
        m.insert("Back to Files", "파일로 돌아가기");
        m.insert("Selected files too large to generate zip file.", "선택한 파일들은 ZIP 파일을 생성하기에 너무 큽니다.");
        m.insert("Download the files in smaller chunks, seperately or kindly ask your administrator.", "작은 조각들 안에 들어있는 파일들을 받고자 하신다면, 나누어서 받으시거나 혹은 시스템 관리자에게 정중하게 물어보십시오");
        m.insert("No source specified when installing app", "앱을 설치할 때 소스가 지정되지 않았습니다.");
        m.insert("No href specified when installing app from http", "http에서 앱을 설치할 대 href가 지정되지 않았습니다.");
        m.insert("No path specified when installing app from local file", "로컬 파일에서 앱을 설치할 때 경로가 지정되지 않았습니다.");
        m.insert("Archives of type %s are not supported", "%s 타입 아카이브는 지원되지 않습니다.");
        m.insert("Failed to open archive when installing app", "앱을 설치할 때 아카이브를 열지 못했습니다.");
        m.insert("App does not provide an info.xml file", "앱에서 info.xml 파일이 제공되지 않았습니다.");
        m.insert("App can't be installed because of not allowed code in the App", "앱에 허용되지 않는 코드가 있어서 앱을 설치할 수 없습니다. ");
        m.insert("App can't be installed because it is not compatible with this version of ownCloud", "현재 ownCloud 버전과 호환되지 않기 때문에 앱을 설치할 수 없습니다.");
        m.insert("App can't be installed because it contains the <shipped>true</shipped> tag which is not allowed for non shipped apps", "출하되지 않은 앱에 허용되지 않는 <shipped>true</shipped> 태그를 포함하고 있기 때문에 앱을 설치할 수 없습니다.");
        m.insert("App can't be installed because the version in info.xml/version is not the same as the version reported from the app store", "info.xml/version에 포함된 버전과 앱 스토어에 보고된 버전이 같지 않아서 앱을 설치할 수 없습니다. ");
        m.insert("App directory already exists", "앱 디렉토리가 이미 존재합니다. ");
        m.insert("Can't create app folder. Please fix permissions. %s", "앱 폴더를 만들 수 없습니다. 권한을 수정하십시오. %s ");
        m.insert("Application is not enabled", "앱이 활성화되지 않았습니다");
        m.insert("Authentication error", "인증 오류");
        m.insert("Token expired. Please reload page.", "토큰이 만료되었습니다. 페이지를 새로 고치십시오.");
        m.insert("Files", "파일");
        m.insert("Text", "텍스트");
        m.insert("Images", "그림");
        m.insert("%s enter the database username.", "데이터베이스 사용자 명을 %s 에 입력해주십시오");
        m.insert("%s enter the database name.", "데이터베이스 명을 %s 에 입력해주십시오");
        m.insert("%s you may not use dots in the database name", "%s 에 적으신 데이터베이스 이름에는 점을 사용할수 없습니다");
        m.insert("MS SQL username and/or password not valid: %s", "MS SQL 사용자 이름이나 암호가 잘못되었습니다: %s");
        m.insert("You need to enter either an existing account or the administrator.", "기존 계정이나 administrator(관리자)를 입력해야 합니다.");
        m.insert("MySQL username and/or password not valid", "MySQL 사용자 이름이나 암호가 잘못되었습니다.");
        m.insert("DB Error: \"%s\"", "DB 오류: \"%s\"");
        m.insert("Offending command was: \"%s\"", "잘못된 명령: \"%s\"");
        m.insert("MySQL user '%s'@'localhost' exists already.", "MySQL 사용자 '%s'@'localhost'이(가) 이미 존재합니다.");
        m.insert("Drop this user from MySQL", "이 사용자를 MySQL에서 뺍니다.");
        m.insert("MySQL user '%s'@'%%' already exists", "MySQL 사용자 '%s'@'%%'이(가) 이미 존재합니다. ");
        m.insert("Drop this user from MySQL.", "이 사용자를 MySQL에서 뺍니다.");
        m.insert("Oracle connection could not be established", "Oracle 연결을 수립할 수 없습니다.");
        m.insert("Oracle username and/or password not valid", "Oracle 사용자 이름이나 암호가 잘못되었습니다.");
        m.insert("Offending command was: \"%s\", name: %s, password: %s", "잘못된 명령: \"%s\", 이름: %s, 암호: %s");
        m.insert("PostgreSQL username and/or password not valid", "PostgreSQL의 사용자 명 혹은 비밀번호가 잘못되었습니다");
        m.insert("Set an admin username.", "관리자 이름 설정");
        m.insert("Set an admin password.", "관리자 비밀번호 설정");
        m.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "WebDAV 인터페이스가 제대로 작동하지 않습니다. 웹 서버에서 파일 동기화를 사용할 수 있도록 설정이 제대로 되지 않은 것 같습니다.");
        m.insert("Please double check the <a href='%s'>installation guides</a>.", "<a href='%s'>설치 가이드</a>를 다시 한 번 확인하십시오.");
        m.insert("Could not find category \"%s\"", "분류 \"%s\"을(를) 찾을 수 없습니다.");
        m.insert("seconds ago", "초 전");
        m.insert("_%n minute ago_::_%n minutes ago_", "%n분 전 ");
        m.insert("_%n hour ago_::_%n hours ago_", "%n시간 전 ");
        m.insert("today", "오늘");
        m.insert("yesterday", "어제");
        m.insert("_%n day go_::_%n days ago_", "%n일 전 ");
        m.insert("last month", "지난 달");
        m.insert("_%n month ago_::_%n months ago_", "%n달 전 ");
        m.insert("last year", "작년");
        m.insert("years ago", "년 전");
        m.insert("Caused by:", "원인: ");
        m
    };
}

pub fn plural_forms() -> &'static str {
    "nplurals=1; plural=0;"
}

// Función auxiliar para registrar las traducciones con el sistema i18n
pub fn register_translations() {
    // Aquí se registrarían las traducciones con el sistema de i18n que se esté utilizando
    // Por ejemplo, con rust_i18n o similar
}