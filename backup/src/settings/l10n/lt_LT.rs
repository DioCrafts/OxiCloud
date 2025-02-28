use std::collections::HashMap;
use rust_i18n::i18n;

i18n!("lt_LT");

pub fn create_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Unable to load list from App Store", "Neįmanoma įkelti sąrašo iš Programų Katalogo");
    translations.insert("Authentication error", "Autentikacijos klaida");
    translations.insert("Group already exists", "Grupė jau egzistuoja");
    translations.insert("Unable to add group", "Nepavyko pridėti grupės");
    translations.insert("Email saved", "El. paštas išsaugotas");
    translations.insert("Invalid email", "Netinkamas el. paštas");
    translations.insert("Unable to delete group", "Nepavyko ištrinti grupės");
    translations.insert("Unable to delete user", "Nepavyko ištrinti vartotojo");
    translations.insert("Language changed", "Kalba pakeista");
    translations.insert("Invalid request", "Klaidinga užklausa");
    translations.insert("Admins can't remove themself from the admin group", "Administratoriai negali pašalinti savęs iš administratorių grupės");
    translations.insert("Unable to add user to group %s", "Nepavyko pridėti vartotojo prie grupės %s");
    translations.insert("Unable to remove user from group %s", "Nepavyko ištrinti vartotojo iš grupės %s");
    translations.insert("Couldn't update app.", "Nepavyko atnaujinti programos.");
    translations.insert("Wrong password", "Neteisingas slaptažodis");
    translations.insert("No user supplied", "Nepateiktas naudotojas");
    translations.insert("Please provide an admin recovery password, otherwise all user data will be lost", "Prašome įvesti administratoriaus atkūrimo slaptažodį, kitaip visi naudotojo suomenys bus prarasti");
    translations.insert("Wrong admin recovery password. Please check the password and try again.", "Netinkamas administratoriau atkūrimo slaptažodis. Prašome pasitikrinti ir bandyti vėl.");
    translations.insert("Back-end doesn't support password change, but the users encryption key was successfully updated.", "Sistema nepalaiko slaptažodžio keitimo, bet naudotojo šifravimo raktas buvo sėkmingai atnaujintas.");
    translations.insert("Unable to change password", "Nepavyksta pakeisti slaptažodžio");
    translations.insert("Update to {appversion}", "Atnaujinti iki {appversion}");
    translations.insert("Disable", "Išjungti");
    translations.insert("Enable", "Įjungti");
    translations.insert("Please wait....", "Prašome palaukti...");
    translations.insert("Error while disabling app", "Klaida išjungiant programą");
    translations.insert("Error while enabling app", "Klaida įjungiant programą");
    translations.insert("Updating....", "Atnaujinama...");
    translations.insert("Error while updating app", "Įvyko klaida atnaujinant programą");
    translations.insert("Error", "Klaida");
    translations.insert("Update", "Atnaujinti");
    translations.insert("Updated", "Atnaujinta");
    translations.insert("Select a profile picture", "Pažymėkite profilio paveikslėlį");
    translations.insert("Decrypting files... Please wait, this can take some time.", "Iššifruojami failai... Prašome palaukti, tai gali užtrukti.");
    translations.insert("Saving...", "Saugoma...");
    translations.insert("deleted", "ištrinta");
    translations.insert("undo", "anuliuoti");
    translations.insert("Unable to remove user", "Nepavyko ištrinti vartotojo");
    translations.insert("Groups", "Grupės");
    translations.insert("Group Admin", "Grupės administratorius");
    translations.insert("Delete", "Ištrinti");
    translations.insert("add group", "pridėti grupę");
    translations.insert("A valid username must be provided", "Vartotojo vardas turi būti tinkamas");
    translations.insert("Error creating user", "Klaida kuriant vartotoją");
    translations.insert("A valid password must be provided", "Slaptažodis turi būti tinkamas");
    translations.insert("Warning: Home directory for user \"{user}\" already exists", "Įspėjimas: Vartotojo \"{user}\" namų aplankas jau egzistuoja");
    translations.insert("__language_name__", "Kalba");
    translations.insert("Security Warning", "Saugumo pranešimas");
    translations.insert("Your data directory and your files are probably accessible from the internet. The .htaccess file is not working. We strongly suggest that you configure your webserver in a way that the data directory is no longer accessible or you move the data directory outside the webserver document root.", "Jūsų duomenų katalogas ir Jūsų failai turbūt yra pasiekiami per internetą. Failas .htaccess neveikia. Mes labai rekomenduojame sukonfigūruoti serverį taip, kad katalogas nebūtų daugiau pasiekiamas, arba iškelkite duomenis kitur iš webserverio šakninio aplanko.");
    translations.insert("Setup Warning", "Nustatyti perspėjimą");
    translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Jūsų serveris nėra tvarkingai nustatytas leisti failų sinchronizaciją, nes WebDAV sąsaja panašu, kad yra sugadinta.");
    translations.insert("Please double check the <a href=\"%s\">installation guides</a>.", "Prašome pažiūrėkite dar kartą <a href=\"%s\">diegimo instrukcijas</a>.");
    translations.insert("Module 'fileinfo' missing", "Trūksta 'fileinfo' modulio");
    translations.insert("The PHP module 'fileinfo' is missing. We strongly recommend to enable this module to get best results with mime-type detection.", "Trūksta PHP modulio „fileinfo". Labai rekomenduojame įjungti šį modulį, kad gauti geriausius rezultatus nustatant mime-tipą.");
    translations.insert("Locale not working", "Lokalė neveikia");
    translations.insert("System locale can't be set to %s. This means that there might be problems with certain characters in file names. We strongly suggest to install the required packages on your system to support %s.", "Negalima nustatyti sistemos lokalės į %s. Tai reiškia, kad gali būti problemų su tam tikrais simboliais failų pavadinimuose. Labai rekomenduojame įdiegti reikalingus paketus Jūsų sistemoje, kad palaikyti %s.");
    translations.insert("Internet connection not working", "Nėra interneto ryšio");
    translations.insert("This server has no working internet connection. This means that some of the features like mounting of external storage, notifications about updates or installation of 3rd party apps don´t work. Accessing files from remote and sending of notification emails might also not work. We suggest to enable internet connection for this server if you want to have all features.", "Šis serveris neturi veikiančio ryšio. Tai reiškia, kas kai kurios funkcijos kaip išorinės saugyklos prijungimas, perspėjimai apie atnaujinimus ar trečių šalių programų įdiegimas neveikia. Failų pasiekimas iš kitur ir pranešimų siuntimas el. paštu gali taip pat neveikti. Rekomenduojame įjungti interneto ryšį šiame serveryje, jei norite naudoti visas funkcijas.");
    translations.insert("Cron", "Cron");
    translations.insert("Execute one task with each page loaded", "Įvykdyti vieną užduotį su kiekvieno puslapio įkėlimu");
    translations.insert("cron.php is registered at a webcron service to call cron.php every 15 minutes over http.", "cron.php yra registruotas tinklapio suplanuotų užduočių paslaugose, kad iškviesti cron.php kas 15 minučių per http.");
    translations.insert("Use systems cron service to call the cron.php file every 15 minutes.", "Naudoti sistemos planuotų užduočių paslaugą, kad iškvieti cron.php kas 15 minučių.");
    translations.insert("Sharing", "Dalijimasis");
    translations.insert("Enable Share API", "Įjungti Share API");
    translations.insert("Allow apps to use the Share API", "Leidžia programoms naudoti Share API");
    translations.insert("Allow links", "Lesti nuorodas");
    translations.insert("Allow users to share items to the public with links", "Leisti naudotojams viešai dalintis elementais su nuorodomis");
    translations.insert("Allow public uploads", "Leisti viešus įkėlimus");
    translations.insert("Allow users to enable others to upload into their publicly shared folders", "Leisti naudotojams įgalinti kitus įkelti į savo viešai dalinamus aplankus");
    translations.insert("Allow resharing", "Leisti dalintis");
    translations.insert("Allow users to share items shared with them again", "Leisti naudotojams toliau dalintis elementais pasidalintais su jais");
    translations.insert("Allow users to share with anyone", "Leisti naudotojams dalintis su bet kuo");
    translations.insert("Allow users to only share with users in their groups", "Leisti naudotojams dalintis tik su naudotojais savo grupėje");
    translations.insert("Allow mail notification", "Leisti el. pašto perspėjimą");
    translations.insert("Allow user to send mail notification for shared files", "Leisti naudotojui siųsti perspėjimą el. laišku dėl bendrinamų failų");
    translations.insert("Security", "Saugumas");
    translations.insert("Enforce HTTPS", "Reikalauti HTTPS");
    translations.insert("Forces the clients to connect to %s via an encrypted connection.", "Verčia klientus jungtis prie %s per šifruotą ryšį.");
    translations.insert("Please connect to your %s via HTTPS to enable or disable the SSL enforcement.", "Prašome prisijungti prie savo %s per HTTPS, kad įjungti ar išjungti SSL reikalavimą.");
    translations.insert("Log", "Žurnalas");
    translations.insert("Log level", "Žurnalo išsamumas");
    translations.insert("More", "Daugiau");
    translations.insert("Less", "Mažiau");
    translations.insert("Version", "Versija");
    translations.insert("Developed by the <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud community</a>, the <a href=\"https://github.com/owncloud\" target=\"_blank\">source code</a> is licensed under the <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.", "Sukurta <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud bendruomenės</a>, <a href=\"https://github.com/owncloud\" target=\"_blank\">pirminis kodas</a> platinamas pagal <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.");
    translations.insert("Add your App", "Pridėti programėlę");
    translations.insert("More Apps", "Daugiau aplikacijų");
    translations.insert("Select an App", "Pasirinkite programą");
    translations.insert("See application page at apps.owncloud.com", "Žiūrėti programos puslapį svetainėje apps.owncloud.com");
    translations.insert("<span class=\"licence\"></span>-licensed by <span class=\"author\"></span>", "<span class=\"licence\"></span>- autorius<span class=\"author\"></span>");
    translations.insert("User Documentation", "Naudotojo dokumentacija");
    translations.insert("Administrator Documentation", "Administratoriaus dokumentacija");
    translations.insert("Online Documentation", "Dokumentacija tinkle");
    translations.insert("Forum", "Forumas");
    translations.insert("Bugtracker", "Klaidų sekimas");
    translations.insert("Commercial Support", "Komercinis palaikymas");
    translations.insert("Get the apps to sync your files", "Atsisiųskite programėlių, kad sinchronizuotumėte savo failus");
    translations.insert("Show First Run Wizard again", "Rodyti pirmo karto vedlį dar kartą");
    translations.insert("You have used <strong>%s</strong> of the available <strong>%s</strong>", "Jūs naudojate <strong>%s</strong> iš galimų <strong>%s</strong>");
    translations.insert("Password", "Slaptažodis");
    translations.insert("Your password was changed", "Jūsų slaptažodis buvo pakeistas");
    translations.insert("Unable to change your password", "Neįmanoma pakeisti slaptažodžio");
    translations.insert("Current password", "Dabartinis slaptažodis");
    translations.insert("New password", "Naujas slaptažodis");
    translations.insert("Change password", "Pakeisti slaptažodį");
    translations.insert("Email", "El. Paštas");
    translations.insert("Your email address", "Jūsų el. pašto adresas");
    translations.insert("Fill in an email address to enable password recovery", "Pamiršto slaptažodžio atkūrimui įveskite savo el. pašto adresą");
    translations.insert("Profile picture", "Profilio paveikslėlis");
    translations.insert("Upload new", "Įkelti naują");
    translations.insert("Select new from Files", "Pasirinkti naują iš failų");
    translations.insert("Remove image", "Pašalinti paveikslėlį");
    translations.insert("Either png or jpg. Ideally square but you will be able to crop it.", "Arba png arba jpg. Geriausia kvadratinį, bet galėsite jį apkarpyti.");
    translations.insert("Abort", "Atšaukti");
    translations.insert("Choose as profile image", "Pasirinkite profilio paveiksliuką");
    translations.insert("Language", "Kalba");
    translations.insert("Help translate", "Padėkite išversti");
    translations.insert("WebDAV", "WebDAV");
    translations.insert("Use this address to <a href=\"%s\" target=\"_blank\">access your Files via WebDAV</a>", "Naudokite šį adresą, kad <a href=\"%s\" target=\"_blank\">pasiektumėte savo failus per WebDAV</a>");
    translations.insert("Encryption", "Šifravimas");
    translations.insert("The encryption app is no longer enabled, decrypt all your file", "Šifravimo programa nebėra įjungta, iššifruokite visus savo failus");
    translations.insert("Log-in password", "Prisijungimo slaptažodis");
    translations.insert("Decrypt all Files", "Iššifruoti visus failus");
    translations.insert("Login Name", "Vartotojo vardas");
    translations.insert("Create", "Sukurti");
    translations.insert("Admin Recovery Password", "Administracinis atkūrimo slaptažodis");
    translations.insert("Enter the recovery password in order to recover the users files during password change", "Įveskite atkūrimo slaptažodį, kad atkurti naudotojo failus keičiant slaptažodį");
    translations.insert("Default Storage", "Numatytas saugojimas");
    translations.insert("Unlimited", "Neribota");
    translations.insert("Other", "Kita");
    translations.insert("Username", "Prisijungimo vardas");
    translations.insert("Storage", "Saugojimas");
    translations.insert("set new password", "nustatyti naują slaptažodį");
    translations.insert("Default", "Numatytasis");
    translations
}

#[allow(dead_code)]
fn get_plural_form(n: i64) -> usize {
    if n % 10 == 1 && n % 100 != 11 {
        0
    } else if n % 10 >= 2 && (n % 100 < 10 || n % 100 >= 20) {
        1
    } else {
        2
    }
}

pub fn get_translation(key: &str) -> &'static str {
    let translations = create_translations();
    translations.get(key).copied().unwrap_or(key)
}