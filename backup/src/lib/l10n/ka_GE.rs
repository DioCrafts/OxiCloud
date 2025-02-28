use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Help", "დახმარება");
        m.insert("Personal", "პირადი");
        m.insert("Settings", "პარამეტრები");
        m.insert("Users", "მომხმარებელი");
        m.insert("Admin", "ადმინისტრატორი");
        m.insert("web services under your control", "web services under your control");
        m.insert("ZIP download is turned off.", "ZIP download–ი გათიშულია");
        m.insert("Files need to be downloaded one by one.", "ფაილები უნდა გადმოიტვირთოს სათითაოდ.");
        m.insert("Back to Files", "უკან ფაილებში");
        m.insert("Selected files too large to generate zip file.", "არჩეული ფაილები ძალიან დიდია zip ფაილის გენერაციისთვის.");
        m.insert("Application is not enabled", "აპლიკაცია არ არის აქტიური");
        m.insert("Authentication error", "ავთენტიფიკაციის შეცდომა");
        m.insert("Token expired. Please reload page.", "Token–ს ვადა გაუვიდა. გთხოვთ განაახლოთ გვერდი.");
        m.insert("Files", "ფაილები");
        m.insert("Text", "ტექსტი");
        m.insert("Images", "სურათები");
        m.insert("%s enter the database username.", "%s შეიყვანეთ ბაზის იუზერნეიმი.");
        m.insert("%s enter the database name.", "%s შეიყვანეთ ბაზის სახელი.");
        m.insert("%s you may not use dots in the database name", "%s არ მიუთითოთ წერტილი ბაზის სახელში");
        m.insert("MS SQL username and/or password not valid: %s", "MS SQL მომხმარებელი და/ან პაროლი არ არის მართებული: %s");
        m.insert("You need to enter either an existing account or the administrator.", "თქვენ უნდა შეიყვანოთ არსებული მომხმარებელის სახელი ან ადმინისტრატორი.");
        m.insert("MySQL username and/or password not valid", "MySQL იუზერნეიმი და/ან პაროლი არ არის სწორი");
        m.insert("DB Error: \"%s\"", "DB შეცდომა: \"%s\"");
        m.insert("Offending command was: \"%s\"", "Offending ბრძანება იყო: \"%s\"");
        m.insert("MySQL user '%s'@'localhost' exists already.", "MySQL მომხმარებელი '%s'@'localhost' უკვე არსებობს.");
        m.insert("Drop this user from MySQL", "წაშალე ეს მომხამრებელი MySQL–იდან");
        m.insert("MySQL user '%s'@'%%' already exists", "MySQL მომხმარებელი '%s'@'%%' უკვე არსებობს");
        m.insert("Drop this user from MySQL.", "წაშალე ეს მომხამრებელი MySQL–იდან");
        m.insert("Oracle username and/or password not valid", "Oracle იუზერნეიმი და/ან პაროლი არ არის სწორი");
        m.insert("Offending command was: \"%s\", name: %s, password: %s", "Offending ბრძანება იყო: \"%s\", სახელი: %s, პაროლი: %s");
        m.insert("PostgreSQL username and/or password not valid", "PostgreSQL იუზერნეიმი და/ან პაროლი არ არის სწორი");
        m.insert("Set an admin username.", "დააყენეთ ადმინისტრატორის სახელი.");
        m.insert("Set an admin password.", "დააყენეთ ადმინისტრატორის პაროლი.");
        m.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "თქვენი web სერვერი არ არის კონფიგურირებული ფაილ სინქრონიზაციისთვის, რადგან WebDAV ინტერფეისი შეიძლება იყოს გატეხილი.");
        m.insert("Please double check the <a href='%s'>installation guides</a>.", "გთხოვთ გადაათვალიეროთ <a href='%s'>ინსტალაციის გზამკვლევი</a>.");
        m.insert("Could not find category \"%s\"", "\"%s\" კატეგორიის მოძებნა ვერ მოხერხდა");
        m.insert("seconds ago", "წამის წინ");
        m.insert("_%n minute ago_::_%n minutes ago_", "");
        m.insert("_%n hour ago_::_%n hours ago_", "");
        m.insert("today", "დღეს");
        m.insert("yesterday", "გუშინ");
        m.insert("_%n day go_::_%n days ago_", "");
        m.insert("last month", "გასულ თვეში");
        m.insert("_%n month ago_::_%n months ago_", "");
        m.insert("last year", "ბოლო წელს");
        m.insert("years ago", "წლის წინ");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}

pub struct Ka_GE;

impl Ka_GE {
    pub fn get_translation(&self, key: &str) -> Option<&'static str> {
        TRANSLATIONS.get(key).copied()
    }

    pub fn get_plural_form(&self) -> &'static str {
        &PLURAL_FORMS
    }
}