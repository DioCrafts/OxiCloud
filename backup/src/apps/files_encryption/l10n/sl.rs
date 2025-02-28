use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Recovery key successfully enabled", "Ključ za obnovitev gesla je bil uspešno nastavljen");
        m.insert("Could not enable recovery key. Please check your recovery key password!", "Ključa za obnovitev gesla ni bilo mogoče nastaviti. Preverite ključ!");
        m.insert("Recovery key successfully disabled", "Ključ za obnovitev gesla je bil uspešno onemogočen");
        m.insert("Could not disable recovery key. Please check your recovery key password!", "Ključa za obnovitev gesla ni bilo mogoče onemogočiti. Preverite ključ!");
        m.insert("Password successfully changed.", "Geslo je bilo uspešno spremenjeno.");
        m.insert("Could not change the password. Maybe the old password was not correct.", "Gesla ni bilo mogoče spremeniti. Morda vnos starega gesla ni bil pravilen.");
        m.insert("Private key password successfully updated.", "Zasebni ključ za geslo je bil uspešno posodobljen.");
        m.insert("Could not update the private key password. Maybe the old password was not correct.", "Zasebnega ključa za geslo ni bilo mogoče posodobiti. Morda vnos starega gesla ni bil pravilen.");
        m.insert("Missing requirements.", "Manjkajoče zahteve");
        m.insert("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "Preverite, da imate na strežniku nameščen paket PHP 5.3.3 ali novejši in da je omogočen in pravilno nastavljen PHP OpenSSL . Zaenkrat je šifriranje onemogočeno.");
        m.insert("Following users are not set up for encryption:", "Naslednji uporabniki še nimajo nastavljenega šifriranja:");
        m.insert("Saving...", "Poteka shranjevanje ...");
        m.insert("personal settings", "osebne nastavitve");
        m.insert("Encryption", "Šifriranje");
        m.insert("Enable recovery key (allow to recover users files in case of password loss):", "Omogoči ključ za obnovitev datotek (v primeru izgube gesla)");
        m.insert("Recovery key password", "Ključ za obnovitev gesla");
        m.insert("Enabled", "Omogočeno");
        m.insert("Disabled", "Onemogočeno");
        m.insert("Change recovery key password:", "Spremeni ključ za obnovitev gesla:");
        m.insert("Old Recovery key password", "Stari ključ za obnovitev gesla");
        m.insert("New Recovery key password", "Nov ključ za obnovitev gesla");
        m.insert("Change Password", "Spremeni geslo");
        m.insert("Your private key password no longer match your log-in password:", "Vaš zasebni ključ za geslo se ne ujema z vnešenim geslom ob prijavi:");
        m.insert("Set your old private key password to your current log-in password.", "Nastavite svoj star zasebni ključ v geslo, vnešeno ob prijavi.");
        m.insert(" If you don't remember your old password you can ask your administrator to recover your files.", "Če ste svoje geslo pozabili, lahko vaše datoteke obnovi skrbnik sistema.");
        m.insert("Old log-in password", "Staro geslo");
        m.insert("Current log-in password", "Trenutno geslo");
        m.insert("Update Private Key Password", "Posodobi zasebni ključ");
        m.insert("Enable password recovery:", "Omogoči obnovitev gesla:");
        m.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "Nastavitev te možnosti omogoča ponovno pridobitev dostopa do šifriranih datotek, v primeru da boste geslo pozabili.");
        m.insert("File recovery settings updated", "Nastavitve obnavljanja dokumentov so bile posodobljene");
        m.insert("Could not update file recovery", "Nastavitev za obnavljanje dokumentov ni bilo mogoče posodobiti");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=4; plural=(n%100==1 ? 0 : n%100==2 ? 1 : n%100==3 || n%100==4 ? 2 : 3);";
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(key)
}