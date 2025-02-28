use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::Plural;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Files", "文件");
        m.insert("Share", "分享");
        m.insert("_%n folder_::_%n folders_", "");
        m.insert("_%n file_::_%n files_", "");
        m.insert("_Uploading %n file_::_Uploading %n files_", "");
        m.insert("Error", "錯誤");
        m.insert("Name", "名稱");
        m.insert("Upload", "上傳");
        m.insert("Save", "儲存");
        m.insert("Download", "下載");
        m.insert("Unshare", "取消分享");
        m.insert("Delete", "刪除");
        m
    };
}

pub fn plural_forms(n: usize) -> Plural {
    // nplurals=1; plural=0;
    Plural::One
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_translation(key: &str, n: usize) -> &'static str {
    // For this language, there is only one plural form
    get_translation(key)
}