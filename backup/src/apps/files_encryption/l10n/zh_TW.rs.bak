use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Recovery key successfully enabled", "還原金鑰已成功開啟");
    m.insert("Could not enable recovery key. Please check your recovery key password!", "無法啟用還原金鑰。請檢查您的還原金鑰密碼!");
    m.insert("Recovery key successfully disabled", "還原金鑰已成功停用");
    m.insert("Could not disable recovery key. Please check your recovery key password!", "無法停用還原金鑰。請檢查您的還原金鑰密碼!");
    m.insert("Password successfully changed.", "成功變更密碼。");
    m.insert("Could not change the password. Maybe the old password was not correct.", "無法變更密碼，或許是輸入的舊密碼不正確。");
    m.insert("Private key password successfully updated.", "私人金鑰密碼已成功更新。");
    m.insert("Could not update the private key password. Maybe the old password was not correct.", "無法更新私人金鑰密碼。可能舊的密碼不正確。");
    m.insert("Encryption app not initialized! Maybe the encryption app was re-enabled during your session. Please try to log out and log back in to initialize the encryption app.", "加密功能未初始化!可能加密功能需要重新啟用在現在的連線上。請試著登出再登入來初始化加密功能。");
    m.insert("Your private key is not valid! Likely your password was changed outside of %s (e.g. your corporate directory). You can update your private key password in your personal settings to recover access to your encrypted files.", "您的私人金鑰不正確!可能您的密碼已經變更在外部的 %s (例如:您的企業目錄)。您可以在您的個人設定中更新私人金鑰密碼來還原存取您的加密檔案。");
    m.insert("Can not decrypt this file, probably this is a shared file. Please ask the file owner to reshare the file with you.", "無法解密這個檔案，也許這是分享的檔案。請詢問檔案所有人重新分享檔案給您。");
    m.insert("Unknown error please check your system settings or contact your administrator", "未知錯誤請檢查您的系統設定或是聯絡您的管理員");
    m.insert("Missing requirements.", "遺失必要條件。");
    m.insert("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "請確認已安裝 PHP 5.3.3 或是更新的版本以及 OpenSSL 也一併安裝在 PHP extension 裡面並啟用及設置完成。現在，加密功能是停用的。");
    m.insert("Following users are not set up for encryption:", "以下的使用者無法設定加密:");
    m.insert("Saving...", "儲存中...");
    m.insert("Go directly to your ", "直接到您的");
    m.insert("personal settings", "個人設定");
    m.insert("Encryption", "加密");
    m.insert("Enable recovery key (allow to recover users files in case of password loss):", "啟用還原金鑰 (因忘記密碼仍允許還原使用者檔案):");
    m.insert("Recovery key password", "還原金鑰密碼");
    m.insert("Repeat Recovery key password", "再輸入還原金鑰密碼一次");
    m.insert("Enabled", "已啓用");
    m.insert("Disabled", "已停用");
    m.insert("Change recovery key password:", "變更還原金鑰密碼:");
    m.insert("Old Recovery key password", "舊的還原金鑰密碼");
    m.insert("New Recovery key password", "新的還原金鑰密碼");
    m.insert("Repeat New Recovery key password", "再輸入新的還原金鑰密碼一次");
    m.insert("Change Password", "變更密碼");
    m.insert("Your private key password no longer match your log-in password:", "您的私人金鑰密碼不符合您的登入密碼:");
    m.insert("Set your old private key password to your current log-in password.", "設定您的舊私人金鑰密碼到您現在的登入密碼。");
    m.insert(" If you don't remember your old password you can ask your administrator to recover your files.", "如果您忘記舊密碼，可以請求管理員協助取回檔案。");
    m.insert("Old log-in password", "舊登入密碼");
    m.insert("Current log-in password", "目前的登入密碼");
    m.insert("Update Private Key Password", "更新私人金鑰密碼");
    m.insert("Enable password recovery:", "啟用密碼還原:");
    m.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "啟用這個選項將會允許您因忘記密碼但需要存取您的加密檔案");
    m.insert("File recovery settings updated", "檔案還原設定已更新");
    m.insert("Could not update file recovery", "無法更新檔案還原設定");
    m
});

pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}