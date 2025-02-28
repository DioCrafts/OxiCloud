use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Access granted", "დაშვება მინიჭებულია");
        m.insert("Error configuring Dropbox storage", "შეცდომა Dropbox საცავის კონფიგურირების დროს");
        m.insert("Grant access", "დაშვების მინიჭება");
        m.insert("Please provide a valid Dropbox app key and secret.", "გთხოვთ მიუთითოთ Dropbox აპლიკაციის გასაღები და კოდი.");
        m.insert("Error configuring Google Drive storage", "შეცდომა Google Drive საცავის კონფიგურირების დროს");
        m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>გაფრთხილება:</b> \"smbclient\" არ არის ინსტალირებული. CIFS/SMB ზიარების მონტირება შეუძლებელია. გთხოვთ თხოვოთ თქვენს სისტემურ ადმინისტრატორებს დააინსტალიროს ის.");
        m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>გაფრთხილება:</b>  FTP მხარდაჭერა არ არის აქტიური ან დაინსტალირებული. FTP ზიარის მონტირება შეუძლებელია. გთხოვთ თხოვოთ თქვენს სისტემურ ადმინისტრატორებს დააინსტალიროს ის.");
        m.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>გაფრთხილება:</b>PHP–ის Curl მხარდაჭერა არ არის ჩართული ან ინსტალირებული. ownCloud / WebDAV ან GoogleDrive–ის მონტირება შეუძლებელია. თხოვეთ თქვენს ადმინისტრატორს დააინსტალიროს ის.");
        m.insert("External Storage", "ექსტერნალ საცავი");
        m.insert("Folder name", "ფოლდერის სახელი");
        m.insert("External storage", "ექსტერნალ საცავი");
        m.insert("Configuration", "კონფიგურაცია");
        m.insert("Options", "ოფცია");
        m.insert("Applicable", "მიღებადი");
        m.insert("Add storage", "საცავის დამატება");
        m.insert("None set", "არაფერია მითითებული");
        m.insert("All Users", "ყველა მომხმარებელი");
        m.insert("Groups", "ჯგუფები");
        m.insert("Users", "მომხმარებელი");
        m.insert("Delete", "წაშლა");
        m.insert("Enable User External Storage", "მომხმარებლის ექსტერნალ საცავის აქტივირება");
        m.insert("Allow users to mount their own external storage", "მიეცით მომხმარებლებს თავისი ექსტერნალ საცავის მონტირების  უფლება");
        m.insert("SSL root certificates", "SSL root სერთიფიკატები");
        m.insert("Import Root Certificate", "Root სერთიფიკატის იმპორტირება");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}