use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Translations for Taiwanese Chinese (zh_TW)
pub static ZH_TW: Lazy<Translations> = Lazy::new(|| {
    let mut translations = HashMap::new();
    
    translations.insert("Unable to load list from App Store".to_string(), "無法從 App Store 讀取清單".to_string());
    translations.insert("Authentication error".to_string(), "認證錯誤".to_string());
    translations.insert("Group already exists".to_string(), "群組已存在".to_string());
    translations.insert("Unable to add group".to_string(), "群組增加失敗".to_string());
    translations.insert("Email saved".to_string(), "Email已儲存".to_string());
    translations.insert("Invalid email".to_string(), "無效的email".to_string());
    translations.insert("Unable to delete group".to_string(), "群組刪除錯誤".to_string());
    translations.insert("Unable to delete user".to_string(), "使用者刪除錯誤".to_string());
    translations.insert("Language changed".to_string(), "語言已變更".to_string());
    translations.insert("Invalid request".to_string(), "無效請求".to_string());
    translations.insert("Admins can't remove themself from the admin group".to_string(), "管理者帳號無法從管理者群組中移除".to_string());
    translations.insert("Unable to add user to group %s".to_string(), "使用者加入群組 %s 錯誤".to_string());
    translations.insert("Unable to remove user from group %s".to_string(), "使用者移出群組 %s 錯誤".to_string());
    translations.insert("Couldn't update app.".to_string(), "無法更新應用程式".to_string());
    translations.insert("Wrong password".to_string(), "密碼錯誤".to_string());
    translations.insert("No user supplied".to_string(), "未提供使用者".to_string());
    translations.insert("Please provide an admin recovery password, otherwise all user data will be lost".to_string(), "請提供管理者還原密碼，否則會遺失所有使用者資料".to_string());
    translations.insert("Wrong admin recovery password. Please check the password and try again.".to_string(), "錯誤的管理者還原密碼".to_string());
    translations.insert("Back-end doesn't support password change, but the users encryption key was successfully updated.".to_string(), "後端不支援變更密碼，但成功更新使用者的加密金鑰".to_string());
    translations.insert("Unable to change password".to_string(), "無法修改密碼".to_string());
    translations.insert("Update to {appversion}".to_string(), "更新至 {appversion}".to_string());
    translations.insert("Disable".to_string(), "停用".to_string());
    translations.insert("Enable".to_string(), "啟用".to_string());
    translations.insert("Please wait....".to_string(), "請稍候...".to_string());
    translations.insert("Error while disabling app".to_string(), "停用應用程式錯誤".to_string());
    translations.insert("Error while enabling app".to_string(), "啓用應用程式錯誤".to_string());
    translations.insert("Updating....".to_string(), "更新中...".to_string());
    translations.insert("Error while updating app".to_string(), "更新應用程式錯誤".to_string());
    translations.insert("Error".to_string(), "錯誤".to_string());
    translations.insert("Update".to_string(), "更新".to_string());
    translations.insert("Updated".to_string(), "已更新".to_string());
    translations.insert("Select a profile picture".to_string(), "選擇大頭貼".to_string());
    translations.insert("Decrypting files... Please wait, this can take some time.".to_string(), "檔案解密中，請稍候。".to_string());
    translations.insert("Saving...".to_string(), "儲存中...".to_string());
    translations.insert("deleted".to_string(), "已刪除".to_string());
    translations.insert("undo".to_string(), "復原".to_string());
    translations.insert("Unable to remove user".to_string(), "無法刪除用戶".to_string());
    translations.insert("Groups".to_string(), "群組".to_string());
    translations.insert("Group Admin".to_string(), "群組管理員".to_string());
    translations.insert("Delete".to_string(), "刪除".to_string());
    translations.insert("add group".to_string(), "新增群組".to_string());
    translations.insert("A valid username must be provided".to_string(), "必須提供一個有效的用戶名".to_string());
    translations.insert("Error creating user".to_string(), "建立用戶時出現錯誤".to_string());
    translations.insert("A valid password must be provided".to_string(), "一定要提供一個有效的密碼".to_string());
    translations.insert("Warning: Home directory for user \"{user}\" already exists".to_string(), "警告：使用者 {user} 的家目錄已經存在".to_string());
    translations.insert("__language_name__".to_string(), "__language_name__".to_string());
    translations.insert("Security Warning".to_string(), "安全性警告".to_string());
    translations.insert("Your data directory and your files are probably accessible from the internet. The .htaccess file is not working. We strongly suggest that you configure your webserver in a way that the data directory is no longer accessible or you move the data directory outside the webserver document root.".to_string(), "您的資料目錄 (Data Directory) 和檔案可能可以由網際網路上面公開存取。Owncloud 所提供的 .htaccess 設定檔並未生效，我們強烈建議您設定您的網頁伺服器以防止資料目錄被公開存取，或將您的資料目錄移出網頁伺服器的 document root 。".to_string());
    translations.insert("Setup Warning".to_string(), "設定警告".to_string());
    translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.".to_string(), "您的網頁伺服器尚未被正確設定來進行檔案同步，因為您的 WebDAV 界面似乎無法使用。".to_string());
    translations.insert("Please double check the <a href=\"%s\">installation guides</a>.".to_string(), "請參考<a href='%s'>安裝指南</a>。".to_string());
    translations.insert("Module 'fileinfo' missing".to_string(), "遺失 'fileinfo' 模組".to_string());
    translations.insert("The PHP module 'fileinfo' is missing. We strongly recommend to enable this module to get best results with mime-type detection.".to_string(), "未偵測到 PHP 模組 'fileinfo'。我們強烈建議啟用這個模組以取得最好的 mime-type 支援。".to_string());
    translations.insert("Locale not working".to_string(), "語系無法運作".to_string());
    translations.insert("System locale can't be set to %s. This means that there might be problems with certain characters in file names. We strongly suggest to install the required packages on your system to support %s.".to_string(), "ownCloud 伺服器無法將系統語系設為 %s ，可能有一些檔名中的字元有問題，建議您安裝所有所需的套件以支援 %s 。".to_string());
    translations.insert("Internet connection not working".to_string(), "無網際網路存取".to_string());
    translations.insert("This server has no working internet connection. This means that some of the features like mounting of external storage, notifications about updates or installation of 3rd party apps don´t work. Accessing files from remote and sending of notification emails might also not work. We suggest to enable internet connection for this server if you want to have all features.".to_string(), "這臺 ownCloud 伺服器沒有連接到網際網路，因此有些功能像是掛載外部儲存空間、更新 ownCloud 或應用程式的通知沒有辦法運作。透過網際網路存取檔案還有電子郵件通知可能也無法運作。如果想要 ownCloud 完整的功能，建議您將這臺伺服器連接至網際網路。".to_string());
    translations.insert("Cron".to_string(), "Cron".to_string());
    translations.insert("Execute one task with each page loaded".to_string(), "當頁面載入時，執行".to_string());
    translations.insert("cron.php is registered at a webcron service to call cron.php every 15 minutes over http.".to_string(), "已經與 webcron 服務註冊好，將會每15分鐘呼叫 cron.php".to_string());
    translations.insert("Use systems cron service to call the cron.php file every 15 minutes.".to_string(), "使用系統的 cron 服務每15分鐘呼叫 cron.php 一次".to_string());
    translations.insert("Sharing".to_string(), "分享".to_string());
    translations.insert("Enable Share API".to_string(), "啟用分享 API".to_string());
    translations.insert("Allow apps to use the Share API".to_string(), "允許 apps 使用分享 API".to_string());
    translations.insert("Allow links".to_string(), "允許連結".to_string());
    translations.insert("Allow users to share items to the public with links".to_string(), "允許使用者以結連公開分享檔案".to_string());
    translations.insert("Allow public uploads".to_string(), "允許任何人上傳".to_string());
    translations.insert("Allow users to enable others to upload into their publicly shared folders".to_string(), "允許使用者將他們公開分享的資料夾設定為「任何人皆可上傳」".to_string());
    translations.insert("Allow resharing".to_string(), "允許轉貼分享".to_string());
    translations.insert("Allow users to share items shared with them again".to_string(), "允許使用者分享其他使用者分享給他的檔案".to_string());
    translations.insert("Allow users to share with anyone".to_string(), "允許使用者與任何人分享檔案".to_string());
    translations.insert("Allow users to only share with users in their groups".to_string(), "僅允許使用者在群組內分享".to_string());
    translations.insert("Allow mail notification".to_string(), "允許郵件通知".to_string());
    translations.insert("Allow user to send mail notification for shared files".to_string(), "允許使用者分享檔案時寄出通知郵件".to_string());
    translations.insert("Security".to_string(), "安全性".to_string());
    translations.insert("Enforce HTTPS".to_string(), "強制啟用 HTTPS".to_string());
    translations.insert("Forces the clients to connect to %s via an encrypted connection.".to_string(), "強迫用戶端使用加密連線連接到 %s".to_string());
    translations.insert("Please connect to your %s via HTTPS to enable or disable the SSL enforcement.".to_string(), "請使用 HTTPS 連線到 %s 以啓用或停用強制 SSL 加密。".to_string());
    translations.insert("Log".to_string(), "紀錄".to_string());
    translations.insert("Log level".to_string(), "紀錄層級".to_string());
    translations.insert("More".to_string(), "更多".to_string());
    translations.insert("Less".to_string(), "更少".to_string());
    translations.insert("Version".to_string(), "版本".to_string());
    translations.insert("Developed by the <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud community</a>, the <a href=\"https://github.com/owncloud\" target=\"_blank\">source code</a> is licensed under the <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.".to_string(), "由 <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud 社群</a>開發，<a href=\"https://github.com/owncloud\" target=\"_blank\">原始碼</a>在 <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a> 授權許可下發布。".to_string());
    translations.insert("Add your App".to_string(), "添加你的 App".to_string());
    translations.insert("More Apps".to_string(), "更多Apps".to_string());
    translations.insert("Select an App".to_string(), "選擇一個應用程式".to_string());
    translations.insert("See application page at apps.owncloud.com".to_string(), "查看應用程式頁面於 apps.owncloud.com".to_string());
    translations.insert("<span class=\"licence\"></span>-licensed by <span class=\"author\"></span>".to_string(), "<span class=\"licence\"></span>-核准: <span class=\"author\"></span>".to_string());
    translations.insert("User Documentation".to_string(), "用戶說明文件".to_string());
    translations.insert("Administrator Documentation".to_string(), "管理者說明文件".to_string());
    translations.insert("Online Documentation".to_string(), "線上說明文件".to_string());
    translations.insert("Forum".to_string(), "論壇".to_string());
    translations.insert("Bugtracker".to_string(), "Bugtracker".to_string());
    translations.insert("Commercial Support".to_string(), "商用支援".to_string());
    translations.insert("Get the apps to sync your files".to_string(), "下載應用程式來同步您的檔案".to_string());
    translations.insert("Show First Run Wizard again".to_string(), "再次顯示首次使用精靈".to_string());
    translations.insert("You have used <strong>%s</strong> of the available <strong>%s</strong>".to_string(), "您已經使用了 <strong>%s</strong> ，目前可用空間為 <strong>%s</strong>".to_string());
    translations.insert("Password".to_string(), "密碼".to_string());
    translations.insert("Your password was changed".to_string(), "你的密碼已更改".to_string());
    translations.insert("Unable to change your password".to_string(), "無法變更您的密碼".to_string());
    translations.insert("Current password".to_string(), "目前密碼".to_string());
    translations.insert("New password".to_string(), "新密碼".to_string());
    translations.insert("Change password".to_string(), "變更密碼".to_string());
    translations.insert("Email".to_string(), "信箱".to_string());
    translations.insert("Your email address".to_string(), "您的電子郵件信箱".to_string());
    translations.insert("Fill in an email address to enable password recovery".to_string(), "請填入電子郵件信箱以便回復密碼".to_string());
    translations.insert("Profile picture".to_string(), "個人資料照片".to_string());
    translations.insert("Upload new".to_string(), "上傳新的".to_string());
    translations.insert("Select new from Files".to_string(), "從已上傳的檔案中選一個".to_string());
    translations.insert("Remove image".to_string(), "移除圖片".to_string());
    translations.insert("Either png or jpg. Ideally square but you will be able to crop it.".to_string(), "可以使用 png 或 jpg 格式，最好是方形的，但是您之後也可以裁剪它".to_string());
    translations.insert("Abort".to_string(), "中斷".to_string());
    translations.insert("Choose as profile image".to_string(), "設定為大頭貼".to_string());
    translations.insert("Language".to_string(), "語言".to_string());
    translations.insert("Help translate".to_string(), "幫助翻譯".to_string());
    translations.insert("WebDAV".to_string(), "WebDAV".to_string());
    translations.insert("Use this address to <a href=\"%s\" target=\"_blank\">access your Files via WebDAV</a>".to_string(), "使用這個地址<a href=\"%s\" target=\"_blank\">來透過 WebDAV 存取檔案</a>".to_string());
    translations.insert("Encryption".to_string(), "加密".to_string());
    translations.insert("The encryption app is no longer enabled, decrypt all your file".to_string(), "加密應用程式已經停用，請您解密您所有的檔案".to_string());
    translations.insert("Log-in password".to_string(), "登入密碼".to_string());
    translations.insert("Decrypt all Files".to_string(), "解密所有檔案".to_string());
    translations.insert("Login Name".to_string(), "登入名稱".to_string());
    translations.insert("Create".to_string(), "建立".to_string());
    translations.insert("Admin Recovery Password".to_string(), "管理者復原密碼".to_string());
    translations.insert("Enter the recovery password in order to recover the users files during password change".to_string(), "為了修改密碼時能夠取回使用者資料，請輸入另一組還原用密碼".to_string());
    translations.insert("Default Storage".to_string(), "預設儲存區".to_string());
    translations.insert("Unlimited".to_string(), "無限制".to_string());
    translations.insert("Other".to_string(), "其他".to_string());
    translations.insert("Username".to_string(), "使用者名稱".to_string());
    translations.insert("Storage".to_string(), "儲存區".to_string());
    translations.insert("set new password".to_string(), "設定新密碼".to_string());
    translations.insert("Default".to_string(), "預設".to_string());
    
    Translations {
        translations,
        plural_forms: "nplurals=1; plural=0;".to_string(),
    }
});

/// Structure for storing translations
pub struct Translations {
    /// Map of string keys to their translated values
    pub translations: HashMap<String, String>,
    /// Plural forms expression for this language
    pub plural_forms: String,
}

impl Translations {
    /// Get a translation by key
    pub fn get(&self, key: &str) -> Option<&String> {
        self.translations.get(key)
    }
    
    /// Get a translation by key or fallback to the key itself
    pub fn get_or_key(&self, key: &str) -> &str {
        self.translations.get(key).map_or(key, |s| s.as_str())
    }
}