use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("%s shared »%s« with you", "%s s vámi sdílí »%s«");
        m.insert("Couldn't send mail to following users: %s ", "Nebylo možné odeslat e-mail následujícím uživatelům: %s");
        m.insert("Turned on maintenance mode", "Zapnut režim údržby");
        m.insert("Turned off maintenance mode", "Vypnut režim údržby");
        m.insert("Updated database", "Zaktualizována databáze");
        m.insert("Updating filecache, this may take really long...", "Aktualizuji souborovou mezipaměť, toto může trvat opravdu dlouho...");
        m.insert("Updated filecache", "Aktualizována souborová mezipaměť");
        m.insert("... %d%% done ...", "... %d%% dokončeno ...");
        m.insert("No image or file provided", "Soubor nebo obrázek nebyl zadán");
        m.insert("Unknown filetype", "Neznámý typ souboru");
        m.insert("Invalid image", "Chybný obrázek");
        m.insert("No temporary profile picture available, try again", "Dočasný profilový obrázek není k dispozici, zkuste to znovu");
        m.insert("No crop data provided", "Nebyla poskytnuta data pro oříznutí obrázku");
        m.insert("Sunday", "Neděle");
        m.insert("Monday", "Pondělí");
        m.insert("Tuesday", "Úterý");
        m.insert("Wednesday", "Středa");
        m.insert("Thursday", "Čtvrtek");
        m.insert("Friday", "Pátek");
        m.insert("Saturday", "Sobota");
        m.insert("January", "Leden");
        m.insert("February", "Únor");
        m.insert("March", "Březen");
        m.insert("April", "Duben");
        m.insert("May", "Květen");
        m.insert("June", "Červen");
        m.insert("July", "Červenec");
        m.insert("August", "Srpen");
        m.insert("September", "Září");
        m.insert("October", "Říjen");
        m.insert("November", "Listopad");
        m.insert("December", "Prosinec");
        m.insert("Settings", "Nastavení");
        m.insert("seconds ago", "před pár vteřinami");
        m.insert("today", "dnes");
        m.insert("yesterday", "včera");
        m.insert("last month", "minulý měsíc");
        m.insert("months ago", "před měsíci");
        m.insert("last year", "minulý rok");
        m.insert("years ago", "před lety");
        m.insert("Choose", "Vybrat");
        m.insert("Error loading file picker template: {error}", "Chyba při nahrávání šablony výběru souborů: {error}");
        m.insert("Yes", "Ano");
        m.insert("No", "Ne");
        m.insert("Ok", "Ok");
        m.insert("Error loading message template: {error}", "Chyba při nahrávání šablony zprávy: {error}");
        m.insert("One file conflict", "Jeden konflikt souboru");
        m.insert("Which files do you want to keep?", "Které soubory chcete ponechat?");
        m.insert("If you select both versions, the copied file will have a number added to its name.", "Pokud zvolíte obě verze, zkopírovaný soubor bude mít název doplněn o číslo.");
        m.insert("Cancel", "Zrušit");
        m.insert("Continue", "Pokračovat");
        m.insert("(all selected)", "(vybráno vše)");
        m.insert("({count} selected)", "(vybráno {count})");
        m.insert("Error loading file exists template", "Chyba při nahrávání šablony existence souboru");
        m.insert("Shared", "Sdílené");
        m.insert("Share", "Sdílet");
        m.insert("Error", "Chyba");
        m.insert("Error while sharing", "Chyba při sdílení");
        m.insert("Error while unsharing", "Chyba při rušení sdílení");
        m.insert("Error while changing permissions", "Chyba při změně oprávnění");
        m.insert("Shared with you and the group {group} by {owner}", "S Vámi a skupinou {group} sdílí {owner}");
        m.insert("Shared with you by {owner}", "S Vámi sdílí {owner}");
        m.insert("Password protect", "Chránit heslem");
        m.insert("Password", "Heslo");
        m.insert("Allow Public Upload", "Povolit veřejné nahrávání");
        m.insert("Email link to person", "Odeslat osobě odkaz e-mailem");
        m.insert("Send", "Odeslat");
        m.insert("Set expiration date", "Nastavit datum vypršení platnosti");
        m.insert("Expiration date", "Datum vypršení platnosti");
        m.insert("Share via email:", "Sdílet e-mailem:");
        m.insert("No people found", "Žádní lidé nenalezeni");
        m.insert("group", "skupina");
        m.insert("Resharing is not allowed", "Sdílení již sdílené položky není povoleno");
        m.insert("Shared in {item} with {user}", "Sdíleno v {item} s {user}");
        m.insert("Unshare", "Zrušit sdílení");
        m.insert("can edit", "lze upravovat");
        m.insert("access control", "řízení přístupu");
        m.insert("create", "vytvořit");
        m.insert("update", "aktualizovat");
        m.insert("delete", "smazat");
        m.insert("share", "sdílet");
        m.insert("Password protected", "Chráněno heslem");
        m.insert("Error unsetting expiration date", "Chyba při odstraňování data vypršení platnosti");
        m.insert("Error setting expiration date", "Chyba při nastavení data vypršení platnosti");
        m.insert("Sending ...", "Odesílám ...");
        m.insert("Email sent", "E-mail odeslán");
        m.insert("Warning", "Varování");
        m.insert("The object type is not specified.", "Není určen typ objektu.");
        m.insert("Enter new", "Zadat nový");
        m.insert("Delete", "Smazat");
        m.insert("Add", "Přidat");
        m.insert("Edit tags", "Editovat štítky");
        m.insert("Error loading dialog template: {error}", "Chyba při načítání šablony dialogu: {error}");
        m.insert("No tags selected for deletion.", "Žádné štítky nebyly vybrány ke smazání.");
        m.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.", "Aktualizace neproběhla úspěšně. Nahlaste prosím problém do <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">evidence chyb ownCloud</a>");
        m.insert("The update was successful. Redirecting you to ownCloud now.", "Aktualizace byla úspěšná. Přesměrovávám na ownCloud.");
        m.insert("%s password reset", "reset hesla %s");
        m.insert("Use the following link to reset your password: {link}", "Heslo obnovíte použitím následujícího odkazu: {link}");
        m.insert("The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator .", "Odkaz na obnovení hesla byl odeslán na vaši e-mailovou adresu.<br>Pokud jej v krátké době neobdržíte, zkontrolujte váš koš a složku spam.<br>Pokud jej nenaleznete, kontaktujte svého správce.");
        m.insert("Request failed!<br>Did you make sure your email/username was right?", "Požadavek selhal!<br>Ujistili jste se, že vaše uživatelské jméno a e-mail jsou správně?");
        m.insert("You will receive a link to reset your password via Email.", "E-mailem Vám bude zaslán odkaz pro obnovu hesla.");
        m.insert("Username", "Uživatelské jméno");
        m.insert("Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?", "Vaše soubory jsou šifrovány. Pokud nemáte povolen klíč pro obnovu, neexistuje způsob jak získat, po změně hesla, vaše data. Pokud si nejste jisti co dělat, kontaktujte nejprve svého správce. Opravdu si přejete pokračovat?");
        m.insert("Yes, I really want to reset my password now", "Ano, opravdu si nyní přeji obnovit mé heslo");
        m.insert("Reset", "Restartovat složku");
        m.insert("Your password was reset", "Vaše heslo bylo obnoveno");
        m.insert("To login page", "Na stránku přihlášení");
        m.insert("New password", "Nové heslo");
        m.insert("Reset password", "Obnovit heslo");
        m.insert("Personal", "Osobní");
        m.insert("Users", "Uživatelé");
        m.insert("Apps", "Aplikace");
        m.insert("Admin", "Administrace");
        m.insert("Help", "Nápověda");
        m.insert("Error loading tags", "Chyba při načítání štítků");
        m.insert("Tag already exists", "Štítek již existuje");
        m.insert("Error deleting tag(s)", "Chyba při mazání štítku(ů)");
        m.insert("Error tagging", "Chyba při označování štítkem");
        m.insert("Error untagging", "Chyba při odznačování štítků");
        m.insert("Error favoriting", "Chyba při označování jako oblíbené");
        m.insert("Error unfavoriting", "Chyba při odznačování jako oblíbené");
        m.insert("Access forbidden", "Přístup zakázán");
        m.insert("Cloud not found", "Cloud nebyl nalezen");
        m.insert("Hey there,\n\njust letting you know that %s shared %s with you.\nView it: %s\n\n", "Hej ty tam,\n\njen ti chci dát vědět, že %s sdílel %s s tebou.\nZobraz si to: %s\n\n");
        m.insert("The share will expire on %s.\n\n", "Sdílení expiruje %s.\n\n");
        m.insert("Cheers!", "Ať slouží!");
        m.insert("Security Warning", "Bezpečnostní upozornění");
        m.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)", "Verze vašeho PHP je napadnutelná pomocí techniky \"NULL Byte\" (CVE-2006-7243)");
        m.insert("Please update your PHP installation to use %s securely.", "Aktualizujte prosím vaši instanci PHP pro bezpečné používání %s.");
        m.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", "Není dostupný žádný bezpečný generátor náhodných čísel. Povolte, prosím, rozšíření OpenSSL v PHP.");
        m.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "Bez bezpečného generátoru náhodných čísel může útočník předpovědět token pro obnovu hesla a převzít kontrolu nad Vaším účtem.");
        m.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.", "Váš adresář s daty a soubory jsou dostupné z internetu, protože soubor .htaccess nefunguje.");
        m.insert("For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>.", "Pro informace, jak správně nastavit váš server, se podívejte do <a href=\"%s\" target=\"_blank\">dokumentace</a>.");
        m.insert("Create an <strong>admin account</strong>", "Vytvořit <strong>účet správce</strong>");
        m.insert("Advanced", "Pokročilé");
        m.insert("Data folder", "Složka s daty");
        m.insert("Configure the database", "Nastavit databázi");
        m.insert("will be used", "bude použito");
        m.insert("Database user", "Uživatel databáze");
        m.insert("Database password", "Heslo databáze");
        m.insert("Database name", "Název databáze");
        m.insert("Database tablespace", "Tabulkový prostor databáze");
        m.insert("Database host", "Hostitel databáze");
        m.insert("Finish setup", "Dokončit nastavení");
        m.insert("Finishing …", "Dokončuji...");
        m.insert("%s is available. Get more information on how to update.", "%s je dostupná. Získejte více informací k postupu aktualizace.");
        m.insert("Log out", "Odhlásit se");
        m.insert("Automatic logon rejected!", "Automatické přihlášení odmítnuto!");
        m.insert("If you did not change your password recently, your account may be compromised!", "Pokud jste v nedávné době neměnili své heslo, Váš účet může být kompromitován!");
        m.insert("Please change your password to secure your account again.", "Změňte, prosím, své heslo pro opětovné zabezpečení Vašeho účtu.");
        m.insert("Server side authentication failed!", "Autentizace na serveru selhala!");
        m.insert("Please contact your administrator.", "Kontaktujte prosím vašeho správce.");
        m.insert("Lost your password?", "Ztratili jste své heslo?");
        m.insert("remember", "zapamatovat");
        m.insert("Log in", "Přihlásit");
        m.insert("Alternative Logins", "Alternativní přihlášení");
        m.insert("Hey there,<br><br>just letting you know that %s shared »%s« with you.<br><a href=\"%s\">View it!</a><br><br>", "Hej ty tam,<br><br>jen ti chci dát vědět, že %s sdílel »%s« s tebou.<br><a href=\"%s\">Zobrazit!</a><br><br>");
        m.insert("The share will expire on %s.<br><br>", "Sdílení expiruje %s.<br><br>");
        m.insert("Updating ownCloud to version %s, this may take a while.", "Aktualizuji ownCloud na verzi %s, bude to chvíli trvat.");
        m
    };

    pub static ref PLURAL_FORMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n minute ago_::_%n minutes ago_", vec!["před %n minutou", "před %n minutami", "před %n minutami"]);
        m.insert("_%n hour ago_::_%n hours ago_", vec!["před %n hodinou", "před %n hodinami", "před %n hodinami"]);
        m.insert("_%n day ago_::_%n days ago_", vec!["před %n dnem", "před %n dny", "před %n dny"]);
        m.insert("_%n month ago_::_%n months ago_", vec!["před %n měsícem", "před %n měsíci", "před %n měsíci"]);
        m.insert("_{count} file conflict_::_{count} file conflicts_", vec!["{count} souborový konflikt", "{count} souborové konflikty", "{count} souborových konfliktů"]);
        m
    };
}

pub fn get_plural_form(n: i64) -> usize {
    if n == 1 {
        0
    } else if n >= 2 && n <= 4 {
        1
    } else {
        2
    }
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_translation(key: &str, n: i64) -> Option<&'static str> {
    PLURAL_FORMS.get(key).and_then(|forms| {
        let idx = get_plural_form(n);
        forms.get(idx).copied()
    })
}