use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Access granted", "접근 허가됨");
    m.insert("Error configuring Dropbox storage", "Dropbox 저장소 설정 오류");
    m.insert("Grant access", "접근 권한 부여");
    m.insert("Please provide a valid Dropbox app key and secret.", "올바른 Dropbox 앱 키와 암호를 입력하십시오.");
    m.insert("Error configuring Google Drive storage", "Google 드라이브 저장소 설정 오류");
    m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>경고:</b> \"smbclient\"가 설치되지 않았습니다. CIFS/SMB 공유 자원에 연결할 수 없습니다. 시스템 관리자에게 설치를 요청하십시오.");
    m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>경고:</b> PHP FTP 지원이 비활성화되어 있거나 설치되지 않았습니다. FTP 공유를 마운트할 수 없습니다. 시스템 관리자에게 설치를 요청하십시오.");
    m.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>경고:</b> PHP Curl 지원이 비활성화되어 있거나 설치되지 않았습니다. 다른 ownCloud, WebDAV, Google 드라이브 공유를 마운트할 수 없습니다. 시스템 관리자에게 설치를 요청하십시오.");
    m.insert("External Storage", "외부 저장소");
    m.insert("Folder name", "폴더 이름");
    m.insert("External storage", "외부 저장소");
    m.insert("Configuration", "설정");
    m.insert("Options", "옵션");
    m.insert("Applicable", "적용 가능");
    m.insert("Add storage", "저장소 추가");
    m.insert("None set", "설정되지 않음");
    m.insert("All Users", "모든 사용자");
    m.insert("Groups", "그룹");
    m.insert("Users", "사용자");
    m.insert("Delete", "삭제");
    m.insert("Enable User External Storage", "사용자 외부 저장소 사용");
    m.insert("Allow users to mount their own external storage", "사용자별 외부 저장소 마운트 허용");
    m.insert("SSL root certificates", "SSL 루트 인증서");
    m.insert("Import Root Certificate", "루트 인증서 가져오기");
    m
});

pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";