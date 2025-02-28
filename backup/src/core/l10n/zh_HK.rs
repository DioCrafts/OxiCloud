// core/l10n/zh_hk.rs

use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Sunday", "星期日");
        m.insert("Monday", "星期一");
        m.insert("Tuesday", "星期二");
        m.insert("Wednesday", "星期三");
        m.insert("Thursday", "星期四");
        m.insert("Friday", "星期五");
        m.insert("Saturday", "星期六");
        m.insert("January", "一月");
        m.insert("February", "二月");
        m.insert("March", "三月");
        m.insert("April", "四月");
        m.insert("May", "五月");
        m.insert("June", "六月");
        m.insert("July", "七月");
        m.insert("August", "八月");
        m.insert("September", "九月");
        m.insert("October", "十月");
        m.insert("November", "十一月");
        m.insert("December", "十二月");
        m.insert("Settings", "設定");
        m.insert("_%n minute ago_::_%n minutes ago_", "");
        m.insert("_%n hour ago_::_%n hours ago_", "");
        m.insert("today", "今日");
        m.insert("yesterday", "昨日");
        m.insert("_%n day ago_::_%n days ago_", "");
        m.insert("last month", "前一月");
        m.insert("_%n month ago_::_%n months ago_", "");
        m.insert("months ago", "個月之前");
        m.insert("Yes", "Yes");
        m.insert("No", "No");
        m.insert("Ok", "OK");
        m.insert("_{count} file conflict_::_{count} file conflicts_", "");
        m.insert("Cancel", "取消");
        m.insert("Shared", "已分享");
        m.insert("Share", "分享");
        m.insert("Error", "錯誤");
        m.insert("Error while sharing", "分享時發生錯誤");
        m.insert("Error while unsharing", "取消分享時發生錯誤");
        m.insert("Error while changing permissions", "更改權限時發生錯誤");
        m.insert("Shared with you and the group {group} by {owner}", "{owner}與你及群組的分享");
        m.insert("Shared with you by {owner}", "{owner}與你的分享");
        m.insert("Password protect", "密碼保護");
        m.insert("Password", "密碼");
        m.insert("Send", "傳送");
        m.insert("Set expiration date", "設定分享期限");
        m.insert("Expiration date", "分享期限");
        m.insert("Share via email:", "以電郵分享");
        m.insert("No people found", "找不到");
        m.insert("Unshare", "取消分享");
        m.insert("create", "新增");
        m.insert("update", "更新");
        m.insert("delete", "刪除");
        m.insert("share", "分享");
        m.insert("Password protected", "密碼保護");
        m.insert("Sending ...", "傳送中");
        m.insert("Email sent", "郵件已傳");
        m.insert("Delete", "刪除");
        m.insert("Add", "加入");
        m.insert("The update was successful. Redirecting you to ownCloud now.", "更新成功, 正");
        m.insert("Use the following link to reset your password: {link}", "請用以下連結重設你的密碼: {link}");
        m.insert("You will receive a link to reset your password via Email.", "你將收到一封電郵");
        m.insert("Username", "用戶名稱");
        m.insert("Your password was reset", "你的密碼已被重設");
        m.insert("To login page", "前往登入版面");
        m.insert("New password", "新密碼");
        m.insert("Reset password", "重設密碼");
        m.insert("Personal", "個人");
        m.insert("Users", "用戶");
        m.insert("Apps", "軟件");
        m.insert("Admin", "管理");
        m.insert("Help", "幫助");
        m.insert("Cloud not found", "未找到Cloud");
        m.insert("Create an <strong>admin account</strong>", "建立管理員帳戶");
        m.insert("Advanced", "進階");
        m.insert("Configure the database", "設定資料庫");
        m.insert("will be used", "將被使用");
        m.insert("Database user", "資料庫帳戶");
        m.insert("Database password", "資料庫密碼");
        m.insert("Database name", "資料庫名稱");
        m.insert("Log out", "登出");
        m.insert("Automatic logon rejected!", "自動登入被拒");
        m.insert("If you did not change your password recently, your account may be compromised!", "如果你近期未曾更改密碼, 你的帳號可能被洩露!");
        m.insert("Please change your password to secure your account again.", "請更改你的密碼以保護你的帳戶");
        m.insert("Lost your password?", "忘記密碼");
        m.insert("remember", "記住");
        m.insert("Log in", "登入");
        m.insert("Updating ownCloud to version %s, this may take a while.", "ownCloud (ver. %s)更新中, 請耐心等侯");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}