use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("%s shared »%s« with you", "%s pasidalino »%s« su tavimi");
    map.insert("Couldn't send mail to following users: %s ", "Nepavyko nusiųsti el. pašto šiems naudotojams: %s ");
    map.insert("Turned on maintenance mode", "Įjungta priežiūros veiksena");
    map.insert("Turned off maintenance mode", "Išjungta priežiūros veiksena");
    map.insert("Updated database", "Atnaujinta duomenų bazė");
    map.insert("Updating filecache, this may take really long...", "Atnaujinama failų talpykla, tai gali užtrukti labai ilgai...");
    map.insert("Updated filecache", "Atnaujinta failų talpykla");
    map.insert("... %d%% done ...", "... %d%% atlikta ...");
    map.insert("No image or file provided", "Nenurodytas paveikslėlis ar failas");
    map.insert("Unknown filetype", "Nežinomas failo tipas");
    map.insert("Invalid image", "Netinkamas paveikslėlis");
    map.insert("No temporary profile picture available, try again", "Nėra laikino profilio paveikslėlio, bandykite dar kartą");
    map.insert("No crop data provided", "Nenurodyti apkirpimo duomenys");
    map.insert("Sunday", "Sekmadienis");
    map.insert("Monday", "Pirmadienis");
    map.insert("Tuesday", "Antradienis");
    map.insert("Wednesday", "Trečiadienis");
    map.insert("Thursday", "Ketvirtadienis");
    map.insert("Friday", "Penktadienis");
    map.insert("Saturday", "Šeštadienis");
    map.insert("January", "Sausis");
    map.insert("February", "Vasaris");
    map.insert("March", "Kovas");
    map.insert("April", "Balandis");
    map.insert("May", "Gegužė");
    map.insert("June", "Birželis");
    map.insert("July", "Liepa");
    map.insert("August", "Rugpjūtis");
    map.insert("September", "Rugsėjis");
    map.insert("October", "Spalis");
    map.insert("November", "Lapkritis");
    map.insert("December", "Gruodis");
    map.insert("Settings", "Nustatymai");
    map.insert("seconds ago", "prieš sekundę");
    map.insert("today", "šiandien");
    map.insert("yesterday", "vakar");
    map.insert("last month", "praeitą mėnesį");
    map.insert("months ago", "prieš mėnesį");
    map.insert("last year", "praeitais metais");
    map.insert("years ago", "prieš metus");
    map.insert("Choose", "Pasirinkite");
    map.insert("Error loading file picker template: {error}", "Klaida įkeliant failo parinkimo ruošinį: {error}");
    map.insert("Yes", "Taip");
    map.insert("No", "Ne");
    map.insert("Ok", "Gerai");
    map.insert("Error loading message template: {error}", "Klaida įkeliant žinutės ruošinį: {error}");
    map.insert("One file conflict", "Vienas failo konfliktas");
    map.insert("Which files do you want to keep?", "Kuriuos failus norite laikyti?");
    map.insert("If you select both versions, the copied file will have a number added to its name.", "Jei pasirenkate abi versijas, nukopijuotas failas turės pridėtą numerį pavadinime.");
    map.insert("Cancel", "Atšaukti");
    map.insert("Continue", "Tęsti");
    map.insert("(all selected)", "(visi pažymėti)");
    map.insert("Error loading file exists template", "Klaida įkeliant esančių failų ruošinį");
    map.insert("Shared", "Dalinamasi");
    map.insert("Share", "Dalintis");
    map.insert("Error", "Klaida");
    map.insert("Error while sharing", "Klaida, dalijimosi metu");
    map.insert("Error while unsharing", "Klaida, kai atšaukiamas dalijimasis");
    map.insert("Error while changing permissions", "Klaida, keičiant privilegijas");
    map.insert("Shared with you and the group {group} by {owner}", "Pasidalino su Jumis ir {group} grupe {owner}");
    map.insert("Shared with you by {owner}", "Pasidalino su Jumis {owner}");
    map.insert("Share with user or group …", "Dalintis su vartotoju arba grupe...");
    map.insert("Share link", "Dalintis nuoroda");
    map.insert("Password protect", "Apsaugotas slaptažodžiu");
    map.insert("Password", "Slaptažodis");
    map.insert("Allow Public Upload", "Leisti viešą įkėlimą");
    map.insert("Email link to person", "Nusiųsti nuorodą paštu");
    map.insert("Send", "Siųsti");
    map.insert("Set expiration date", "Nustatykite galiojimo laiką");
    map.insert("Expiration date", "Galiojimo laikas");
    map.insert("Share via email:", "Dalintis per el. paštą:");
    map.insert("No people found", "Žmonių nerasta");
    map.insert("group", "grupė");
    map.insert("Resharing is not allowed", "Dalijinasis išnaujo negalimas");
    map.insert("Shared in {item} with {user}", "Pasidalino {item} su {user}");
    map.insert("Unshare", "Nebesidalinti");
    map.insert("notify by email", "pranešti el. paštu");
    map.insert("can edit", "gali redaguoti");
    map.insert("access control", "priėjimo kontrolė");
    map.insert("create", "sukurti");
    map.insert("update", "atnaujinti");
    map.insert("delete", "ištrinti");
    map.insert("share", "dalintis");
    map.insert("Password protected", "Apsaugota slaptažodžiu");
    map.insert("Error unsetting expiration date", "Klaida nuimant galiojimo laiką");
    map.insert("Error setting expiration date", "Klaida nustatant galiojimo laiką");
    map.insert("Sending ...", "Siunčiama...");
    map.insert("Email sent", "Laiškas išsiųstas");
    map.insert("Warning", "Įspėjimas");
    map.insert("The object type is not specified.", "Objekto tipas nenurodytas.");
    map.insert("Enter new", "Įveskite naują");
    map.insert("Delete", "Ištrinti");
    map.insert("Add", "Pridėti");
    map.insert("Edit tags", "Redaguoti žymes");
    map.insert("Error loading dialog template: {error}", "Klaida įkeliant dialogo ruošinį: {error}");
    map.insert("No tags selected for deletion.", "Trynimui nepasirinkta jokia žymė.");
    map.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.", "Atnaujinimas buvo nesėkmingas. PApie tai prašome pranešti the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud bendruomenei</a>.");
    map.insert("The update was successful. Redirecting you to ownCloud now.", "Atnaujinimas buvo sėkmingas. Nukreipiame į jūsų ownCloud.");
    map.insert("%s password reset", "%s slaptažodžio atnaujinimas");
    map.insert("Use the following link to reset your password: {link}", "Slaptažodžio atkūrimui naudokite šią nuorodą: {link}");
    map.insert("The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator .", "Nuorodą su jūsų slaptažodžio atkūrimu buvo nusiųsta jums į paštą.<br>Jei jo negausite per atitinkamą laiką, pasižiūrėkite brukalo aplankale.<br> Jei jo ir ten nėra, teiraukitės administratoriaus.");
    map.insert("Request failed!<br>Did you make sure your email/username was right?", "Klaida!<br>Ar tikrai jūsų el paštas/vartotojo vardas buvo teisingi?");
    map.insert("You will receive a link to reset your password via Email.", "Elektroniniu paštu gausite nuorodą, su kuria galėsite iš naujo nustatyti slaptažodį.");
    map.insert("Username", "Prisijungimo vardas");
    map.insert("Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?", "Jūsų failai yra užšifruoti. Jei neįjungėte atstatymo rakto, nebus galimybės atstatyti duomenų po slaptažodžio atstatymo. Jei nesate tikri ką daryti, prašome susisiekti su administratoriumi prie tęsiant. Ar tikrai tęsti?");
    map.insert("Yes, I really want to reset my password now", "Taip, aš tikrai noriu atnaujinti slaptažodį");
    map.insert("Your password was reset", "Jūsų slaptažodis buvo nustatytas iš naujo");
    map.insert("To login page", "Į prisijungimo puslapį");
    map.insert("New password", "Naujas slaptažodis");
    map.insert("Reset password", "Atkurti slaptažodį");
    map.insert("Personal", "Asmeniniai");
    map.insert("Users", "Vartotojai");
    map.insert("Apps", "Programos");
    map.insert("Admin", "Administravimas");
    map.insert("Help", "Pagalba");
    map.insert("Error loading tags", "Klaida įkeliant žymes");
    map.insert("Tag already exists", "Žymė jau egzistuoja");
    map.insert("Error deleting tag(s)", "Klaida trinant žymę(-es)");
    map.insert("Error tagging", "Klaida pridedant žymę");
    map.insert("Error untagging", "Klaida šalinant žymę");
    map.insert("Access forbidden", "Priėjimas draudžiamas");
    map.insert("Cloud not found", "Negalima rasti");
    map.insert("Hey there,\n\njust letting you know that %s shared %s with you.\nView it: %s\n\n", "Labas,\n\nInformuojame, kad %s pasidalino su Jumis %s.\nPažiūrėti tai: %s\n");
    map.insert("The share will expire on %s.\n\n", "Bendrinimo laikas baigsis %s.\n");
    map.insert("Cheers!", "Sveikinimai!");
    map.insert("Security Warning", "Saugumo pranešimas");
    map.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)", "Jūsų PHP versija yra pažeidžiama prieš NULL Byte ataką (CVE-2006-7243)");
    map.insert("Please update your PHP installation to use %s securely.", "Prašome atnaujinti savo PHP, kad saugiai naudoti %s.");
    map.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", "Saugaus atsitiktinių skaičių generatoriaus nėra, prašome įjungti PHP OpenSSL modulį.");
    map.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "Be saugaus atsitiktinių skaičių generatoriaus, piktavaliai gali atspėti Jūsų slaptažodį ir pasisavinti paskyrą.");
    map.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.", "Jūsų failai yra tikriausiai prieinami per internetą nes .htaccess failas neveikia.");
    map.insert("For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>.", "Kad gauti informaciją apie tai kaip tinkamai sukonfigūruoti savo serverį, prašome skaityti <a href=\"%s\" target=\"_blank\">dokumentaciją</a>.");
    map.insert("Create an <strong>admin account</strong>", "Sukurti <strong>administratoriaus paskyrą</strong>");
    map.insert("Advanced", "Išplėstiniai");
    map.insert("Data folder", "Duomenų katalogas");
    map.insert("Configure the database", "Nustatyti duomenų bazę");
    map.insert("will be used", "bus naudojama");
    map.insert("Database user", "Duomenų bazės vartotojas");
    map.insert("Database password", "Duomenų bazės slaptažodis");
    map.insert("Database name", "Duomenų bazės pavadinimas");
    map.insert("Database tablespace", "Duomenų bazės loginis saugojimas");
    map.insert("Database host", "Duomenų bazės serveris");
    map.insert("Finish setup", "Baigti diegimą");
    map.insert("Finishing …", "Baigiama ...");
    map.insert("%s is available. Get more information on how to update.", "%s yra prieinama. Gaukite daugiau informacijos apie atnaujinimą.");
    map.insert("Log out", "Atsijungti");
    map.insert("Automatic logon rejected!", "Automatinis prisijungimas atmestas!");
    map.insert("If you did not change your password recently, your account may be compromised!", "Jei paskutinių metu nekeitėte savo slaptažodžio, Jūsų paskyra gali būti pavojuje!");
    map.insert("Please change your password to secure your account again.", "Prašome pasikeisti slaptažodį dar kartą, dėl paskyros saugumo.");
    map.insert("Server side authentication failed!", "Autentikacija serveryje nepavyko!");
    map.insert("Please contact your administrator.", "Kreipkitės į savo sistemos administratorių.");
    map.insert("Lost your password?", "Pamiršote slaptažodį?");
    map.insert("remember", "prisiminti");
    map.insert("Log in", "Prisijungti");
    map.insert("Alternative Logins", "Alternatyvūs prisijungimai");
    map.insert("Hey there,<br><br>just letting you know that %s shared »%s« with you.<br><a href=\"%s\">View it!</a><br><br>", "Labas,<br><br>tik informuojame, kad %s pasidalino su Jumis »%s«.<br><a href=\"%s\">Peržiūrėk!</a><br><br>");
    map.insert("The share will expire on %s.<br><br>", "Bendrinimo laikas baigsis %s.<br><br>");
    map.insert("Updating ownCloud to version %s, this may take a while.", "Atnaujinama ownCloud į %s versiją. tai gali šiek tiek užtrukti.");
    map.insert("This ownCloud instance is currently being updated, which may take a while.", "Šiuo metu vyksta ownCloud atnaujinamas, tai gali šiek tiek užtrukti.");
    map.insert("Please reload this page after a short time to continue using ownCloud.", "Po trupučio laiko atnaujinkite šį puslapį kad galėtumėte toliau naudoti ownCloud.");
    map.insert("Contact your system administrator if this message persists or appeared unexpectedly.", "Susisiekite su savo sistemos administratoriumi jei šis pranešimas nedingsta arba jei jis pasirodė netikėtai.");
    map.insert("Thank you for your patience.", "Dėkojame už jūsų kantrumą.");
    map
});

pub static PLURAL_FORMS: Lazy<HashMap<&'static str, Vec<&'static str>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("_%n minute ago_::_%n minutes ago_", vec![" prieš %n minutę", " prieš %n minučių", " prieš %n minučių"]);
    map.insert("_%n hour ago_::_%n hours ago_", vec!["prieš %n valandą", "prieš %n valandų", "prieš %n valandų"]);
    map.insert("_%n day ago_::_%n days ago_", vec!["prieš %n dieną", "prieš %n dienas", "prieš %n dienų"]);
    map.insert("_%n month ago_::_%n months ago_", vec!["prieš %n mėnesį", "prieš %n mėnesius", "prieš %n mėnesių"]);
    map.insert("_{count} file conflict_::_{count} file conflicts_", vec!["{count} failas konfliktuoja", "{count} failai konfliktuoja", "{count} failų konfliktų"]);
    map
});

pub fn get_plural_form(n: i64) -> usize {
    if n % 10 == 1 && n % 100 != 11 {
        0
    } else if n % 10 >= 2 && (n % 100 < 10 || n % 100 >= 20) {
        1
    } else {
        2
    }
}

pub fn translate(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn translate_plural(key: &str, count: i64) -> Option<&'static str> {
    PLURAL_FORMS
        .get(key)
        .and_then(|forms| forms.get(get_plural_form(count)).copied())
}