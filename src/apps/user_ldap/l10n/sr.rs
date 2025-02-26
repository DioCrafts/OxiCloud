use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Deletion failed", "Брисање није успело");
        m.insert("Error", "Грешка");
        m.insert("Save", "Сачувај");
        m.insert("Help", "Помоћ");
        m.insert("Host", "Домаћин");
        m.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", 
                "Можете да изоставите протокол, осим ако захтевате SSL. У том случају почните са ldaps://.");
        m.insert("Port", "Порт");
        m.insert("User DN", "Корисник DN");
        m.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", 
                "DN корисника клијента са којим треба да се успостави веза, нпр. uid=agent,dc=example,dc=com. За анониман приступ, оставите поља DN и лозинка празним.");
        m.insert("Password", "Лозинка");
        m.insert("For anonymous access, leave DN and Password empty.", 
                "За анониман приступ, оставите поља DN и лозинка празним.");
        m.insert("Back", "Назад");
        m.insert("User Login Filter", "Филтер за пријаву корисника");
        m.insert("Case insensitve LDAP server (Windows)", 
                "LDAP сервер осетљив на велика и мала слова (Windows)");
        m.insert("Turn off SSL certificate validation.", 
                "Искључите потврду SSL сертификата.");
        m.insert("in seconds. A change empties the cache.", 
                "у секундама. Промена испражњава кеш меморију.");
        m.insert("User Display Name Field", "Име приказа корисника");
        m.insert("Base User Tree", "Основно стабло корисника");
        m.insert("Group Display Name Field", "Име приказа групе");
        m.insert("Base Group Tree", "Основна стабло група");
        m.insert("Group-Member association", "Придруживање чланова у групу");
        m.insert("in bytes", "у бајтовима");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";
    
    // Plural translations
    pub static ref PLURAL_TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%s group found_::_%s groups found_", vec!["", "", ""]);
        m.insert("_%s user found_::_%s users found_", vec!["", "", ""]);
        m
    };
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_translation(key: &str, count: i64) -> Option<&'static str> {
    if let Some(forms) = PLURAL_TRANSLATIONS.get(key) {
        // Calculate the plural form based on PLURAL_FORMS formula
        let plural_index = if count % 10 == 1 && count % 100 != 11 {
            0
        } else if count % 10 >= 2 && count % 10 <= 4 && (count % 100 < 10 || count % 100 >= 20) {
            1
        } else {
            2
        };
        
        forms.get(plural_index).copied()
    } else {
        None
    }
}