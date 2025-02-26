use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Password successfully changed.", "암호가 성공적으로 변경되었습니다");
    translations.insert("Could not change the password. Maybe the old password was not correct.", "암호를 변경할수 없습니다. 아마도 예전 암호가 정확하지 않은것 같습니다.");
    translations.insert("Private key password successfully updated.", "개인키 암호가 성공적으로 업데이트 됨.");
    translations.insert("Saving...", "저장 중...");
    translations.insert("personal settings", "개인 설정");
    translations.insert("Encryption", "암호화");
    translations.insert("Recovery key password", "키 비밀번호 복구");
    translations.insert("Change recovery key password:", "복구 키 비밀번호 변경");
    translations.insert("Old Recovery key password", "예전 복구 키 비밀번호");
    translations.insert("New Recovery key password", "새 복구 키 비밀번호");
    translations.insert("Change Password", "암호 변경");
    translations.insert("Old log-in password", "예전 로그인 암호");
    translations.insert("Current log-in password", "현재 로그인 암호");
    translations.insert("Update Private Key Password", "개인 키 암호 업데이트");
    translations.insert("File recovery settings updated", "파일 복구 설정 업데이트됨");
    translations.insert("Could not update file recovery", "파일 복구를 업데이트 할수 없습니다");
    translations
});

pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";