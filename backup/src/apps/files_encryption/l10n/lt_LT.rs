use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Recovery key successfully enabled", "Atkūrimo raktas sėkmingai įjungtas");
        m.insert("Could not enable recovery key. Please check your recovery key password!", "Neišėjo įjungti jūsų atkūrimo rakto. Prašome jį patikrinti!");
        m.insert("Recovery key successfully disabled", "Atkūrimo raktas sėkmingai išjungtas");
        m.insert("Could not disable recovery key. Please check your recovery key password!", "Neišėjo išjungti jūsų atkūrimo rakto. Prašome jį patikrinti!");
        m.insert("Password successfully changed.", "Slaptažodis sėkmingai pakeistas");
        m.insert("Could not change the password. Maybe the old password was not correct.", "Slaptažodis nebuvo pakeistas. Gali būti, kad buvo neteisingai suvestas senasis.");
        m.insert("Private key password successfully updated.", "Privataus rakto slaptažodis buvo sėkmingai atnaujintas.");
        m.insert("Could not update the private key password. Maybe the old password was not correct.", "Nepavyko atnaujinti privataus rakto slaptažodžio. Gali būti, kad buvo neteisingai suvestas senasis.");
        m.insert("Encryption app not initialized! Maybe the encryption app was re-enabled during your session. Please try to log out and log back in to initialize the encryption app.", "Šifravimo programa nepaleista! Galbūt šifravimo programa buvo įjungta dar kartą Jūsų sesijos metu. Prašome atsijungti ir vėl prisijungti, kad paleisti šifravimo programą.");
        m.insert("Can not decrypt this file, probably this is a shared file. Please ask the file owner to reshare the file with you.", "Failo iššifruoti nepavyko, gali būti jog jis yra pasidalintas su jumis. Paprašykite failo savininko, kad jums iš naujo pateiktų šį failą.");
        m.insert("Unknown error please check your system settings or contact your administrator", "Neatpažinta klaida, patikrinkite sistemos nustatymus arba kreipkitės į savo sistemos aministratorių");
        m.insert("Missing requirements.", "Trūkstami laukai.");
        m.insert("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "Prašome įsitikinti, kad PHP 5.3.3 ar naujesnė yra įdiegta ir kad OpenSSL kartu su PHP plėtiniu yra šjungti ir teisingai sukonfigūruoti. Kol kas šifravimo programa bus išjungta.");
        m.insert("Following users are not set up for encryption:", "Sekantys naudotojai nenustatyti šifravimui:");
        m.insert("Saving...", "Saugoma...");
        m.insert("Go directly to your ", "Eiti tiesiai į Jūsų");
        m.insert("personal settings", "asmeniniai nustatymai");
        m.insert("Encryption", "Šifravimas");
        m.insert("Enable recovery key (allow to recover users files in case of password loss):", "Įjunkite atkūrimo raktą, (leisti atkurti naudotojų failus praradus slaptažodį):");
        m.insert("Recovery key password", "Atkūrimo rakto slaptažodis");
        m.insert("Repeat Recovery key password", "Pakartokite atkūrimo rakto slaptažodį");
        m.insert("Enabled", "Įjungta");
        m.insert("Disabled", "Išjungta");
        m.insert("Change recovery key password:", "Pakeisti atkūrimo rakto slaptažodį:");
        m.insert("Old Recovery key password", "Senas atkūrimo rakto slaptažodis");
        m.insert("New Recovery key password", "Naujas atkūrimo rakto slaptažodis");
        m.insert("Repeat New Recovery key password", "Pakartokite naują atkūrimo rakto slaptažodį");
        m.insert("Change Password", "Pakeisti slaptažodį");
        m.insert("Your private key password no longer match your log-in password:", "Privatus rakto slaptažodis daugiau neatitinka Jūsų prisijungimo slaptažodžio:");
        m.insert("Set your old private key password to your current log-in password.", "Nustatyti Jūsų privataus rakto slaptažodį į Jūsų dabartinį prisijungimo.");
        m.insert(" If you don't remember your old password you can ask your administrator to recover your files.", "Jei nepamenate savo seno slaptažodžio, galite paprašyti administratoriaus atkurti Jūsų failus.");
        m.insert("Old log-in password", "Senas prisijungimo slaptažodis");
        m.insert("Current log-in password", "Dabartinis prisijungimo slaptažodis");
        m.insert("Update Private Key Password", "Atnaujinti privataus rakto slaptažodį");
        m.insert("Enable password recovery:", "Įjungti slaptažodžio atkūrimą:");
        m.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "Įjungus šią funkciją jums bus suteiktas pakartotinis priėjimas prie Jūsų šifruotų failų pamiršus slaptažodį.");
        m.insert("File recovery settings updated", "Failų atkūrimo nustatymai pakeisti");
        m.insert("Could not update file recovery", "Neišėjo atnaujinti failų atkūrimo");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && (n%100<10 || n%100>=20) ? 1 : 2);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn translate(key: &str) -> &'static str {
    get_translation(key).unwrap_or(key)
}