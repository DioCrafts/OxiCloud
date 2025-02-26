use rust_i18n::i18n;

i18n!("sk_SK", {
    "Recovery key successfully enabled": "Záchranný kľúč bol úspešne povolený",
    "Could not enable recovery key. Please check your recovery key password!": "Nepodarilo sa povoliť záchranný kľúč. Skontrolujte prosím Vaše heslo záchranného kľúča!",
    "Recovery key successfully disabled": "Záchranný kľúč bol úspešne zakázaný",
    "Could not disable recovery key. Please check your recovery key password!": "Nepodarilo sa zakázať záchranný kľúč. Skontrolujte prosím Vaše heslo záchranného kľúča!",
    "Password successfully changed.": "Heslo úspešne zmenené.",
    "Could not change the password. Maybe the old password was not correct.": "Nemožno zmeniť heslo. Pravdepodobne nebolo staré heslo zadané správne.",
    "Private key password successfully updated.": "Heslo súkromného kľúča je úspešne aktualizované.",
    "Could not update the private key password. Maybe the old password was not correct.": "Nemožno aktualizovať heslo súkromného kľúča. Možno nebolo staré heslo správne.",
    "Encryption app not initialized! Maybe the encryption app was re-enabled during your session. Please try to log out and log back in to initialize the encryption app.": "Šifrovacia aplikácia nie je inicializovaná. Je možné, že aplikácia bola znova aktivovaná počas vášho prihlasovania. Pokúste sa odhlásiť a znova prihlásiť pre inicializáciu šifrovania.",
    "Can not decrypt this file, probably this is a shared file. Please ask the file owner to reshare the file with you.": "Tento súbor sa nepodarilo dešifrovať, pravdepodobne je zdieľaný. Požiadajte majiteľa súboru, aby ho s vami znovu vyzdieľal.",
    "Unknown error please check your system settings or contact your administrator": "Neznáma chyba, skontrolujte si vaše systémové nastavenia alebo kontaktujte administrátora",
    "Missing requirements.": "Chýbajúce požiadavky.",
    "Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.": "Prosím uistite sa, že PHP verzie 5.3.3 alebo novšej je nainštalované a tiež, že OpenSSL knižnica spolu z PHP rozšírením je povolená a konfigurovaná správne. Nateraz bola aplikácia šifrovania zablokovaná.",
    "Following users are not set up for encryption:": "Nasledujúci používatelia nie sú nastavení pre šifrovanie:",
    "Saving...": "Ukladám...",
    "Go directly to your ": "Choďte priamo do vášho",
    "personal settings": "osobné nastavenia",
    "Encryption": "Šifrovanie",
    "Enable recovery key (allow to recover users files in case of password loss):": "Povoliť obnovovací kľúč (umožňuje obnoviť používateľské súbory v prípade straty hesla):",
    "Recovery key password": "Heslo obnovovacieho kľúča",
    "Repeat Recovery key password": "Zopakujte heslo kľúča pre obnovu",
    "Enabled": "Povolené",
    "Disabled": "Zakázané",
    "Change recovery key password:": "Zmeniť heslo obnovovacieho kľúča:",
    "Old Recovery key password": "Staré heslo obnovovacieho kľúča",
    "New Recovery key password": "Nové heslo obnovovacieho kľúča",
    "Repeat New Recovery key password": "Zopakujte nové heslo kľúča pre obnovu",
    "Change Password": "Zmeniť heslo",
    "Your private key password no longer match your log-in password:": "Vaše heslo súkromného kľúča je rovnaké ako Vaše prihlasovacie heslo:",
    "Set your old private key password to your current log-in password.": "Nastavte si staré heslo súkromného kľúča k Vášmu súčasnému prihlasovaciemu heslu.",
    " If you don't remember your old password you can ask your administrator to recover your files.": "Ak si nepamätáte svoje staré heslo, môžete požiadať správcu o obnovenie svojich súborov.",
    "Old log-in password": "Staré prihlasovacie heslo",
    "Current log-in password": "Súčasné prihlasovacie heslo",
    "Update Private Key Password": "Aktualizovať heslo súkromného kľúča",
    "Enable password recovery:": "Povoliť obnovu hesla:",
    "Enabling this option will allow you to reobtain access to your encrypted files in case of password loss": "Povolenie Vám umožní znovu získať prístup k Vašim zašifrovaným súborom, ak stratíte heslo",
    "File recovery settings updated": "Nastavenie obnovy súborov aktualizované",
    "Could not update file recovery": "Nemožno aktualizovať obnovenie súborov"
});

#[cfg(feature = "plural_forms")]
pub fn get_plural_forms() -> &'static str {
    "nplurals=3; plural=(n==1) ? 0 : (n>=2 && n<=4) ? 1 : 2;"
}