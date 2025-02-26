use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Recovery key successfully enabled", "Herstelsleutel succesvol geactiveerd");
        m.insert("Could not enable recovery key. Please check your recovery key password!", "Kon herstelsleutel niet activeren. Controleer het wachtwoord van uw herstelsleutel!");
        m.insert("Recovery key successfully disabled", "Herstelsleutel succesvol gedeactiveerd");
        m.insert("Could not disable recovery key. Please check your recovery key password!", "Kon herstelsleutel niet deactiveren. Controleer het wachtwoord van uw herstelsleutel!");
        m.insert("Password successfully changed.", "Wachtwoord succesvol gewijzigd.");
        m.insert("Could not change the password. Maybe the old password was not correct.", "Kon wachtwoord niet wijzigen. Wellicht oude wachtwoord niet juist ingevoerd.");
        m.insert("Private key password successfully updated.", "Privésleutel succesvol bijgewerkt.");
        m.insert("Could not update the private key password. Maybe the old password was not correct.", "Kon het wachtwoord van de privésleutel niet wijzigen. Misschien was het oude wachtwoord onjuist.");
        m.insert("Encryption app not initialized! Maybe the encryption app was re-enabled during your session. Please try to log out and log back in to initialize the encryption app.", "Crypto app niet geïnitialiseerd. Misschien werd de crypto app geheractiveerd tijdens de sessie. Log uit en log daarna opnieuw in om de crypto app te initialiseren.");
        m.insert("Your private key is not valid! Likely your password was changed outside of %s (e.g. your corporate directory). You can update your private key password in your personal settings to recover access to your encrypted files.", "Uw privésleutel is niet geldig! Waarschijnlijk is uw wachtwoord gewijzigd buiten %s (bijv. uw corporate directory). U kunt uw privésleutel wachtwoord in uw persoonlijke instellingen bijwerken om toegang te krijgen tot uw versleutelde bestanden.");
        m.insert("Can not decrypt this file, probably this is a shared file. Please ask the file owner to reshare the file with you.", "Kan dit bestand niet ontcijferen, waarschijnlijk is het een gedeeld bestand, Vraag de eigenaar om het bestand opnieuw met u te delen.");
        m.insert("Unknown error please check your system settings or contact your administrator", "Onbekende fout, Controleer uw systeeminstellingen of neem contact op met uw systeembeheerder");
        m.insert("Missing requirements.", "Missende benodigdheden.");
        m.insert("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "Wees er zeker van dat PHP5.3.3 of nieuwer is geïstalleerd en dat de OpenSSL PHP extensie is ingeschakeld en correct geconfigureerd. De versleutel-app is voorlopig uitgeschakeld.");
        m.insert("Following users are not set up for encryption:", "De volgende gebruikers hebben geen configuratie voor encryptie:");
        m.insert("Saving...", "Opslaan");
        m.insert("Go directly to your ", "Ga meteen naar uw");
        m.insert("personal settings", "persoonlijke instellingen");
        m.insert("Encryption", "Versleuteling");
        m.insert("Enable recovery key (allow to recover users files in case of password loss):", "Activeren herstelsleutel (maakt het mogelijk om gebruikersbestanden terug te halen in geval van verlies van het wachtwoord):");
        m.insert("Recovery key password", "Wachtwoord herstelsleulel");
        m.insert("Repeat Recovery key password", "Herhaal het herstelsleutel wachtwoord");
        m.insert("Enabled", "Geactiveerd");
        m.insert("Disabled", "Gedeactiveerd");
        m.insert("Change recovery key password:", "Wijzig wachtwoord herstelsleutel:");
        m.insert("Old Recovery key password", "Oude wachtwoord herstelsleutel");
        m.insert("New Recovery key password", "Nieuwe wachtwoord herstelsleutel");
        m.insert("Repeat New Recovery key password", "Herhaal het nieuwe herstelsleutel wachtwoord");
        m.insert("Change Password", "Wijzigen wachtwoord");
        m.insert("Your private key password no longer match your log-in password:", "Het wachtwoord van uw privésleutel komt niet meer overeen met uw inlogwachtwoord:");
        m.insert("Set your old private key password to your current log-in password.", "Stel het wachtwoord van uw oude privésleutel in op uw huidige inlogwachtwoord.");
        m.insert(" If you don't remember your old password you can ask your administrator to recover your files.", "Als u uw oude wachtwoord niet meer weet, kunt u uw beheerder vragen uw bestanden terug te halen.");
        m.insert("Old log-in password", "Oude wachtwoord");
        m.insert("Current log-in password", "Huidige wachtwoord");
        m.insert("Update Private Key Password", "Bijwerken wachtwoord Privésleutel");
        m.insert("Enable password recovery:", "Activeren wachtwoord herstel:");
        m.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "Het activeren van deze optie maakt het mogelijk om uw versleutelde bestanden te benaderen als uw wachtwoord kwijt is");
        m.insert("File recovery settings updated", "Bestandsherstel instellingen bijgewerkt");
        m.insert("Could not update file recovery", "Kon bestandsherstel niet bijwerken");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}