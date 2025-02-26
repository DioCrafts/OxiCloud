use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Chinese (China) translation strings
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("The password is wrong. Try again.", "用户名或密码错误！请重试");
    m.insert("Password", "密码");
    m.insert("Sorry, this link doesn't seem to work anymore.", "抱歉，此链接已失效");
    m.insert("Reasons might be:", "可能原因是：");
    m.insert("the item was removed", "此项已移除");
    m.insert("the link expired", "链接过期");
    m.insert("sharing is disabled", "共享已禁用");
    m.insert("For more info, please ask the person who sent this link.", "欲知详情，请联系发给你链接的人。");
    m.insert("%s shared the folder %s with you", "%s与您共享了%s文件夹");
    m.insert("%s shared the file %s with you", "%s与您共享了%s文件");
    m.insert("Download", "下载");
    m.insert("Upload", "上传");
    m.insert("Cancel upload", "取消上传");
    m.insert("No preview available for", "没有预览");
    m
});

/// Plural forms formula for Chinese (China)
pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";