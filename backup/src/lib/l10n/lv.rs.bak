use std::collections::HashMap;
use rust_i18n::i18n;

// Definición de la estructura de traducción para letonio (lv)
pub struct LvTranslations;

impl LvTranslations {
    pub fn get_translations() -> HashMap<String, String> {
        let mut translations = HashMap::new();
        
        translations.insert("Help".to_string(), "Palīdzība".to_string());
        translations.insert("Personal".to_string(), "Personīgi".to_string());
        translations.insert("Settings".to_string(), "Iestatījumi".to_string());
        translations.insert("Users".to_string(), "Lietotāji".to_string());
        translations.insert("Admin".to_string(), "Administratori".to_string());
        translations.insert("Failed to upgrade \"%s\".".to_string(), "Kļūda atjauninot \"%s\"".to_string());
        translations.insert("web services under your control".to_string(), "tīmekļa servisi tavā varā".to_string());
        translations.insert("cannot open \"%s\"".to_string(), "Nevar atvērt \"%s\"".to_string());
        translations.insert("ZIP download is turned off.".to_string(), "ZIP lejupielādēšana ir izslēgta.".to_string());
        translations.insert("Files need to be downloaded one by one.".to_string(), "Datnes var lejupielādēt tikai katru atsevišķi.".to_string());
        translations.insert("Back to Files".to_string(), "Atpakaļ pie datnēm".to_string());
        translations.insert("Selected files too large to generate zip file.".to_string(), "Izvēlētās datnes ir pārāk lielas, lai izveidotu zip datni.".to_string());
        translations.insert("Download the files in smaller chunks, seperately or kindly ask your administrator.".to_string(), "Lejupielādējiet savus failus mazākās daļās, atsevišķi vai palūdziet tos administratoram.".to_string());
        translations.insert("Application is not enabled".to_string(), "Lietotne nav aktivēta".to_string());
        translations.insert("Authentication error".to_string(), "Autentifikācijas kļūda".to_string());
        translations.insert("Token expired. Please reload page.".to_string(), "Pilnvarai ir beidzies termiņš. Lūdzu, pārlādējiet lapu.".to_string());
        translations.insert("Files".to_string(), "Datnes".to_string());
        translations.insert("Text".to_string(), "Teksts".to_string());
        translations.insert("Images".to_string(), "Attēli".to_string());
        translations.insert("%s enter the database username.".to_string(), "%s ievadiet datubāzes lietotājvārdu.".to_string());
        translations.insert("%s enter the database name.".to_string(), "%s ievadiet datubāzes nosaukumu.".to_string());
        translations.insert("%s you may not use dots in the database name".to_string(), "%s datubāžu nosaukumos nedrīkst izmantot punktus".to_string());
        translations.insert("MS SQL username and/or password not valid: %s".to_string(), "Nav derīga MySQL parole un/vai lietotājvārds — %s".to_string());
        translations.insert("You need to enter either an existing account or the administrator.".to_string(), "Jums jāievada vai nu esošs vai administratora konts.".to_string());
        translations.insert("MySQL username and/or password not valid".to_string(), "Nav derīga MySQL parole un/vai lietotājvārds".to_string());
        translations.insert("DB Error: \"%s\"".to_string(), "DB kļūda — "%s"".to_string());
        translations.insert("Offending command was: \"%s\"".to_string(), "Vainīgā komanda bija "%s"".to_string());
        translations.insert("MySQL user '%s'@'localhost' exists already.".to_string(), "MySQL lietotājs %s'@'localhost' jau eksistē.".to_string());
        translations.insert("Drop this user from MySQL".to_string(), "Izmest šo lietotāju no MySQL".to_string());
        translations.insert("MySQL user '%s'@'%%' already exists".to_string(), "MySQL lietotājs '%s'@'%%' jau eksistē".to_string());
        translations.insert("Drop this user from MySQL.".to_string(), "Izmest šo lietotāju no MySQL.".to_string());
        translations.insert("Oracle connection could not be established".to_string(), "Nevar izveidot savienojumu ar Oracle".to_string());
        translations.insert("Oracle username and/or password not valid".to_string(), "Nav derīga Oracle parole un/vai lietotājvārds".to_string());
        translations.insert("Offending command was: \"%s\", name: %s, password: %s".to_string(), "Vainīgā komanda bija \"%s\", vārds: %s, parole: %s".to_string());
        translations.insert("PostgreSQL username and/or password not valid".to_string(), "Nav derīga PostgreSQL parole un/vai lietotājvārds".to_string());
        translations.insert("Set an admin username.".to_string(), "Iestatiet administratora lietotājvārdu.".to_string());
        translations.insert("Set an admin password.".to_string(), "Iestatiet administratora paroli.".to_string());
        translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.".to_string(), "Jūsu serveris vēl nav pareizi iestatīts, lai ļautu sinhronizēt datnes, jo izskatās, ka WebDAV saskarne ir salauzta.".to_string());
        translations.insert("Please double check the <a href='%s'>installation guides</a>.".to_string(), "Lūdzu, vēlreiz pārbaudiet <a href='%s'>instalēšanas palīdzību</a>.".to_string());
        translations.insert("Could not find category \"%s\"".to_string(), "Nevarēja atrast kategoriju "%s"".to_string());
        translations.insert("seconds ago".to_string(), "sekundes atpakaļ".to_string());
        translations.insert("today".to_string(), "šodien".to_string());
        translations.insert("yesterday".to_string(), "vakar".to_string());
        translations.insert("last month".to_string(), "pagājušajā mēnesī".to_string());
        translations.insert("last year".to_string(), "gājušajā gadā".to_string());
        translations.insert("years ago".to_string(), "gadus atpakaļ".to_string());
        translations.insert("Caused by:".to_string(), "Cēlonis:".to_string());
        
        translations
    }
    
    pub fn get_plural_forms() -> &'static str {
        "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n != 0 ? 1 : 2);"
    }
    
    pub fn get_plural_translations() -> HashMap<String, Vec<String>> {
        let mut plural_translations = HashMap::new();
        
        plural_translations.insert(
            "_%n minute ago_::_%n minutes ago_".to_string(), 
            vec!["".to_string(), "".to_string(), "Pirms %n minūtēm".to_string()]
        );
        
        plural_translations.insert(
            "_%n hour ago_::_%n hours ago_".to_string(), 
            vec!["".to_string(), "".to_string(), "Pirms %n stundām".to_string()]
        );
        
        plural_translations.insert(
            "_%n day go_::_%n days ago_".to_string(), 
            vec!["".to_string(), "".to_string(), "Pirms %n dienām".to_string()]
        );
        
        plural_translations.insert(
            "_%n month ago_::_%n months ago_".to_string(), 
            vec!["".to_string(), "".to_string(), "Pirms %n mēnešiem".to_string()]
        );
        
        plural_translations
    }
}

impl i18n::Translations for LvTranslations {
    fn locale(&self) -> &str {
        "lv"
    }
    
    fn translate(&self, key: &str) -> Option<String> {
        Self::get_translations().get(key).cloned()
    }
    
    fn translate_plural(&self, key: &str, count: usize) -> Option<String> {
        if let Some(forms) = Self::get_plural_translations().get(key) {
            let plural_idx = self.get_plural_index(count);
            if plural_idx < forms.len() {
                return Some(forms[plural_idx].replace("%n", &count.to_string()));
            }
        }
        None
    }
    
    fn get_plural_index(&self, n: usize) -> usize {
        // Implementar la fórmula de pluralización:
        // nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n != 0 ? 1 : 2);
        let n = n as u64;
        if n % 10 == 1 && n % 100 != 11 {
            0
        } else if n != 0 {
            1
        } else {
            2
        }
    }
}