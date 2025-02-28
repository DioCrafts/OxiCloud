use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Recovery key successfully enabled", "کلید بازیابی با موفقیت فعال شده است.");
        m.insert("Could not enable recovery key. Please check your recovery key password!", "کلید بازیابی نمی تواند فعال شود. لطفا رمزعبور کلید بازیابی خود را بررسی نمایید!");
        m.insert("Recovery key successfully disabled", "کلید بازیابی با موفقیت غیر فعال شده است.");
        m.insert("Could not disable recovery key. Please check your recovery key password!", "کلید بازیابی را نمی تواند غیرفعال نماید. لطفا رمزعبور کلید بازیابی خود را بررسی کنید!");
        m.insert("Password successfully changed.", "رمزعبور با موفقیت تغییر یافت.");
        m.insert("Could not change the password. Maybe the old password was not correct.", "رمزعبور را نمیتواند تغییر دهد. شاید رمزعبورقدیمی صحیح نمی باشد.");
        m.insert("Private key password successfully updated.", "رمزعبور کلید خصوصی با موفقیت به روز شد.");
        m.insert("Could not update the private key password. Maybe the old password was not correct.", "رمزعبور کلید خصوصی را نمی تواند به روز کند. شاید رمزعبور قدیمی صحیح نمی باشد.");
        m.insert("Missing requirements.", "نیازمندی های گمشده");
        m.insert("Saving...", "در حال ذخیره سازی...");
        m.insert("personal settings", "تنظیمات شخصی");
        m.insert("Encryption", "رمزگذاری");
        m.insert("Enable recovery key (allow to recover users files in case of password loss):", "فعال کردن کلید بازیابی(اجازه بازیابی فایل های کاربران در صورت از دست دادن رمزعبور):");
        m.insert("Recovery key password", "رمزعبور کلید بازیابی");
        m.insert("Enabled", "فعال شده");
        m.insert("Disabled", "غیرفعال شده");
        m.insert("Change recovery key password:", "تغییر رمزعبور کلید بازیابی:");
        m.insert("Old Recovery key password", "رمزعبور قدیمی  کلید بازیابی ");
        m.insert("New Recovery key password", "رمزعبور جدید کلید بازیابی");
        m.insert("Change Password", "تغییر رمزعبور");
        m.insert("Your private key password no longer match your log-in password:", "رمزعبور کلید خصوصی شما با رمزعبور شما یکسان نیست :");
        m.insert("Set your old private key password to your current log-in password.", "رمزعبور قدیمی  کلید خصوصی خود را با رمزعبور فعلی تنظیم نمایید.");
        m.insert(" If you don't remember your old password you can ask your administrator to recover your files.", "اگر رمزعبور قدیمی را فراموش کرده اید میتوانید از مدیر خود برای بازیابی فایل هایتان درخواست نمایید.");
        m.insert("Old log-in password", "رمزعبور قدیمی");
        m.insert("Current log-in password", "رمزعبور فعلی");
        m.insert("Update Private Key Password", "به روز رسانی رمزعبور کلید خصوصی");
        m.insert("Enable password recovery:", "فعال سازی بازیابی رمزعبور:");
        m.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "فعال کردن این گزینه به شما اجازه خواهد داد در صورت از دست دادن رمزعبور به فایل های رمزگذاری شده خود دسترسی داشته باشید.");
        m.insert("File recovery settings updated", "تنظیمات بازیابی فایل به روز شده است.");
        m.insert("Could not update file recovery", "به روز رسانی بازیابی فایل را نمی تواند انجام دهد.");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}