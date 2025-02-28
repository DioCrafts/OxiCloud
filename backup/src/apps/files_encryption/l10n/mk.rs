use rust_i18n::t;

// Language file: mk (Macedonian)
pub fn register_translations() {
    rust_i18n::set_translation("mk", "Password successfully changed.", "Лозинката е успешно променета.");
    rust_i18n::set_translation("mk", "Could not change the password. Maybe the old password was not correct.", "Лозинката не можеше да се промени. Можеби старата лозинка не беше исправна.");
    rust_i18n::set_translation("mk", "Missing requirements.", "Барања кои недостасуваат.");
    rust_i18n::set_translation("mk", "Saving...", "Снимам...");
    rust_i18n::set_translation("mk", "Go directly to your ", "Одете директно на вашиот");
    rust_i18n::set_translation("mk", "personal settings", "лични подесувања");
    rust_i18n::set_translation("mk", "Encryption", "Енкрипција");
    rust_i18n::set_translation("mk", "Repeat Recovery key password", "Повтори ја лозинката за клучот на обновување");
    rust_i18n::set_translation("mk", "Enabled", "Овозможен");
    rust_i18n::set_translation("mk", "Disabled", "Оневозможен");
    rust_i18n::set_translation("mk", "Old Recovery key password", "Старата лозинка за клучот на обновување ");
    rust_i18n::set_translation("mk", "Repeat New Recovery key password", "Повтори ја лозинката за клучот на обновувањето");
    rust_i18n::set_translation("mk", "Change Password", "Смени лозинка");
    rust_i18n::set_translation("mk", "Old log-in password", "Старата лозинка за најавување");
    rust_i18n::set_translation("mk", "Current log-in password", "Тековната лозинка за најавување");
    rust_i18n::set_translation("mk", "Enable password recovery:", "Овозможи го обновувањето на лозинката:");
    
    // Set plural forms rule for Macedonian
    rust_i18n::set_plural_rule("mk", |n| {
        if n % 10 == 1 && n % 100 != 11 { 0 } else { 1 }
    });
}

// Function to initialize this translation module
pub fn init() {
    register_translations();
}