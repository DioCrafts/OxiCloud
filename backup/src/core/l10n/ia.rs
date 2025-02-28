use std::collections::HashMap;
use rust_i18n::i18n;

i18n!("ia");

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Sunday", "Dominica");
    translations.insert("Monday", "Lunedi");
    translations.insert("Tuesday", "Martedi");
    translations.insert("Wednesday", "Mercuridi");
    translations.insert("Thursday", "Jovedi");
    translations.insert("Friday", "Venerdi");
    translations.insert("Saturday", "Sabbato");
    translations.insert("January", "januario");
    translations.insert("February", "Februario");
    translations.insert("March", "Martio");
    translations.insert("April", "April");
    translations.insert("May", "Mai");
    translations.insert("June", "Junio");
    translations.insert("July", "Julio");
    translations.insert("August", "Augusto");
    translations.insert("September", "Septembre");
    translations.insert("October", "Octobre");
    translations.insert("November", "Novembre");
    translations.insert("December", "Decembre");
    translations.insert("Settings", "Configurationes");
    translations.insert("_%n minute ago_::_%n minutes ago_", "");
    translations.insert("_%n hour ago_::_%n hours ago_", "");
    translations.insert("_%n day ago_::_%n days ago_", "");
    translations.insert("_%n month ago_::_%n months ago_", "");
    translations.insert("_{count} file conflict_::_{count} file conflicts_", "");
    translations.insert("Cancel", "Cancellar");
    translations.insert("Share", "Compartir");
    translations.insert("Error", "Error");
    translations.insert("Password", "Contrasigno");
    translations.insert("Send", "Invia");
    translations.insert("Delete", "Deler");
    translations.insert("Add", "Adder");
    translations.insert("Username", "Nomine de usator");
    translations.insert("Your password was reset", "Tu contrasigno esseva reinitialisate");
    translations.insert("To login page", "al pagina de initio de session");
    translations.insert("New password", "Nove contrasigno");
    translations.insert("Reset password", "Reinitialisar contrasigno");
    translations.insert("Personal", "Personal");
    translations.insert("Users", "Usatores");
    translations.insert("Apps", "Applicationes");
    translations.insert("Admin", "Administration");
    translations.insert("Help", "Adjuta");
    translations.insert("Access forbidden", "Accesso prohibite");
    translations.insert("Cloud not found", "Nube non trovate");
    translations.insert("Create an <strong>admin account</strong>", "Crear un <strong>conto de administration</strong>");
    translations.insert("Advanced", "Avantiate");
    translations.insert("Data folder", "Dossier de datos");
    translations.insert("Configure the database", "Configurar le base de datos");
    translations.insert("will be used", "essera usate");
    translations.insert("Database user", "Usator de base de datos");
    translations.insert("Database password", "Contrasigno de base de datos");
    translations.insert("Database name", "Nomine de base de datos");
    translations.insert("Database host", "Hospite de base de datos");
    translations.insert("Log out", "Clauder le session");
    translations.insert("Lost your password?", "Tu perdeva le contrasigno?");
    translations.insert("remember", "memora");
    translations.insert("Log in", "Aperir session");
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn translate(key: &str) -> String {
    let translations = get_translations();
    translations.get(key).copied().unwrap_or(key).to_string()
}

pub fn translate_plural(key: &str, count: usize) -> String {
    // A simple implementation for plural forms
    // In a real application this would parse the plural formula
    let form = if count != 1 { 1 } else { 0 };
    
    // This is a simplified implementation
    // In a real app, you'd parse the plural key format properly
    if key.contains("::") {
        let parts: Vec<&str> = key.split("::").collect();
        if parts.len() >= 2 {
            let translations = get_translations();
            if let Some(plural_forms) = translations.get(key) {
                if !plural_forms.is_empty() {
                    return plural_forms.to_string();
                }
            }
            
            // Return appropriate form based on count
            if form == 0 {
                return parts[0].to_string().replace("_%n", &count.to_string());
            } else {
                return parts[1].to_string().replace("_%n", &count.to_string());
            }
        }
    }
    
    // Fallback: return the key with count substituted
    key.to_string().replace("_%n", &count.to_string())
}