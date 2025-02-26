use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Translations for Occitan (oc) language
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Unable to load list from App Store", "Pas possible de cargar la tièra dempuèi App Store");
    translations.insert("Authentication error", "Error d'autentificacion");
    translations.insert("Group already exists", "Lo grop existís ja");
    translations.insert("Unable to add group", "Pas capable d'apondre  un grop");
    translations.insert("Email saved", "Corrièl enregistrat");
    translations.insert("Invalid email", "Corrièl incorrècte");
    translations.insert("Unable to delete group", "Pas capable d'escafar un grop");
    translations.insert("Unable to delete user", "Pas capable d'escafar un usancièr");
    translations.insert("Language changed", "Lengas cambiadas");
    translations.insert("Invalid request", "Demanda invalida");
    translations.insert("Unable to add user to group %s", "Pas capable d'apondre un usancièr al grop %s");
    translations.insert("Unable to remove user from group %s", "Pas capable de tira un usancièr del grop %s");
    translations.insert("Disable", "Desactiva");
    translations.insert("Enable", "Activa");
    translations.insert("Error", "Error");
    translations.insert("Saving...", "Enregistra...");
    translations.insert("deleted", "escafat");
    translations.insert("undo", "defar");
    translations.insert("Groups", "Grops");
    translations.insert("Group Admin", "Grop Admin");
    translations.insert("Delete", "Escafa");
    translations.insert("__language_name__", "__language_name__");
    translations.insert("Security Warning", "Avertiment de securitat");
    translations.insert("Cron", "Cron");
    translations.insert("Execute one task with each page loaded", "Executa un prètfach amb cada pagina cargada");
    translations.insert("Sharing", "Al partejar");
    translations.insert("Enable Share API", "Activa API partejada");
    translations.insert("Log", "Jornal");
    translations.insert("More", "Mai d'aquò");
    translations.insert("Add your App", "Ajusta ton App");
    translations.insert("Select an App", "Selecciona una applicacion");
    translations.insert("See application page at apps.owncloud.com", "Agacha la pagina d'applications en cò de apps.owncloud.com");
    translations.insert("<span class=\"licence\"></span>-licensed by <span class=\"author\"></span>", "<span class=\"licence\"></span>-licençiat per <span class=\"author\"></span>");
    translations.insert("Password", "Senhal");
    translations.insert("Your password was changed", "Ton senhal a cambiat");
    translations.insert("Unable to change your password", "Pas possible de cambiar ton senhal");
    translations.insert("Current password", "Senhal en cors");
    translations.insert("New password", "Senhal novèl");
    translations.insert("Change password", "Cambia lo senhal");
    translations.insert("Email", "Corrièl");
    translations.insert("Your email address", "Ton adreiça de corrièl");
    translations.insert("Fill in an email address to enable password recovery", "Emplena una adreiça de corrièl per permetre lo mandadís del senhal perdut");
    translations.insert("Language", "Lenga");
    translations.insert("Help translate", "Ajuda a la revirada");
    translations.insert("Create", "Crea");
    translations.insert("Other", "Autres");
    translations.insert("Username", "Non d'usancièr");
    translations
});

/// Plural forms rule for Occitan language
pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n > 1);";

/// Get a translation for a given key
pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

/// Format a translation that contains placeholders
pub fn format_translation(key: &str, args: &[&str]) -> Option<String> {
    get_translation(key).map(|translation| {
        let mut result = translation.to_string();
        for (i, arg) in args.iter().enumerate() {
            result = result.replace(&format!("%s", i + 1), arg);
            // For compatibility with PHP's sprintf, also replace %s without an index
            if i == 0 {
                result = result.replace("%s", arg);
            }
        }
        result
    })
}