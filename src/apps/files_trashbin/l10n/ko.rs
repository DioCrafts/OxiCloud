use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "%s를 영구적으로 삭제할수 없습니다");
        m.insert("Couldn't restore %s", "%s를 복원할수 없습니다");
        m.insert("Error", "오류");
        m.insert("restored", "복원됨");
        m.insert("Nothing in here. Your trash bin is empty!", "현재 휴지통은 비어있습니다!");
        m.insert("Name", "이름");
        m.insert("Restore", "복원");
        m.insert("Deleted", "삭제됨");
        m.insert("Delete", "삭제");
        m.insert("Deleted Files", "삭제된 파일들");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}