//! Albanian (sq) language translations for ownCloud

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Unable to load list from App Store", "E pamundur të shkarkohet lista nga App Store");
        m.insert("Authentication error", "Gabim autentifikimi");
        m.insert("Group already exists", "Grupi ekziston");
        m.insert("Unable to add group", "E pamundur të shtohet grupi");
        m.insert("Email saved", "Email u ruajt");
        m.insert("Invalid email", "Email jo i vlefshëm");
        m.insert("Unable to delete group", "E pamundur të fshihet grupi");
        m.insert("Unable to delete user", "E pamundur të fshihet përdoruesi");
        m.insert("Language changed", "Gjuha u ndryshua");
        m.insert("Invalid request", "Kërkesë e pavlefshme");
        m.insert("Admins can't remove themself from the admin group", "Administratorët nuk mund të heqin vehten prej grupit admin");
        m.insert("Unable to add user to group %s", "E pamundur t'i shtohet përdoruesi grupit %s");
        m.insert("Unable to remove user from group %s", "E pamundur të hiqet përdoruesi nga grupi %s");
        m.insert("Couldn't update app.", "E pamundur të përditësohet app.");
        m.insert("Update to {appversion}", "Përditësim për {appversion}");
        m.insert("Disable", "Çaktivizo");
        m.insert("Enable", "Aktivizo");
        m.insert("Please wait....", "Ju lutem prisni...");
        m.insert("Updating....", "Duke përditësuar...");
        m.insert("Error while updating app", "Gabim gjatë përditësimit të app");
        m.insert("Error", "Gabim");
        m.insert("Update", "Përditësim");
        m.insert("Updated", "I përditësuar");
        m.insert("Saving...", "Duke ruajtur...");
        m.insert("deleted", "fshirë");
        m.insert("undo", "anullo veprimin");
        m.insert("Unable to remove user", "E pamundur të fshiet përdoruesi");
        m.insert("Groups", "Grupet");
        m.insert("Group Admin", "Grupi Admin");
        m.insert("Delete", "Fshi");
        m.insert("add group", "shto grup");
        m.insert("A valid username must be provided", "Duhet të jepni një emër të vlefshëm përdoruesi");
        m.insert("Error creating user", "Gabim gjatë krijimit të përdoruesit");
        m.insert("A valid password must be provided", "Duhet të jepni një fjalëkalim te vlefshëm");
        m.insert("__language_name__", "Shqip");
        m.insert("Security Warning", "Njoftim për sigurinë");
        m.insert("Setup Warning", "Lajmërim konfigurimi");
        m.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Web Serveri juaj nuk është konfigurar sic duhet në mënyre që të lejojë sinkronizimin e skedare pasi ndërfaqja WevDAV duket të jetë e demtuar.");
        m.insert("Module 'fileinfo' missing", "Mungon moduli 'fileinfo'");
        m.insert("The PHP module 'fileinfo' is missing. We strongly recommend to enable this module to get best results with mime-type detection.", "Moduli PHP 'fileinfo' mungon. Ju këshillojmë me këmbngulje të aktivizoni këtë modul për të arritur rezultate më të mirame identifikimin e tipeve te ndryshme MIME.");
        m.insert("Locale not working", "Locale nuk është funksional");
        m.insert("Internet connection not working", "Lidhja me internetin nuk është funksionale");
        m.insert("Cron", "Cron");
        m.insert("Execute one task with each page loaded", "Kryeni vetëm një veprim me secilën prej faqeve të ngarkuara");
        m.insert("Sharing", "Ndarje");
        m.insert("Enable Share API", "Aktivizo API për ndarjet");
        m.insert("Allow apps to use the Share API", "Lejoni aplikacionet të përdorin share API");
        m.insert("Allow links", "Lejo lidhjet");
        m.insert("Allow users to share items to the public with links", "Lejoni përdoruesit të ndajnë elementët publikisht nëpermjet lidhjeve");
        m.insert("Allow public uploads", "Lejo ngarkimin publik");
        m.insert("Allow users to enable others to upload into their publicly shared folders", "Lejo përdoruesit të mundësojnë të tjerët që të ngarkojnë materiale në dosjen e tyre publike");
        m.insert("Allow resharing", "Lejo ri-ndarjen");
        m.insert("Allow users to share items shared with them again", "Lejoni përdoruesit të ndjanë dhe ata elementë të ndarë më parë ngë të tjerë");
        m.insert("Allow users to share with anyone", "Lejo përdoruesit të ndajnë me cilindo");
        m.insert("Allow users to only share with users in their groups", "Lejoni përdoruesit të ndajnë vetëm me përdorues të të njëjtit grup");
        m.insert("Security", "Siguria");
        m.insert("Enforce HTTPS", "Detyro HTTPS");
        m.insert("Log", "Historik aktiviteti");
        m.insert("Log level", "Niveli i Historikut");
        m.insert("More", "Më tepër");
        m.insert("Version", "Versioni");
        m.insert("Developed by the <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud community</a>, the <a href=\"https://github.com/owncloud\" target=\"_blank\">source code</a> is licensed under the <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.", "Zhvilluar nga <a href=\"http://ownCloud.org/contact\" target=\"_blank\">Komuniteti OwnCloud</a>, gjithashtu <a href=\"https://github.com/owncloud\" target=\"_blank\">source code</a> është licensuar me anë të <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.");
        m.insert("Add your App", "Shtoni apliakcionin tuaj");
        m.insert("More Apps", "Apliakcione të tjera");
        m.insert("Select an App", "Zgjidhni një Aplikacion");
        m.insert("See application page at apps.owncloud.com", "Shihni faqen e aplikacionit tek apps.owncloud.com");
        m.insert("<span class=\"licence\"></span>-licensed by <span class=\"author\"></span>", "<span class=\"licence\"></span>-licensuar nga <span class=\"author\"></span>");
        m.insert("User Documentation", "Dokumentacion përdoruesi");
        m.insert("Administrator Documentation", "Dokumentacion administratori");
        m.insert("Online Documentation", "Dokumentacion online");
        m.insert("Forum", "Forumi");
        m.insert("Bugtracker", "Bugtracker - ndjekja e problemeve");
        m.insert("Commercial Support", "Suport komercial");
        m.insert("Get the apps to sync your files", "Bëni që aplikacionet të sinkronizojnë skedarët tuaj");
        m.insert("Show First Run Wizard again", "Rishfaq përsëri fazat për hapjen e herës së parë");
        m.insert("You have used <strong>%s</strong> of the available <strong>%s</strong>", "Ju keni përdorur <strong>%s</strong> nga <strong>%s</strong> të mundshme ");
        m.insert("Password", "Fjalëkalim");
        m.insert("Your password was changed", "fjalëkalimi juaj u ndryshua");
        m.insert("Unable to change your password", "Nuk është e mundur të ndryshohet fjalëkalimi");
        m.insert("Current password", "Fjalëkalimi aktual");
        m.insert("New password", "Fjalëkalimi i ri");
        m.insert("Change password", "Ndrysho fjalëkalimin");
        m.insert("Email", "Email");
        m.insert("Your email address", "Adresa juaj email");
        m.insert("Fill in an email address to enable password recovery", "Jepni një adresë email për të aktivizuar rigjetjen e fjalëkalimit");
        m.insert("Language", "Gjuha");
        m.insert("Help translate", "Ndihmoni në përkthim");
        m.insert("WebDAV", "WebDAV");
        m.insert("Login Name", "Emri i Përdoruesit");
        m.insert("Create", "Krijo");
        m.insert("Admin Recovery Password", "Rigjetja e fjalëkalimit të Admin");
        m.insert("Enter the recovery password in order to recover the users files during password change", "Jepni fjalëkalimin e rigjetjes për të rigjetur skedarët e përdoruesit gjatë ndryshimit të fjalëkalimit");
        m.insert("Default Storage", "Vendruajtje e paracaktuar/Default Storage");
        m.insert("Unlimited", "E pakufizuar");
        m.insert("Other", "Tjetër");
        m.insert("Username", "Përdoruesi");
        m.insert("Storage", "Vendruajtja/Storage");
        m.insert("set new password", "vendos fjalëkalim të ri");
        m.insert("Default", "Paracaktuar");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

/// Returns the translation for the given key
pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

/// Returns the plural forms expression for this language
pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}