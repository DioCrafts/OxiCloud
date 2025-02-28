use std::collections::HashMap;
use rust_i18n::Translations;

/// Finnish (Finland) translation file
pub fn get_translations() -> Translations {
    let mut translations = HashMap::new();
    
    translations.insert("Access granted".to_string(), "Pääsy sallittu".to_string());
    translations.insert("Error configuring Dropbox storage".to_string(), "Virhe Dropbox levyn asetuksia tehtäessä".to_string());
    translations.insert("Grant access".to_string(), "Salli pääsy".to_string());
    translations.insert("Please provide a valid Dropbox app key and secret.".to_string(), "Anna kelvollinen Dropbox-sovellusavain ja salainen vastaus.".to_string());
    translations.insert("Error configuring Google Drive storage".to_string(), "Virhe Google Drive levyn asetuksia tehtäessä".to_string());
    translations.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.".to_string(), "<b>Varoitus:</b> \"smbclient\" ei ole asennettuna. CIFS-/SMB-jakojen liittäminen ei ole mahdollista. Pyydä järjestelmän ylläpitäjää asentamaan smbclient.".to_string());
    translations.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.".to_string(), "<b>Varoitus:</b> PHP:n FTP-tuki ei ole käytössä tai sitä ei ole asennettu. FTP-jakojen liittäminen ei ole mahdollista. Pyydä järjestelmän ylläpitäjää ottamaan FTP-tuki käyttöön.".to_string());
    translations.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.".to_string(), "<b>Varoitus:</b> PHP:n Curl-tuki ei ole käytössä tai sitä ei ole lainkaan asennettu. ownCloudin, WebDAV:in tai Google Driven liittäminen ei ole mahdollista. Pyydä järjestelmän ylläpitäjää ottamaan Curl-tuki käyttöön.".to_string());
    translations.insert("External Storage".to_string(), "Erillinen tallennusväline".to_string());
    translations.insert("Folder name".to_string(), "Kansion nimi".to_string());
    translations.insert("External storage".to_string(), "Ulkoinen tallennustila".to_string());
    translations.insert("Configuration".to_string(), "Asetukset".to_string());
    translations.insert("Options".to_string(), "Valinnat".to_string());
    translations.insert("Applicable".to_string(), "Sovellettavissa".to_string());
    translations.insert("Add storage".to_string(), "Lisää tallennustila".to_string());
    translations.insert("None set".to_string(), "Ei asetettu".to_string());
    translations.insert("All Users".to_string(), "Kaikki käyttäjät".to_string());
    translations.insert("Groups".to_string(), "Ryhmät".to_string());
    translations.insert("Users".to_string(), "Käyttäjät".to_string());
    translations.insert("Delete".to_string(), "Poista".to_string());
    translations.insert("Enable User External Storage".to_string(), "Ota käyttöön ulkopuoliset tallennuspaikat".to_string());
    translations.insert("Allow users to mount their own external storage".to_string(), "Salli käyttäjien liittää omia erillisiä tallennusvälineitä".to_string());
    translations.insert("SSL root certificates".to_string(), "SSL-juurivarmenteet".to_string());
    translations.insert("Import Root Certificate".to_string(), "Tuo juurivarmenne".to_string());
    
    Translations {
        translations,
        plural_forms: "nplurals=2; plural=(n != 1);".to_string(),
    }
}

pub struct Translations {
    pub translations: HashMap<String, String>,
    pub plural_forms: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translations_loaded() {
        let translations = get_translations();
        assert!(!translations.translations.is_empty());
        assert_eq!(translations.translations.get("Delete"), Some(&"Poista".to_string()));
    }
}