// oc.rs

use std::collections::HashMap;
use rust_i18n::i18n;

#[derive(Debug, Clone)]
pub struct OcTranslations {
    translations: HashMap<String, String>,
    plural_forms: String,
}

impl OcTranslations {
    pub fn new() -> Self {
        let mut translations = HashMap::new();
        
        translations.insert("Sunday".to_string(), "Dimenge".to_string());
        translations.insert("Monday".to_string(), "Diluns".to_string());
        translations.insert("Tuesday".to_string(), "Dimarç".to_string());
        translations.insert("Wednesday".to_string(), "Dimecres".to_string());
        translations.insert("Thursday".to_string(), "Dijòus".to_string());
        translations.insert("Friday".to_string(), "Divendres".to_string());
        translations.insert("Saturday".to_string(), "Dissabte".to_string());
        translations.insert("January".to_string(), "genièr".to_string());
        translations.insert("February".to_string(), "febrièr".to_string());
        translations.insert("March".to_string(), "març".to_string());
        translations.insert("April".to_string(), "abril".to_string());
        translations.insert("May".to_string(), "mai".to_string());
        translations.insert("June".to_string(), "junh".to_string());
        translations.insert("July".to_string(), "julhet".to_string());
        translations.insert("August".to_string(), "agost".to_string());
        translations.insert("September".to_string(), "septembre".to_string());
        translations.insert("October".to_string(), "octobre".to_string());
        translations.insert("November".to_string(), "Novembre".to_string());
        translations.insert("December".to_string(), "Decembre".to_string());
        translations.insert("Settings".to_string(), "Configuracion".to_string());
        translations.insert("seconds ago".to_string(), "segonda a".to_string());
        translations.insert("_%n minute ago_::_%n minutes ago_".to_string(), "".to_string());
        translations.insert("_%n hour ago_::_%n hours ago_".to_string(), "".to_string());
        translations.insert("today".to_string(), "uèi".to_string());
        translations.insert("yesterday".to_string(), "ièr".to_string());
        translations.insert("_%n day ago_::_%n days ago_".to_string(), "".to_string());
        translations.insert("last month".to_string(), "mes passat".to_string());
        translations.insert("_%n month ago_::_%n months ago_".to_string(), "".to_string());
        translations.insert("months ago".to_string(), "meses  a".to_string());
        translations.insert("last year".to_string(), "an passat".to_string());
        translations.insert("years ago".to_string(), "ans a".to_string());
        translations.insert("Choose".to_string(), "Causís".to_string());
        translations.insert("Yes".to_string(), "Òc".to_string());
        translations.insert("No".to_string(), "Non".to_string());
        translations.insert("Ok".to_string(), "D'accòrdi".to_string());
        translations.insert("_{count} file conflict_::_{count} file conflicts_".to_string(), "".to_string());
        translations.insert("Cancel".to_string(), "Annula".to_string());
        translations.insert("Share".to_string(), "Parteja".to_string());
        translations.insert("Error".to_string(), "Error".to_string());
        translations.insert("Error while sharing".to_string(), "Error al partejar".to_string());
        translations.insert("Error while unsharing".to_string(), "Error al non partejar".to_string());
        translations.insert("Error while changing permissions".to_string(), "Error al cambiar permissions".to_string());
        translations.insert("Password protect".to_string(), "Parat per senhal".to_string());
        translations.insert("Password".to_string(), "Senhal".to_string());
        translations.insert("Set expiration date".to_string(), "Met la data d'expiracion".to_string());
        translations.insert("Expiration date".to_string(), "Data d'expiracion".to_string());
        translations.insert("Share via email:".to_string(), "Parteja tras corrièl :".to_string());
        translations.insert("No people found".to_string(), "Deguns trobat".to_string());
        translations.insert("group".to_string(), "grop".to_string());
        translations.insert("Resharing is not allowed".to_string(), "Tornar partejar es pas permis".to_string());
        translations.insert("Unshare".to_string(), "Pas partejador".to_string());
        translations.insert("can edit".to_string(), "pòt modificar".to_string());
        translations.insert("access control".to_string(), "Contraròtle d'acces".to_string());
        translations.insert("create".to_string(), "crea".to_string());
        translations.insert("update".to_string(), "met a jorn".to_string());
        translations.insert("delete".to_string(), "escafa".to_string());
        translations.insert("share".to_string(), "parteja".to_string());
        translations.insert("Password protected".to_string(), "Parat per senhal".to_string());
        translations.insert("Error unsetting expiration date".to_string(), "Error al metre de la data d'expiracion".to_string());
        translations.insert("Error setting expiration date".to_string(), "Error setting expiration date".to_string());
        translations.insert("Delete".to_string(), "Escafa".to_string());
        translations.insert("Add".to_string(), "Ajusta".to_string());
        translations.insert("Use the following link to reset your password: {link}".to_string(), "Utiliza lo ligam seguent per tornar botar lo senhal : {link}".to_string());
        translations.insert("You will receive a link to reset your password via Email.".to_string(), "Reçaupràs un ligam per tornar botar ton senhal via corrièl.".to_string());
        translations.insert("Username".to_string(), "Non d'usancièr".to_string());
        translations.insert("Your password was reset".to_string(), "Ton senhal es estat tornat botar".to_string());
        translations.insert("To login page".to_string(), "Pagina cap al login".to_string());
        translations.insert("New password".to_string(), "Senhal novèl".to_string());
        translations.insert("Reset password".to_string(), "Senhal tornat botar".to_string());
        translations.insert("Personal".to_string(), "Personal".to_string());
        translations.insert("Users".to_string(), "Usancièrs".to_string());
        translations.insert("Apps".to_string(), "Apps".to_string());
        translations.insert("Admin".to_string(), "Admin".to_string());
        translations.insert("Help".to_string(), "Ajuda".to_string());
        translations.insert("Access forbidden".to_string(), "Acces enebit".to_string());
        translations.insert("Cloud not found".to_string(), "Nívol pas trobada".to_string());
        translations.insert("Security Warning".to_string(), "Avertiment de securitat".to_string());
        translations.insert("Create an <strong>admin account</strong>".to_string(), "Crea un <strong>compte admin</strong>".to_string());
        translations.insert("Advanced".to_string(), "Avançat".to_string());
        translations.insert("Data folder".to_string(), "Dorsièr de donadas".to_string());
        translations.insert("Configure the database".to_string(), "Configura la basa de donadas".to_string());
        translations.insert("will be used".to_string(), "serà utilizat".to_string());
        translations.insert("Database user".to_string(), "Usancièr de la basa de donadas".to_string());
        translations.insert("Database password".to_string(), "Senhal de la basa de donadas".to_string());
        translations.insert("Database name".to_string(), "Nom de la basa de donadas".to_string());
        translations.insert("Database tablespace".to_string(), "Espandi de taula de basa de donadas".to_string());
        translations.insert("Database host".to_string(), "Òste de basa de donadas".to_string());
        translations.insert("Finish setup".to_string(), "Configuracion acabada".to_string());
        translations.insert("Log out".to_string(), "Sortida".to_string());
        translations.insert("Lost your password?".to_string(), "L'as perdut lo senhal ?".to_string());
        translations.insert("remember".to_string(), "bremba-te".to_string());
        translations.insert("Log in".to_string(), "Dintrada".to_string());
        
        OcTranslations {
            translations,
            plural_forms: "nplurals=2; plural=(n > 1);".to_string(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.translations.get(key)
    }

    pub fn get_plural_forms(&self) -> &str {
        &self.plural_forms
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translations() {
        let oc = OcTranslations::new();
        assert_eq!(oc.get("Sunday"), Some(&"Dimenge".to_string()));
        assert_eq!(oc.get("Monday"), Some(&"Diluns".to_string()));
        assert_eq!(oc.get("nonexistent"), None);
    }

    #[test]
    fn test_plural_forms() {
        let oc = OcTranslations::new();
        assert_eq!(oc.get_plural_forms(), "nplurals=2; plural=(n > 1);");
    }
}