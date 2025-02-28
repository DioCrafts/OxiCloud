use std::collections::HashMap;
use rust_i18n::i18n;

#[derive(Clone, Debug)]
pub struct ZhTW;

impl ZhTW {
    pub fn get_translations() -> HashMap<String, String> {
        let mut translations = HashMap::new();
        
        translations.insert(
            "App \"%s\" can't be installed because it is not compatible with this version of ownCloud.".to_string(),
            "無法安裝應用程式 %s 因為它和此版本的 ownCloud 不相容。".to_string()
        );
        translations.insert(
            "No app name specified".to_string(),
            "沒有指定應用程式名稱".to_string()
        );
        translations.insert(
            "Help".to_string(),
            "說明".to_string()
        );
        translations.insert(
            "Personal".to_string(),
            "個人".to_string()
        );
        translations.insert(
            "Settings".to_string(),
            "設定".to_string()
        );
        translations.insert(
            "Users".to_string(),
            "使用者".to_string()
        );
        translations.insert(
            "Admin".to_string(),
            "管理".to_string()
        );
        translations.insert(
            "Failed to upgrade \"%s\".".to_string(),
            "升級失敗：%s".to_string()
        );
        translations.insert(
            "Unknown filetype".to_string(),
            "未知的檔案類型".to_string()
        );
        translations.insert(
            "Invalid image".to_string(),
            "無效的圖片".to_string()
        );
        translations.insert(
            "web services under your control".to_string(),
            "由您控制的網路服務".to_string()
        );
        translations.insert(
            "cannot open \"%s\"".to_string(),
            "無法開啓 %s".to_string()
        );
        translations.insert(
            "ZIP download is turned off.".to_string(),
            "ZIP 下載已關閉。".to_string()
        );
        translations.insert(
            "Files need to be downloaded one by one.".to_string(),
            "檔案需要逐一下載。".to_string()
        );
        translations.insert(
            "Back to Files".to_string(),
            "回到檔案列表".to_string()
        );
        translations.insert(
            "Selected files too large to generate zip file.".to_string(),
            "選擇的檔案太大以致於無法產生壓縮檔。".to_string()
        );
        translations.insert(
            "Download the files in smaller chunks, seperately or kindly ask your administrator.".to_string(),
            "以小分割下載您的檔案，請詢問您的系統管理員。".to_string()
        );
        translations.insert(
            "No source specified when installing app".to_string(),
            "沒有指定應用程式安裝來源".to_string()
        );
        translations.insert(
            "No href specified when installing app from http".to_string(),
            "從 http 安裝應用程式，找不到 href 屬性".to_string()
        );
        translations.insert(
            "No path specified when installing app from local file".to_string(),
            "從本地檔案安裝應用程式時沒有指定路徑".to_string()
        );
        translations.insert(
            "Archives of type %s are not supported".to_string(),
            "不支援 %s 格式的壓縮檔".to_string()
        );
        translations.insert(
            "Failed to open archive when installing app".to_string(),
            "安裝應用程式時無法開啓壓縮檔".to_string()
        );
        translations.insert(
            "App does not provide an info.xml file".to_string(),
            "應用程式沒有提供 info.xml 檔案".to_string()
        );
        translations.insert(
            "App can't be installed because of not allowed code in the App".to_string(),
            "無法安裝應用程式因為在當中找到危險的代碼".to_string()
        );
        translations.insert(
            "App can't be installed because it is not compatible with this version of ownCloud".to_string(),
            "無法安裝應用程式因為它和此版本的 ownCloud 不相容。".to_string()
        );
        translations.insert(
            "App can't be installed because it contains the <shipped>true</shipped> tag which is not allowed for non shipped apps".to_string(),
            "無法安裝應用程式，因為它包含了 <shipped>true</shipped> 標籤，在未發行的應用程式當中這是不允許的".to_string()
        );
        translations.insert(
            "App can't be installed because the version in info.xml/version is not the same as the version reported from the app store".to_string(),
            "無法安裝應用程式，因為它在 info.xml/version 宣告的版本與 app store 當中記載的版本不同".to_string()
        );
        translations.insert(
            "App directory already exists".to_string(),
            "應用程式目錄已經存在".to_string()
        );
        translations.insert(
            "Can't create app folder. Please fix permissions. %s".to_string(),
            "無法建立應用程式目錄，請檢查權限：%s".to_string()
        );
        translations.insert(
            "Application is not enabled".to_string(),
            "應用程式未啟用".to_string()
        );
        translations.insert(
            "Authentication error".to_string(),
            "認證錯誤".to_string()
        );
        translations.insert(
            "Token expired. Please reload page.".to_string(),
            "Token 過期，請重新整理頁面。".to_string()
        );
        translations.insert(
            "Files".to_string(),
            "檔案".to_string()
        );
        translations.insert(
            "Text".to_string(),
            "文字".to_string()
        );
        translations.insert(
            "Images".to_string(),
            "圖片".to_string()
        );
        translations.insert(
            "%s enter the database username.".to_string(),
            "%s 輸入資料庫使用者名稱。".to_string()
        );
        translations.insert(
            "%s enter the database name.".to_string(),
            "%s 輸入資料庫名稱。".to_string()
        );
        translations.insert(
            "%s you may not use dots in the database name".to_string(),
            "%s 資料庫名稱不能包含小數點".to_string()
        );
        translations.insert(
            "MS SQL username and/or password not valid: %s".to_string(),
            "MS SQL 使用者和/或密碼無效：%s".to_string()
        );
        translations.insert(
            "You need to enter either an existing account or the administrator.".to_string(),
            "您必須輸入一個現有的帳號或管理員帳號。".to_string()
        );
        translations.insert(
            "MySQL username and/or password not valid".to_string(),
            "MySQL 用戶名和/或密碼無效".to_string()
        );
        translations.insert(
            "DB Error: \"%s\"".to_string(),
            "資料庫錯誤：\"%s\"".to_string()
        );
        translations.insert(
            "Offending command was: \"%s\"".to_string(),
            "有問題的指令是：\"%s\"".to_string()
        );
        translations.insert(
            "MySQL user '%s'@'localhost' exists already.".to_string(),
            "MySQL 使用者 '%s'@'localhost' 已經存在。".to_string()
        );
        translations.insert(
            "Drop this user from MySQL".to_string(),
            "在 MySQL 移除這個使用者".to_string()
        );
        translations.insert(
            "MySQL user '%s'@'%%' already exists".to_string(),
            "MySQL 使用者 '%s'@'%%' 已經存在".to_string()
        );
        translations.insert(
            "Drop this user from MySQL.".to_string(),
            "在 MySQL 移除這個使用者。".to_string()
        );
        translations.insert(
            "Oracle connection could not be established".to_string(),
            "無法建立 Oracle 資料庫連線".to_string()
        );
        translations.insert(
            "Oracle username and/or password not valid".to_string(),
            "Oracle 用戶名和/或密碼無效".to_string()
        );
        translations.insert(
            "Offending command was: \"%s\", name: %s, password: %s".to_string(),
            "有問題的指令是：\"%s\" ，使用者：\"%s\"，密碼：\"%s\"".to_string()
        );
        translations.insert(
            "PostgreSQL username and/or password not valid".to_string(),
            "PostgreSQL 用戶名和/或密碼無效".to_string()
        );
        translations.insert(
            "Set an admin username.".to_string(),
            "設定管理員帳號。".to_string()
        );
        translations.insert(
            "Set an admin password.".to_string(),
            "設定管理員密碼。".to_string()
        );
        translations.insert(
            "Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.".to_string(),
            "您的網頁伺服器尚未被正確設定來進行檔案同步，因為您的 WebDAV 界面似乎無法使用。".to_string()
        );
        translations.insert(
            "Please double check the <a href='%s'>installation guides</a>.".to_string(),
            "請參考<a href='%s'>安裝指南</a>。".to_string()
        );
        translations.insert(
            "Could not find category \"%s\"".to_string(),
            "找不到分類：\"%s\"".to_string()
        );
        translations.insert(
            "seconds ago".to_string(),
            "幾秒前".to_string()
        );
        translations.insert(
            "_%n minute ago_::_%n minutes ago_".to_string(),
            "%n 分鐘前".to_string()
        );
        translations.insert(
            "_%n hour ago_::_%n hours ago_".to_string(),
            "%n 小時前".to_string()
        );
        translations.insert(
            "today".to_string(),
            "今天".to_string()
        );
        translations.insert(
            "yesterday".to_string(),
            "昨天".to_string()
        );
        translations.insert(
            "_%n day go_::_%n days ago_".to_string(),
            "%n 天前".to_string()
        );
        translations.insert(
            "last month".to_string(),
            "上個月".to_string()
        );
        translations.insert(
            "_%n month ago_::_%n months ago_".to_string(),
            "%n 個月前".to_string()
        );
        translations.insert(
            "last year".to_string(),
            "去年".to_string()
        );
        translations.insert(
            "years ago".to_string(),
            "幾年前".to_string()
        );
        translations.insert(
            "Caused by:".to_string(),
            "原因：".to_string()
        );
        
        translations
    }
    
    pub fn get_plural_forms() -> &'static str {
        "nplurals=1; plural=0;"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_translations_load() {
        let translations = ZhTW::get_translations();
        assert!(!translations.is_empty());
        assert_eq!(translations.get("Help"), Some(&"說明".to_string()));
    }
    
    #[test]
    fn test_plural_forms() {
        assert_eq!(ZhTW::get_plural_forms(), "nplurals=1; plural=0;");
    }
}