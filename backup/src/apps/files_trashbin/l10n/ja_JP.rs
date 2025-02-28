use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static JA_JP_TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Couldn't delete %s permanently", "%s を完全に削除出来ませんでした");
    translations.insert("Couldn't restore %s", "%s を復元出来ませんでした");
    translations.insert("Error", "エラー");
    translations.insert("restored", "復元済");
    translations.insert("Nothing in here. Your trash bin is empty!", "ここには何もありません。ゴミ箱は空です！");
    translations.insert("Name", "名前");
    translations.insert("Restore", "復元");
    translations.insert("Deleted", "削除済み");
    translations.insert("Delete", "削除");
    translations.insert("Deleted Files", "ゴミ箱");
    translations
});

pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";