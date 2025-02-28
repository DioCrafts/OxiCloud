use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not revert: %s", "되돌릴 수 없습니다: %s");
        m.insert("Versions", "버전");
        m.insert("Failed to revert {file} to revision {timestamp}.", "{timestamp} 판의 {file}로 돌리는데 실패했습니다.");
        m.insert("More versions...", "더 많은 버전들...");
        m.insert("No other versions available", "다른 버전을 사용할수 없습니다");
        m.insert("Restore", "복원");
        m
    };
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}