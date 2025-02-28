use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("This share is password-protected", "這個分享有密碼保護");
        m.insert("The password is wrong. Try again.", "請檢查您的密碼並再試一次");
        m.insert("Password", "密碼");
        m.insert("Sorry, this link doesn't seem to work anymore.", "抱歉，此連結已經失效");
        m.insert("Reasons might be:", "可能的原因：");
        m.insert("the item was removed", "項目已經移除");
        m.insert("the link expired", "連結過期");
        m.insert("sharing is disabled", "分享功能已停用");
        m.insert("For more info, please ask the person who sent this link.", "請詢問告訴您此連結的人以瞭解更多");
        m.insert("%s shared the folder %s with you", "%s 和您分享了資料夾 %s ");
        m.insert("%s shared the file %s with you", "%s 和您分享了檔案 %s");
        m.insert("Download", "下載");
        m.insert("Upload", "上傳");
        m.insert("Cancel upload", "取消上傳");
        m.insert("No preview available for", "無法預覽");
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