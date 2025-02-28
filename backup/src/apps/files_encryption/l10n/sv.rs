use std::collections::HashMap;
use once_cell::sync::Lazy;

pub(crate) static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Recovery key successfully enabled", "Återställningsnyckeln har framgångsrikt aktiverats");
    m.insert("Could not enable recovery key. Please check your recovery key password!", "Kunde inte aktivera återställningsnyckeln. Vänligen kontrollera ditt lösenord för återställningsnyckeln!");
    m.insert("Recovery key successfully disabled", "Återställningsnyckeln har framgångsrikt inaktiverats");
    m.insert("Could not disable recovery key. Please check your recovery key password!", "Kunde inte inaktivera återställningsnyckeln. Vänligen kontrollera ditt lösenord för återställningsnyckeln!");
    m.insert("Password successfully changed.", "Ändringen av lösenordet lyckades.");
    m.insert("Could not change the password. Maybe the old password was not correct.", "Kunde inte ändra lösenordet. Kanske det gamla lösenordet inte var rätt.");
    m.insert("Private key password successfully updated.", "Den privata nyckelns lösenord uppdaterades utan problem.");
    m.insert("Could not update the private key password. Maybe the old password was not correct.", "Kunde inte uppdatera lösenordet för den privata nyckeln. Kanske var det gamla lösenordet fel.");
    m.insert("Encryption app not initialized! Maybe the encryption app was re-enabled during your session. Please try to log out and log back in to initialize the encryption app.", "Krypteringsprogrammet kunde inte initieras! Möjligen blev krypteringsprogrammet återaktiverad under din session. Försök med att logga ut och in igen för att initiera krypteringsprogrammet.");
    m.insert("Can not decrypt this file, probably this is a shared file. Please ask the file owner to reshare the file with you.", "Kan ej dekryptera denna fil, förmodligen är det en delad fil. Be ägaren av filen att dela den med dig.");
    m.insert("Unknown error please check your system settings or contact your administrator", "Oväntat fel, kolla dina system inställningar eller kontakta din administratör");
    m.insert("Missing requirements.", "Krav som saknas");
    m.insert("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "Kontrollera att PHP 5.3.3 eller senare är installerad och att tillägget OpenSSL PHP är aktiverad och korrekt konfigurerad. Kryptering är tillsvidare inaktiverad.");
    m.insert("Following users are not set up for encryption:", "Följande användare har inte aktiverat kryptering:");
    m.insert("Saving...", "Sparar...");
    m.insert("Go directly to your ", "Gå direkt till din");
    m.insert("personal settings", "personliga inställningar");
    m.insert("Encryption", "Kryptering");
    m.insert("Enable recovery key (allow to recover users files in case of password loss):", "Aktivera återställningsnyckel (för att kunna återfå användarens filer vid glömt eller förlorat lösenord):");
    m.insert("Recovery key password", "Lösenord för återställningsnyckel");
    m.insert("Repeat Recovery key password", "Upprepa återställningsnyckelns lösenord");
    m.insert("Enabled", "Aktiverad");
    m.insert("Disabled", "Inaktiverad");
    m.insert("Change recovery key password:", "Ändra lösenord för återställningsnyckel:");
    m.insert("Old Recovery key password", "Gammalt lösenord för återställningsnyckel");
    m.insert("New Recovery key password", "Nytt lösenord för återställningsnyckel");
    m.insert("Repeat New Recovery key password", "Upprepa lösenord för ny återställningsnyckel");
    m.insert("Change Password", "Byt lösenord");
    m.insert("Your private key password no longer match your log-in password:", "Lösenordet till din privata nyckel stämmer inte längre överens med ditt inloggningslösenord:");
    m.insert("Set your old private key password to your current log-in password.", "Använd din gamla privata nyckels lösenord som ditt aktuella inloggningslösenord.");
    m.insert(" If you don't remember your old password you can ask your administrator to recover your files.", "Om du inte kommer ihåg ditt gamla lösenord kan du be din administratör att återställa dina filer.");
    m.insert("Old log-in password", "Gammalt inloggningslösenord");
    m.insert("Current log-in password", "Nuvarande inloggningslösenord");
    m.insert("Update Private Key Password", "Uppdatera lösenordet för din privata nyckel");
    m.insert("Enable password recovery:", "Aktivera lösenordsåterställning");
    m.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "Genom att aktivera detta alternativ kommer du kunna återfå tillgång till dina krypterade filer om du skulle förlora/glömma ditt lösenord");
    m.insert("File recovery settings updated", "Inställningarna för filåterställning har uppdaterats");
    m.insert("Could not update file recovery", "Kunde inte uppdatera filåterställning");
    m
});

pub(crate) const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";