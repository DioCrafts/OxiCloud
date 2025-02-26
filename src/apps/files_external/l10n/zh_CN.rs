use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Translations for Simplified Chinese (zh_CN)
pub static ZH_CN_TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Access granted", "权限已授予。");
    map.insert("Error configuring Dropbox storage", "配置Dropbox存储时出错");
    map.insert("Grant access", "授权");
    map.insert("Please provide a valid Dropbox app key and secret.", "请提供有效的Dropbox应用key和secret");
    map.insert("Error configuring Google Drive storage", "配置Google Drive存储时出错");
    map.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>警告：</b>"smbclient" 尚未安装。CIFS/SMB 分享挂载无法实现。请咨询系统管理员进行安装。");
    map.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>警告：</b>PHP中尚未启用或安装FTP。FTP 分享挂载无法实现。请咨询系统管理员进行安装。");
    map.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>警告：</b> PHP中未启用或未安装Curl支持。ownCloud / WebDAV 或 GoogleDrive 不能挂载。请请求您的系统管理员安装该它。");
    map.insert("External Storage", "外部存储");
    map.insert("Folder name", "目录名称");
    map.insert("External storage", "外部存储");
    map.insert("Configuration", "配置");
    map.insert("Options", "选项");
    map.insert("Applicable", "适用的");
    map.insert("Add storage", "添加存储");
    map.insert("None set", "未设置");
    map.insert("All Users", "所有用户");
    map.insert("Groups", "组");
    map.insert("Users", "用户");
    map.insert("Delete", "删除");
    map.insert("Enable User External Storage", "启用用户外部存储");
    map.insert("Allow users to mount their own external storage", "允许用户挂载自有外部存储");
    map.insert("SSL root certificates", "SSL根证书");
    map.insert("Import Root Certificate", "导入根证书");
    map
});

pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";

pub fn get_translation(key: &str) -> Option<&'static str> {
    ZH_CN_TRANSLATIONS.get(key).copied()
}