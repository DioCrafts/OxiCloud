use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Recovery key successfully enabled", "Záchranný klíč byl úspěšně povolen");
        m.insert("Could not enable recovery key. Please check your recovery key password!", "Nepodařilo se povolit záchranný klíč. Zkontrolujte prosím vaše heslo záchranného klíče!");
        m.insert("Recovery key successfully disabled", "Záchranný klíč byl úspěšně zakázán");
        m.insert("Could not disable recovery key. Please check your recovery key password!", "Nelze zakázat záchranný klíč. Zkontrolujte prosím heslo vašeho záchranného klíče!");
        m.insert("Password successfully changed.", "Heslo bylo úspěšně změněno.");
        m.insert("Could not change the password. Maybe the old password was not correct.", "Změna hesla se nezdařila. Pravděpodobně nebylo stávající heslo zadáno správně.");
        m.insert("Private key password successfully updated.", "Heslo soukromého klíče úspěšně aktualizováno.");
        m.insert("Could not update the private key password. Maybe the old password was not correct.", "Nelze aktualizovat heslo soukromého klíče. Možná nebylo staré heslo správně.");
        m.insert("Encryption app not initialized! Maybe the encryption app was re-enabled during your session. Please try to log out and log back in to initialize the encryption app.", "Aplikace pro šifrování není inicializována! Je možné, že aplikace byla znovu aktivována během vašeho přihlášení. Zkuste se prosím odhlásit a znovu přihlásit pro provedení inicializace šifrovací aplikace.");
        m.insert("Can not decrypt this file, probably this is a shared file. Please ask the file owner to reshare the file with you.", "Tento soubor se nepodařilo dešifrovat, pravděpodobně je sdílený. Požádejte prosím majitele souboru, aby jej s vámi znovu sdílel.");
        m.insert("Unknown error please check your system settings or contact your administrator", "Neznámá chyba, zkontrolujte vaše systémová nastavení nebo kontaktujte vašeho správce");
        m.insert("Missing requirements.", "Nesplněné závislosti.");
        m.insert("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "Ujistěte se prosím, že máte nainstalované PHP 5.3.3 nebo novější a že máte povolené a správně nakonfigurované OpenSSL včetně jeho rozšíření pro PHP. Prozatím byla aplikace pro šifrování vypnuta.");
        m.insert("Following users are not set up for encryption:", "Následující uživatelé nemají nastavené šifrování:");
        m.insert("Saving...", "Ukládám...");
        m.insert("Go directly to your ", "Běžte přímo do vašeho");
        m.insert("personal settings", "osobní nastavení");
        m.insert("Encryption", "Šifrování");
        m.insert("Enable recovery key (allow to recover users files in case of password loss):", "Povolit klíč pro obnovu (umožňuje obnovu uživatelských souborů v případě ztráty hesla)");
        m.insert("Recovery key password", "Heslo klíče pro obnovu");
        m.insert("Repeat Recovery key password", "Zopakujte heslo klíče pro obnovu");
        m.insert("Enabled", "Povoleno");
        m.insert("Disabled", "Zakázáno");
        m.insert("Change recovery key password:", "Změna hesla klíče pro obnovu:");
        m.insert("Old Recovery key password", "Původní heslo klíče pro obnovu");
        m.insert("New Recovery key password", "Nové heslo klíče pro obnovu");
        m.insert("Repeat New Recovery key password", "Zopakujte nové heslo klíče pro obnovu");
        m.insert("Change Password", "Změnit heslo");
        m.insert("Your private key password no longer match your log-in password:", "Heslo vašeho soukromého klíče se již neshoduje s vaším přihlašovacím heslem:");
        m.insert("Set your old private key password to your current log-in password.", "Změňte heslo vaše soukromého klíče na stejné jako vaše přihlašovací heslo.");
        m.insert(" If you don't remember your old password you can ask your administrator to recover your files.", "Pokud si nepamatujete vaše původní heslo, můžete požádat správce o obnovu vašich souborů.");
        m.insert("Old log-in password", "Původní přihlašovací heslo");
        m.insert("Current log-in password", "Aktuální přihlašovací heslo");
        m.insert("Update Private Key Password", "Změnit heslo soukromého klíče");
        m.insert("Enable password recovery:", "Povolit obnovu hesla:");
        m.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "Zapnutí této volby vám umožní znovu získat přístup k vašim zašifrovaným souborům pokud ztratíte heslo");
        m.insert("File recovery settings updated", "Možnosti záchrany souborů aktualizovány");
        m.insert("Could not update file recovery", "Nelze nastavit záchranu souborů");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n==1) ? 0 : (n>=2 && n<=4) ? 1 : 2;";
}