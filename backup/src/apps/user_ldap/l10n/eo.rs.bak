use std::collections::HashMap;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Failed to delete the server configuration", "Malsukcesis forigo de la agordo de servilo");
        m.insert("Deletion failed", "Forigo malsukcesis");
        m.insert("Keep settings?", "Ĉu daŭrigi la agordon?");
        m.insert("Cannot add server configuration", "Ne eblas aldoni agordon de servilo");
        m.insert("Success", "Sukceso");
        m.insert("Error", "Eraro");
        m.insert("Select groups", "Elekti grupojn");
        m.insert("Select object classes", "Elekti objektoklasojn");
        m.insert("Select attributes", "Elekti atribuojn");
        m.insert("Connection test succeeded", "Provo de konekto sukcesis");
        m.insert("Connection test failed", "Provo de konekto malsukcesis");
        m.insert("Confirm Deletion", "Konfirmi forigon");
        m.insert("_%s group found_::_%s groups found_", "%s grupo troviĝis|%s grupoj troviĝis");
        m.insert("_%s user found_::_%s users found_", "%s uzanto troviĝis|%s uzanto troviĝis");
        m.insert("Invalid Host", "Nevalida gastigo");
        m.insert("Save", "Konservi");
        m.insert("Test Configuration", "Provi agordon");
        m.insert("Help", "Helpo");
        m.insert("only those object classes:", "nur tiuj objektoklasoj:");
        m.insert("only from those groups:", "nur el tiuj grupoj:");
        m.insert("groups found", "grupoj trovitaj");
        m.insert("LDAP Username:", "LDAP-uzantonomo:");
        m.insert("LDAP Email Address:", "LDAP-retpoŝtadreso:");
        m.insert("Other Attributes:", "Aliaj atribuoj:");
        m.insert("Add Server Configuration", "Aldoni agordon de servilo");
        m.insert("Host", "Gastigo");
        m.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "Vi povas neglekti la protokolon, escepte se vi bezonas SSL-on. Tiuokaze, komencu per ldaps://");
        m.insert("Port", "Pordo");
        m.insert("User DN", "Uzanto-DN");
        m.insert("Password", "Pasvorto");
        m.insert("For anonymous access, leave DN and Password empty.", "Por sennoman aliron, lasu DN-on kaj Pasvorton malplenaj.");
        m.insert("users found", "uzantoj trovitaj");
        m.insert("Back", "Antaŭen");
        m.insert("Connection Settings", "Agordo de konekto");
        m.insert("User Login Filter", "Filtrilo de uzantensaluto");
        m.insert("Disable Main Server", "Malkapabligi la ĉefan servilon");
        m.insert("Case insensitve LDAP server (Windows)", "LDAP-servilo blinda je litergrandeco (Vindozo)");
        m.insert("Turn off SSL certificate validation.", "Malkapabligi validkontrolon de SSL-atestiloj.");
        m.insert("Cache Time-To-Live", "Vivotempo de la kaŝmemoro");
        m.insert("in seconds. A change empties the cache.", "sekunde. Ajna ŝanĝo malplenigas la kaŝmemoron.");
        m.insert("User Display Name Field", "Kampo de vidignomo de uzanto");
        m.insert("Base User Tree", "Baza uzantarbo");
        m.insert("Group Display Name Field", "Kampo de vidignomo de grupo");
        m.insert("Base Group Tree", "Baza gruparbo");
        m.insert("Group Search Attributes", "Atribuoj de gruposerĉo");
        m.insert("Group-Member association", "Asocio de grupo kaj membro");
        m.insert("Special Attributes", "Specialaj atribuoj");
        m.insert("Quota Field", "Kampo de kvoto");
        m.insert("in bytes", "duumoke");
        m.insert("Email Field", "Kampo de retpoŝto");
        m.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "Lasu malplena por uzantonomo (defaŭlto). Alie, specifu LDAP/AD-atributon.");
        m.insert("Internal Username", "Ena uzantonomo");
        m.insert("Internal Username Attribute:", "Atribuo de ena uzantonomo:");
        m.insert("UUID Attribute for Users:", "UUID-atribuo por uzantoj:");
        m.insert("UUID Attribute for Groups:", "UUID-atribuo por grupoj:");
        m
    };
}

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

pub fn init() {
    rust_i18n::set_locale("eo");
}

// Función de ayuda para pluralización
pub fn ngettext(singular: &str, plural: &str, n: i64) -> String {
    if n != 1 {
        return plural.to_string();
    }
    singular.to_string()
}