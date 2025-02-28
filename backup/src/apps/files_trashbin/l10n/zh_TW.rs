use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Couldn't delete %s permanently", "無法永久刪除 %s");
    m.insert("Couldn't restore %s", "無法還原 %s");
    m.insert("Error", "錯誤");
    m.insert("restored", "已還原");
    m.insert("Nothing in here. Your trash bin is empty!", "您的回收桶是空的！");
    m.insert("Name", "名稱");
    m.insert("Restore", "還原");
    m.insert("Deleted", "已刪除");
    m.insert("Delete", "刪除");
    m.insert("Deleted Files", "已刪除的檔案");
    m
});

pub static PLURAL_FORMS: &str = "nplurals=1; plural=0;";