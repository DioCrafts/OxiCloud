use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Settings", "Instellings");
    m.insert("Password", "Wagwoord");
    m.insert("Use the following link to reset your password: {link}", "Gebruik die volgende skakel om jou wagwoord te herstel: {link}");
    m.insert("You will receive a link to reset your password via Email.", "Jy sal `n skakel via e-pos ontvang om jou wagwoord te herstel.");
    m.insert("Username", "Gebruikersnaam");
    m.insert("Your password was reset", "Jou wagwoord is herstel");
    m.insert("To login page", "Na aanteken-bladsy");
    m.insert("New password", "Nuwe wagwoord");
    m.insert("Reset password", "Herstel wagwoord");
    m.insert("Personal", "Persoonlik");
    m.insert("Users", "Gebruikers");
    m.insert("Apps", "Toepassings");
    m.insert("Admin", "Admin");
    m.insert("Help", "Hulp");
    m.insert("Cloud not found", "Wolk nie gevind");
    m.insert("Create an <strong>admin account</strong>", "Skep `n <strong>admin-rekening</strong>");
    m.insert("Advanced", "Gevorderd");
    m.insert("Configure the database", "Stel databasis op");
    m.insert("will be used", "sal gebruik word");
    m.insert("Database user", "Databasis-gebruiker");
    m.insert("Database password", "Databasis-wagwoord");
    m.insert("Database name", "Databasis naam");
    m.insert("Finish setup", "Maak opstelling klaar");
    m.insert("Log out", "Teken uit");
    m.insert("Lost your password?", "Jou wagwoord verloor?");
    m.insert("remember", "onthou");
    m.insert("Log in", "Teken aan");
    m
});

pub static PLURAL_FORMS: Lazy<HashMap<&'static str, Vec<&'static str>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("_%n minute ago_::_%n minutes ago_", vec!["", ""]);
    m.insert("_%n hour ago_::_%n hours ago_", vec!["", ""]);
    m.insert("_%n day ago_::_%n days ago_", vec!["", ""]);
    m.insert("_%n month ago_::_%n months ago_", vec!["", ""]);
    m.insert("_{count} file conflict_::_{count} file conflicts_", vec!["", ""]);
    m
});

pub static PLURAL_RULE: &'static str = "nplurals=2; plural=(n != 1);";

pub fn get_plural_index(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}