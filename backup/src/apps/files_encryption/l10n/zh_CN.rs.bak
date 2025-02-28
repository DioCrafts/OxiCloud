use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Recovery key successfully enabled", "恢复密钥成功启用");
        m.insert("Could not enable recovery key. Please check your recovery key password!", "不能启用恢复密钥。请检查恢复密钥密码！");
        m.insert("Recovery key successfully disabled", "恢复密钥成功禁用");
        m.insert("Could not disable recovery key. Please check your recovery key password!", "不能禁用恢复密钥。请检查恢复密钥密码！");
        m.insert("Password successfully changed.", "密码修改成功。");
        m.insert("Could not change the password. Maybe the old password was not correct.", "不能修改密码。旧密码可能不正确。");
        m.insert("Private key password successfully updated.", "私钥密码成功更新。");
        m.insert("Could not update the private key password. Maybe the old password was not correct.", "无法更新私钥密码。可能旧密码不正确。");
        m.insert("Saving...", "保存中");
        m.insert("personal settings", "个人设置");
        m.insert("Encryption", "加密");
        m.insert("Enable recovery key (allow to recover users files in case of password loss):", "启用恢复密钥（允许你在密码丢失后恢复文件）：");
        m.insert("Recovery key password", "恢复密钥密码");
        m.insert("Enabled", "开启");
        m.insert("Disabled", "禁用");
        m.insert("Change recovery key password:", "更改恢复密钥密码");
        m.insert("Old Recovery key password", "旧的恢复密钥密码");
        m.insert("New Recovery key password", "新的恢复密钥密码");
        m.insert("Change Password", "修改密码");
        m.insert("Your private key password no longer match your log-in password:", "您的私钥密码不再匹配您的登录密码：");
        m.insert("Set your old private key password to your current log-in password.", "讲您旧的私钥密码改为当前登录密码。");
        m.insert(" If you don't remember your old password you can ask your administrator to recover your files.", "如果您记不住旧的密码，您可以请求管理员恢复您的文件。");
        m.insert("Old log-in password", "旧登录密码");
        m.insert("Current log-in password", "当前登录密码");
        m.insert("Update Private Key Password", "更新私钥密码");
        m.insert("Enable password recovery:", "启用密码恢复：");
        m.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "启用该项将允许你在密码丢失后取回您的加密文件");
        m.insert("File recovery settings updated", "文件恢复设置已更新");
        m.insert("Could not update file recovery", "不能更新文件恢复");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}