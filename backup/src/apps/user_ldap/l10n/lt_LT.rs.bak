use std::collections::HashMap;
use rust_i18n::i18n;

i18n!("lt_LT");

pub fn initialize_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("Failed to clear the mappings.".to_string(), "Nepavyko išvalyti sąsajų.".to_string());
    translations.insert("Failed to delete the server configuration".to_string(), "Nepavyko pašalinti serverio konfigūracijos".to_string());
    translations.insert("The configuration is valid and the connection could be established!".to_string(), "Konfigūracija yra tinkama bei prisijungta sėkmingai!".to_string());
    translations.insert("Deletion failed".to_string(), "Ištrinti nepavyko".to_string());
    translations.insert("Keep settings?".to_string(), "Išlaikyti nustatymus?".to_string());
    translations.insert("Cannot add server configuration".to_string(), "Negalima pridėti serverio konfigūracijos".to_string());
    translations.insert("mappings cleared".to_string(), "susiejimai išvalyti".to_string());
    translations.insert("Success".to_string(), "Sėkmingai".to_string());
    translations.insert("Error".to_string(), "Klaida".to_string());
    translations.insert("Select groups".to_string(), "Pasirinkti grupes".to_string());
    translations.insert("Connection test succeeded".to_string(), "Ryšio patikrinimas pavyko".to_string());
    translations.insert("Connection test failed".to_string(), "Ryšio patikrinimas nepavyko".to_string());
    translations.insert("Do you really want to delete the current Server Configuration?".to_string(), "Ar tikrai norite ištrinti dabartinę serverio konfigūraciją?".to_string());
    translations.insert("Confirm Deletion".to_string(), "Patvirtinkite trynimą".to_string());
    translations.insert("Save".to_string(), "Išsaugoti".to_string());
    translations.insert("Test Configuration".to_string(), "Bandyti konfigūraciją".to_string());
    translations.insert("Help".to_string(), "Pagalba".to_string());
    translations.insert("Add Server Configuration".to_string(), "Pridėti serverio konfigūraciją".to_string());
    translations.insert("Host".to_string(), "Mazgas".to_string());
    translations.insert("Port".to_string(), "Prievadas".to_string());
    translations.insert("User DN".to_string(), "Naudotojas DN".to_string());
    translations.insert("Password".to_string(), "Slaptažodis".to_string());
    translations.insert("For anonymous access, leave DN and Password empty.".to_string(), "Anoniminiam prisijungimui, palikite DN ir Slaptažodis laukus tuščius.".to_string());
    translations.insert("One Base DN per line".to_string(), "Vienas bazinis DN eilutėje".to_string());
    translations.insert("Back".to_string(), "Atgal".to_string());
    translations.insert("Continue".to_string(), "Tęsti".to_string());
    translations.insert("Connection Settings".to_string(), "Ryšio nustatymai".to_string());
    translations.insert("Configuration Active".to_string(), "Konfigūracija aktyvi".to_string());
    translations.insert("When unchecked, this configuration will be skipped.".to_string(), "Kai nepažymėta, ši konfigūracija bus praleista.".to_string());
    translations.insert("User Login Filter".to_string(), "Naudotojo prisijungimo filtras".to_string());
    translations.insert("Backup (Replica) Host".to_string(), "Atsarginės kopijos (Replica) mazgas".to_string());
    translations.insert("Backup (Replica) Port".to_string(), "Atsarginės kopijos (Replica) prievadas".to_string());
    translations.insert("Disable Main Server".to_string(), "Išjungti pagrindinį serverį".to_string());
    translations.insert("Only connect to the replica server.".to_string(), "Tik prisijungti prie reprodukcinio (replica) serverio.".to_string());
    translations.insert("Turn off SSL certificate validation.".to_string(), "Išjungti SSL sertifikato tikrinimą.".to_string());
    translations.insert("Directory Settings".to_string(), "Katalogo nustatymai".to_string());
    translations.insert("Base User Tree".to_string(), "Bazinis naudotojo medis".to_string());
    translations.insert("User Search Attributes".to_string(), "Naudotojo paieškos atributai".to_string());
    translations.insert("Base Group Tree".to_string(), "Bazinis grupės medis".to_string());
    translations.insert("Group Search Attributes".to_string(), "Grupės paieškos atributai".to_string());
    translations.insert("Group-Member association".to_string(), "Grupės-Nario sąsaja".to_string());
    translations.insert("Special Attributes".to_string(), "Specialūs atributai".to_string());
    translations.insert("Quota Field".to_string(), "Kvotos laukas".to_string());
    translations.insert("Quota Default".to_string(), "Numatyta kvota".to_string());
    translations.insert("in bytes".to_string(), "baitais".to_string());
    translations.insert("Email Field".to_string(), "El. pašto laukas".to_string());
    translations.insert("User Home Folder Naming Rule".to_string(), "Naudotojo namų aplanko pavadinimo taisyklė".to_string());
    translations.insert("Internal Username".to_string(), "Vidinis naudotojo vardas".to_string());
    translations.insert("Internal Username Attribute:".to_string(), "Vidinis naudotojo vardo atributas:".to_string());
    translations.insert("Override UUID detection".to_string(), "Perrašyti UUID aptikimą".to_string());
    translations.insert("Username-LDAP User Mapping".to_string(), "Naudotojo vardo - LDAP naudotojo sąsaja".to_string());
    translations.insert("Clear Username-LDAP User Mapping".to_string(), "Išvalyti naudotojo vardo - LDAP naudotojo sąsają".to_string());
    translations.insert("Clear Groupname-LDAP Group Mapping".to_string(), "Išvalyti grupės pavadinimo - LDAP naudotojo sąsają".to_string());
    
    translations
}

pub fn initialize_plural_forms() -> &'static str {
    "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && (n%100<10 || n%100>=20) ? 1 : 2);"
}

pub fn translate_plural(singular: &str, plural: &str, count: i64) -> String {
    match (count % 10, count % 100) {
        (1, c) if c != 11 => format!("{} group found", count),
        (2..=9, c) if c < 10 || c >= 20 => format!("{} groups found", count),
        _ => format!("{} groups found", count),
    }
}

pub fn translate_user_plural(singular: &str, plural: &str, count: i64) -> String {
    match (count % 10, count % 100) {
        (1, c) if c != 11 => format!("{} user found", count),
        (2..=9, c) if c < 10 || c >= 20 => format!("{} users found", count),
        _ => format!("{} users found", count),
    }
}