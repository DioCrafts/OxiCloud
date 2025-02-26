use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("The password is wrong. Try again.", "비밀번호가 틀립니다. 다시 입력해주세요.");
        m.insert("Password", "암호");
        m.insert("Sorry, this link doesn't seem to work anymore.", "죄송합니다만 이 링크는 더이상 작동되지 않습니다.");
        m.insert("Reasons might be:", "이유는 다음과 같을 수 있습니다:");
        m.insert("the item was removed", "이 항목은 삭제되었습니다");
        m.insert("the link expired", "링크가 만료되었습니다");
        m.insert("sharing is disabled", "공유가 비활성되었습니다");
        m.insert("For more info, please ask the person who sent this link.", "더 자세한 설명은 링크를 보내신 분에게 여쭤보십시오");
        m.insert("%s shared the folder %s with you", "%s 님이 폴더 %s을(를) 공유하였습니다");
        m.insert("%s shared the file %s with you", "%s 님이 파일 %s을(를) 공유하였습니다");
        m.insert("Download", "다운로드");
        m.insert("Upload", "업로드");
        m.insert("Cancel upload", "업로드 취소");
        m.insert("No preview available for", "다음 항목을 미리 볼 수 없음:");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}