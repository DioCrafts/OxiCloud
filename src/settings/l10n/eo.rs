use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Unable to load list from App Store", "Ne eblis ŝargi liston el aplikaĵovendejo");
        m.insert("Authentication error", "Aŭtentiga eraro");
        m.insert("Group already exists", "La grupo jam ekzistas");
        m.insert("Unable to add group", "Ne eblis aldoni la grupon");
        m.insert("Email saved", "La retpoŝtadreso konserviĝis");
        m.insert("Invalid email", "Nevalida retpoŝtadreso");
        m.insert("Unable to delete group", "Ne eblis forigi la grupon");
        m.insert("Unable to delete user", "Ne eblis forigi la uzanton");
        m.insert("Language changed", "La lingvo estas ŝanĝita");
        m.insert("Invalid request", "Nevalida peto");
        m.insert("Admins can't remove themself from the admin group", "Administrantoj ne povas forigi sin mem el la administra grupo.");
        m.insert("Unable to add user to group %s", "Ne eblis aldoni la uzanton al la grupo %s");
        m.insert("Unable to remove user from group %s", "Ne eblis forigi la uzantan el la grupo %s");
        m.insert("Disable", "Malkapabligi");
        m.insert("Enable", "Kapabligi");
        m.insert("Error", "Eraro");
        m.insert("Update", "Ĝisdatigi");
        m.insert("Saving...", "Konservante...");
        m.insert("deleted", "forigita");
        m.insert("undo", "malfari");
        m.insert("Groups", "Grupoj");
        m.insert("Group Admin", "Grupadministranto");
        m.insert("Delete", "Forigi");
        m.insert("__language_name__", "Esperanto");
        m.insert("Security Warning", "Sekureca averto");
        m.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Via TTT-servilo ankoraŭ ne ĝuste agordiĝis por permesi sinkronigi dosierojn ĉar la WebDAV-interfaco ŝajnas rompita.");
        m.insert("Cron", "Cron");
        m.insert("Sharing", "Kunhavigo");
        m.insert("Enable Share API", "Kapabligi API-on por Kunhavigo");
        m.insert("Allow apps to use the Share API", "Kapabligi aplikaĵojn uzi la API-on pri Kunhavigo");
        m.insert("Allow links", "Kapabligi ligilojn");
        m.insert("Allow users to share items to the public with links", "Kapabligi uzantojn kunhavigi erojn kun la publiko perligile");
        m.insert("Allow resharing", "Kapabligi rekunhavigon");
        m.insert("Allow users to share items shared with them again", "Kapabligi uzantojn rekunhavigi erojn kunhavigitajn kun ili");
        m.insert("Allow users to share with anyone", "Kapabligi uzantojn kunhavigi kun ĉiu ajn");
        m.insert("Allow users to only share with users in their groups", "Kapabligi uzantojn nur kunhavigi kun uzantoj el siaj grupoj");
        m.insert("Log", "Protokolo");
        m.insert("Log level", "Registronivelo");
        m.insert("More", "Pli");
        m.insert("Less", "Malpli");
        m.insert("Version", "Eldono");
        m.insert("Developed by the <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud community</a>, the <a href=\"https://github.com/owncloud\" target=\"_blank\">source code</a> is licensed under the <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.", "Ellaborita de la <a href=\"http://ownCloud.org/contact\" target=\"_blank\">komunumo de ownCloud</a>, la <a href=\"https://github.com/owncloud\" target=\"_blank\">fontokodo</a> publikas laŭ la permesilo <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.");
        m.insert("Add your App", "Aldonu vian aplikaĵon");
        m.insert("More Apps", "Pli da aplikaĵoj");
        m.insert("Select an App", "Elekti aplikaĵon");
        m.insert("See application page at apps.owncloud.com", "Vidu la paĝon pri aplikaĵoj ĉe apps.owncloud.com");
        m.insert("<span class=\"licence\"></span>-licensed by <span class=\"author\"></span>", "<span class=\"licence\"</span>-permesilhavigita de <span class=\"author\"></span>");
        m.insert("User Documentation", "Dokumentaro por uzantoj");
        m.insert("Administrator Documentation", "Dokumentaro por administrantoj");
        m.insert("Online Documentation", "Reta dokumentaro");
        m.insert("Forum", "Forumo");
        m.insert("Bugtracker", "Cimoraportejo");
        m.insert("Commercial Support", "Komerca subteno");
        m.insert("Get the apps to sync your files", "Ekhavu la aplikaĵojn por sinkronigi viajn dosierojn");
        m.insert("You have used <strong>%s</strong> of the available <strong>%s</strong>", "Vi uzas <strong>%s</strong> el la haveblaj <strong>%s</strong>");
        m.insert("Password", "Pasvorto");
        m.insert("Your password was changed", "Via pasvorto ŝanĝiĝis");
        m.insert("Unable to change your password", "Ne eblis ŝanĝi vian pasvorton");
        m.insert("Current password", "Nuna pasvorto");
        m.insert("New password", "Nova pasvorto");
        m.insert("Change password", "Ŝanĝi la pasvorton");
        m.insert("Email", "Retpoŝto");
        m.insert("Your email address", "Via retpoŝta adreso");
        m.insert("Fill in an email address to enable password recovery", "Enigu retpoŝtadreson por kapabligi pasvortan restaŭron");
        m.insert("Profile picture", "Profila bildo");
        m.insert("Language", "Lingvo");
        m.insert("Help translate", "Helpu traduki");
        m.insert("WebDAV", "WebDAV");
        m.insert("Encryption", "Ĉifrado");
        m.insert("Create", "Krei");
        m.insert("Default Storage", "Defaŭlta konservejo");
        m.insert("Unlimited", "Senlima");
        m.insert("Other", "Alia");
        m.insert("Username", "Uzantonomo");
        m.insert("Storage", "Konservejo");
        m.insert("Default", "Defaŭlta");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}