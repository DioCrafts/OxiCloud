use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Recovery key successfully enabled", "Gendannelsesnøgle aktiveret med succes");
        m.insert("Could not enable recovery key. Please check your recovery key password!", "Kunne ikke aktivere gendannelsesnøgle. Kontroller venligst dit gendannelsesnøgle kodeord!");
        m.insert("Recovery key successfully disabled", "Gendannelsesnøgle deaktiveret succesfuldt");
        m.insert("Could not disable recovery key. Please check your recovery key password!", "Kunne ikke deaktivere gendannelsesnøgle. Kontroller din gendannelsesnøgle kodeord!");
        m.insert("Password successfully changed.", "Kodeordet blev ændret succesfuldt");
        m.insert("Could not change the password. Maybe the old password was not correct.", "Kunne ikke ændre kodeordet. Måske var det gamle kodeord ikke korrekt.");
        m.insert("Private key password successfully updated.", "Privat nøgle kodeord succesfuldt opdateret.");
        m.insert("Could not update the private key password. Maybe the old password was not correct.", "Kunne ikke opdatere det private nøgle kodeord-. Måske var det gamle kodeord forkert.");
        m.insert("Missing requirements.", "Manglende betingelser.");
        m.insert("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "Sørg for at PHP 5.3.3 eller nyere er installeret og at OpenSSL sammen med PHP-udvidelsen er aktiveret og korrekt konfigureret. Indtil videre er krypteringsprogrammet deaktiveret.");
        m.insert("Following users are not set up for encryption:", "Følgende brugere er ikke sat op til kryptering:");
        m.insert("Saving...", "Gemmer...");
        m.insert("personal settings", "Personlige indstillinger");
        m.insert("Encryption", "Kryptering");
        m.insert("Enable recovery key (allow to recover users files in case of password loss):", "Aktiver gendannelsesnøgle (Tillad gendannelse af brugerfiler i tilfælde af tab af kodeord):");
        m.insert("Recovery key password", "Gendannelsesnøgle kodeord");
        m.insert("Enabled", "Aktiveret");
        m.insert("Disabled", "Deaktiveret");
        m.insert("Change recovery key password:", "Skift gendannelsesnøgle kodeord:");
        m.insert("Old Recovery key password", "Gammel Gendannelsesnøgle kodeord");
        m.insert("New Recovery key password", "Ny Gendannelsesnøgle kodeord");
        m.insert("Change Password", "Skift Kodeord");
        m.insert("Your private key password no longer match your log-in password:", "Dit private nøgle kodeord stemmer ikke længere overens med dit login kodeord:");
        m.insert("Set your old private key password to your current log-in password.", "Sæt dit gamle private nøgle kodeord til at være dit nuværende login kodeord. ");
        m.insert(" If you don't remember your old password you can ask your administrator to recover your files.", "Hvis du ikke kan huske dit gamle kodeord kan du bede din administrator om at gendanne dine filer.");
        m.insert("Old log-in password", "Gammelt login kodeord");
        m.insert("Current log-in password", "Nuvrende login kodeord");
        m.insert("Update Private Key Password", "Opdater Privat Nøgle Kodeord");
        m.insert("Enable password recovery:", "Aktiver kodeord gendannelse:");
        m.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "Aktivering af denne valgmulighed tillader dig at generhverve adgang til dine krypterede filer i tilfælde af tab af kodeord");
        m.insert("File recovery settings updated", "Filgendannelsesindstillinger opdateret");
        m.insert("Could not update file recovery", "Kunne ikke opdatere filgendannelse");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}