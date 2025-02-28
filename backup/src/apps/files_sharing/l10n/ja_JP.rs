use once_cell::sync::Lazy;
use std::collections::HashMap;

// Traducciones en japonés (Japón)
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("This share is password-protected", "この共有はパスワードで保護されています"),
        ("The password is wrong. Try again.", "パスワードが間違っています。再試行してください。"),
        ("Password", "パスワード"),
        ("Sorry, this link doesn't seem to work anymore.", "申し訳ございません。このリンクはもう利用できません。"),
        ("Reasons might be:", "理由は以下の通りと考えられます："),
        ("the item was removed", "アイテムが削除されました"),
        ("the link expired", "リンクの期限が切れています"),
        ("sharing is disabled", "共有が無効になっています"),
        ("For more info, please ask the person who sent this link.", "不明な点は、こちらのリンクの提供者に確認をお願いします。"),
        ("%s shared the folder %s with you", "%s はフォルダー %s をあなたと共有中です"),
        ("%s shared the file %s with you", "%s はファイル %s をあなたと共有中です"),
        ("Download", "ダウンロード"),
        ("Upload", "アップロード"),
        ("Cancel upload", "アップロードをキャンセル"),
        ("No preview available for", "プレビューはありません"),
        ("Direct link", "リンク"),
    ])
});

// Información de pluralización
pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn translate(key: &str) -> &'static str {
    get_translation(key).unwrap_or(key)
}

pub fn translate_with_args(key: &str, args: &[&str]) -> String {
    let template = translate(key);
    let mut result = String::from(template);
    
    for (i, arg) in args.iter().enumerate() {
        let placeholder = format!("%s", if i > 0 { i.to_string() } else { String::new() });
        result = result.replace(&placeholder, arg);
    }
    
    result
}