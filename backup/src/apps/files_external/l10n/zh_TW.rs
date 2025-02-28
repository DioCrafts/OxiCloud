use std::collections::HashMap;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;

pub static ZH_TW: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Access granted", "允許存取");
    translations.insert("Error configuring Dropbox storage", "設定 Dropbox 儲存時發生錯誤");
    translations.insert("Grant access", "允許存取");
    translations.insert("Please provide a valid Dropbox app key and secret.", "請提供有效的 Dropbox app key 和 app secret 。");
    translations.insert("Error configuring Google Drive storage", "設定 Google Drive 儲存時發生錯誤");
    translations.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>警告</b>：未安裝 \"smbclient\" ，因此無法掛載 CIFS/SMB 分享，請洽您的系統管理員將其安裝。");
    translations.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>警告</b>：PHP 並未啓用 FTP 的支援，因此無法掛載 FTP 分享，請洽您的系統管理員將其安裝並啓用。");
    translations.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>警告</b>：PHP 並未啓用 Curl 的支援，因此無法掛載 ownCloud/WebDAV 或 Google Drive 分享，請洽您的系統管理員將其安裝並啓用。");
    translations.insert("External Storage", "外部儲存");
    translations.insert("Folder name", "資料夾名稱");
    translations.insert("External storage", "外部儲存");
    translations.insert("Configuration", "設定");
    translations.insert("Options", "選項");
    translations.insert("Applicable", "可用的");
    translations.insert("Add storage", "增加儲存區");
    translations.insert("None set", "尚未設定");
    translations.insert("All Users", "所有使用者");
    translations.insert("Groups", "群組");
    translations.insert("Users", "使用者");
    translations.insert("Delete", "刪除");
    translations.insert("Enable User External Storage", "啓用使用者外部儲存");
    translations.insert("Allow users to mount their own external storage", "允許使用者自行掛載他們的外部儲存");
    translations.insert("SSL root certificates", "SSL 根憑證");
    translations.insert("Import Root Certificate", "匯入根憑證");
    translations
});

pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";