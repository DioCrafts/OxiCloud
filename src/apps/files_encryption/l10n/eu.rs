use rust_i18n::t;

// Define la table de traducciones como un lazy_static para el idioma Euskera (eu)
lazy_static! {
    pub static ref TRANSLATIONS: phf::Map<&'static str, &'static str> = phf::phf_map! {
        "Recovery key successfully enabled" => "Berreskuratze gakoa behar bezala gaitua",
        "Could not enable recovery key. Please check your recovery key password!" => "Ezin da berreskuratze gako gaitu. Egiaztatu berreskuratze gako pasahitza!",
        "Recovery key successfully disabled" => "Berreskuratze gakoa behar bezala desgaitu da",
        "Could not disable recovery key. Please check your recovery key password!" => "Ezin da berreskuratze gako desgaitu. Egiaztatu berreskuratze gako pasahitza!",
        "Password successfully changed." => "Pasahitza behar bezala aldatu da.",
        "Could not change the password. Maybe the old password was not correct." => "Ezin izan da pasahitza aldatu. Agian pasahitz zaharra okerrekoa da.",
        "Private key password successfully updated." => "Gako pasahitz pribatu behar bezala eguneratu da.",
        "Could not update the private key password. Maybe the old password was not correct." => "Ezin izan da gako pribatu pasahitza eguneratu. Agian pasahitz zaharra okerrekoa da.",
        "Missing requirements." => "Eskakizun batzuk ez dira betetzen.",
        "Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled." => "Mesedez ziurtatu PHP 5.3.3 edo berriago bat instalatuta dagoela eta OpenSSL PHP hedapenarekin gaitua eta ongi konfiguratuta dagoela. Oraingoz, enkriptazio aplikazioa desgaituta dago.",
        "Following users are not set up for encryption:" => "Hurrengo erabiltzaileak ez daude enktriptatzeko konfiguratutak:",
        "Saving..." => "Gordetzen...",
        "personal settings" => "ezarpen pertsonalak",
        "Encryption" => "Enkriptazioa",
        "Enable recovery key (allow to recover users files in case of password loss):" => "Gaitu berreskurapen gakoa (erabiltzaileen fitxategiak berreskuratzea ahalbidetzen du pasahitza galtzen badute ere):",
        "Recovery key password" => "Berreskuratze gako pasahitza",
        "Enabled" => "Gaitua",
        "Disabled" => "Ez-gaitua",
        "Change recovery key password:" => "Aldatu berreskuratze gako pasahitza:",
        "Old Recovery key password" => "Berreskuratze gako pasahitz zaharra",
        "New Recovery key password" => "Berreskuratze gako pasahitz berria",
        "Change Password" => "Aldatu Pasahitza",
        "Your private key password no longer match your log-in password:" => "Zure gako pribatuaren pasahitza ez da dagoeneko zure sarrera pasahitza:",
        "Set your old private key password to your current log-in password." => "Ezarri zure gako pribatu zaharraren pasahitza zure oraingo sarrerako psahitzara.",
        " If you don't remember your old password you can ask your administrator to recover your files." => "Ez baduzu zure pasahitz zaharra gogoratzen eskatu zure administratzaileari zure fitxategiak berreskuratzeko.",
        "Old log-in password" => "Sartzeko pasahitz zaharra",
        "Current log-in password" => "Sartzeko oraingo pasahitza",
        "Update Private Key Password" => "Eguneratu gako pribatu pasahitza",
        "Enable password recovery:" => "Gaitu pasahitz berreskuratzea:",
        "Enabling this option will allow you to reobtain access to your encrypted files in case of password loss" => "Aukera hau gaituz zure enkriptatutako fitxategiak berreskuratu ahal izango dituzu pasahitza galtzekotan",
        "File recovery settings updated" => "Fitxategi berreskuratze ezarpenak eguneratuak",
        "Could not update file recovery" => "Ezin da fitxategi berreskuratzea eguneratu"
    };
}

// Configuración del sistema de pluralización para Euskera
pub fn get_plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}

// Función para registrar el idioma
pub fn register_language() {
    rust_i18n::set_translator!(TRANSLATIONS.clone(), get_plural_form);
}