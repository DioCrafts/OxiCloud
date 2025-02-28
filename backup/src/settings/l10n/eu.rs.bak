use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Unable to load list from App Store", "Ezin izan da App Dendatik zerrenda kargatu");
        m.insert("Authentication error", "Autentifikazio errorea");
        m.insert("Group already exists", "Taldea dagoeneko existitzenda");
        m.insert("Unable to add group", "Ezin izan da taldea gehitu");
        m.insert("Email saved", "Eposta gorde da");
        m.insert("Invalid email", "Baliogabeko eposta");
        m.insert("Unable to delete group", "Ezin izan da taldea ezabatu");
        m.insert("Unable to delete user", "Ezin izan da erabiltzailea ezabatu");
        m.insert("Language changed", "Hizkuntza aldatuta");
        m.insert("Invalid request", "Baliogabeko eskaera");
        m.insert("Admins can't remove themself from the admin group", "Kudeatzaileak ezin du bere burua kendu kudeatzaile taldetik");
        m.insert("Unable to add user to group %s", "Ezin izan da erabiltzailea %s taldera gehitu");
        m.insert("Unable to remove user from group %s", "Ezin izan da erabiltzailea %s taldetik ezabatu");
        m.insert("Couldn't update app.", "Ezin izan da aplikazioa eguneratu.");
        m.insert("Wrong password", "Pasahitz okerra");
        m.insert("Update to {appversion}", "Eguneratu {appversion}-ra");
        m.insert("Disable", "Ez-gaitu");
        m.insert("Enable", "Gaitu");
        m.insert("Please wait....", "Itxoin mesedez...");
        m.insert("Updating....", "Eguneratzen...");
        m.insert("Error while updating app", "Errorea aplikazioa eguneratzen zen bitartean");
        m.insert("Error", "Errorea");
        m.insert("Update", "Eguneratu");
        m.insert("Updated", "Eguneratuta");
        m.insert("Select a profile picture", "Profil argazkia aukeratu");
        m.insert("Saving...", "Gordetzen...");
        m.insert("deleted", "ezabatuta");
        m.insert("undo", "desegin");
        m.insert("Unable to remove user", "Ezin izan da erabiltzailea aldatu");
        m.insert("Groups", "Taldeak");
        m.insert("Group Admin", "Talde administradorea");
        m.insert("Delete", "Ezabatu");
        m.insert("add group", "gehitu taldea");
        m.insert("A valid username must be provided", "Baliozko erabiltzaile izena eman behar da");
        m.insert("Error creating user", "Errore bat egon da erabiltzailea sortzean");
        m.insert("A valid password must be provided", "Baliozko pasahitza eman behar da");
        m.insert("__language_name__", "Euskera");
        m.insert("Security Warning", "Segurtasun abisua");
        m.insert("Setup Warning", "Konfiguratu Abisuak");
        m.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Zure web zerbitzaria ez dago oraindik ongi konfiguratuta fitxategien sinkronizazioa egiteko, WebDAV interfazea ongi ez dagoela dirudi.");
        m.insert("Please double check the <a href=\"%s\">installation guides</a>.", "Mesedez birpasatu <a href=\"%s\">instalazio gidak</a>.");
        m.insert("Module 'fileinfo' missing", "'fileinfo' Modulua falta da");
        m.insert("The PHP module 'fileinfo' is missing. We strongly recommend to enable this module to get best results with mime-type detection.", "PHP 'fileinfo' modulua falta da. Modulu hau gaitzea aholkatzen dizugu mime-type ezberdinak hobe detektatzeko.");
        m.insert("Locale not working", "Lokala ez dabil");
        m.insert("Internet connection not working", "Interneteko konexioak ez du funtzionatzen");
        m.insert("Cron", "Cron");
        m.insert("Execute one task with each page loaded", "Exekutatu zeregin bat orri karga bakoitzean");
        m.insert("Sharing", "Partekatzea");
        m.insert("Enable Share API", "Gaitu Elkarbanatze APIa");
        m.insert("Allow apps to use the Share API", "Baimendu aplikazioak Elkarbanatze APIa erabiltzeko");
        m.insert("Allow links", "Baimendu loturak");
        m.insert("Allow users to share items to the public with links", "Baimendu erabiltzaileak loturen bidez fitxategiak publikoki elkarbanatzen");
        m.insert("Allow public uploads", "Baimendu igoera publikoak");
        m.insert("Allow users to enable others to upload into their publicly shared folders", "Baimendu erabiltzaileak besteak bere partekatutako karpetetan fitxategiak igotzea");
        m.insert("Allow resharing", "Baimendu birpartekatzea");
        m.insert("Allow users to share items shared with them again", "Baimendu erabiltzaileak haiekin elkarbanatutako fitxategiak berriz ere elkarbanatzen");
        m.insert("Allow users to share with anyone", "Baimendu erabiltzaileak edonorekin elkarbanatzen");
        m.insert("Allow users to only share with users in their groups", "Baimendu erabiltzaileak bakarrik bere taldeko erabiltzaileekin elkarbanatzen");
        m.insert("Security", "Segurtasuna");
        m.insert("Enforce HTTPS", "Behartu HTTPS");
        m.insert("Forces the clients to connect to %s via an encrypted connection.", "Bezeroak %s-ra konexio enkriptatu baten bidez konektatzera behartzen ditu.");
        m.insert("Please connect to your %s via HTTPS to enable or disable the SSL enforcement.", "Mesedez konektatu zure %s-ra HTTPS bidez SSL zehaztapenak aldatzeko.");
        m.insert("Log", "Egunkaria");
        m.insert("Log level", "Erregistro maila");
        m.insert("More", "Gehiago");
        m.insert("Less", "Gutxiago");
        m.insert("Version", "Bertsioa");
        m.insert("Developed by the <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud community</a>, the <a href=\"https://github.com/owncloud\" target=\"_blank\">source code</a> is licensed under the <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.", "<a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud komunitateak</a> garatuta, <a href=\"https://github.com/owncloud\" target=\"_blank\">itubruru kodea</a><a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr> lizentziarekin banatzen da</a>.");
        m.insert("Add your App", "Gehitu zure aplikazioa");
        m.insert("More Apps", "App gehiago");
        m.insert("Select an App", "Aukeratu programa bat");
        m.insert("See application page at apps.owncloud.com", "Ikusi programen orria apps.owncloud.com en");
        m.insert("<span class=\"licence\"></span>-licensed by <span class=\"author\"></span>", "<span class=\"licence\"></span>-lizentziatua <span class=\"author\"></span>");
        m.insert("User Documentation", "Erabiltzaile dokumentazioa");
        m.insert("Administrator Documentation", "Administradore dokumentazioa");
        m.insert("Online Documentation", "Online dokumentazioa");
        m.insert("Forum", "Foroa");
        m.insert("Bugtracker", "Bugtracker");
        m.insert("Commercial Support", "Babes komertziala");
        m.insert("Get the apps to sync your files", "Lortu aplikazioak zure fitxategiak sinkronizatzeko");
        m.insert("Show First Run Wizard again", "Erakutsi berriz Lehenengo Aldiko Morroia");
        m.insert("You have used <strong>%s</strong> of the available <strong>%s</strong>", "Dagoeneko <strong>%s</strong> erabili duzu eskuragarri duzun <strong>%s</strong>etatik");
        m.insert("Password", "Pasahitza");
        m.insert("Your password was changed", "Zere pasahitza aldatu da");
        m.insert("Unable to change your password", "Ezin izan da zure pasahitza aldatu");
        m.insert("Current password", "Uneko pasahitza");
        m.insert("New password", "Pasahitz berria");
        m.insert("Change password", "Aldatu pasahitza");
        m.insert("Email", "E-posta");
        m.insert("Your email address", "Zure e-posta");
        m.insert("Fill in an email address to enable password recovery", "Idatz ezazu e-posta bat pasahitza berreskuratu ahal izateko");
        m.insert("Profile picture", "Profilaren irudia");
        m.insert("Upload new", "Igo berria");
        m.insert("Remove image", "Irudia ezabatu");
        m.insert("Abort", "Bertan-behera utzi");
        m.insert("Choose as profile image", "Profil irudi bezala aukeratu");
        m.insert("Language", "Hizkuntza");
        m.insert("Help translate", "Lagundu itzultzen");
        m.insert("WebDAV", "WebDAV");
        m.insert("Encryption", "Enkriptazioa");
        m.insert("Login Name", "Sarrera Izena");
        m.insert("Create", "Sortu");
        m.insert("Admin Recovery Password", "Kudeatzaile pasahitz berreskuratzea");
        m.insert("Enter the recovery password in order to recover the users files during password change", "berreskuratze pasahitza idatzi pasahitz aldaketan erabiltzaileen fitxategiak berreskuratzeko");
        m.insert("Default Storage", "Lehenetsitako Biltegiratzea");
        m.insert("Unlimited", "Mugarik gabe");
        m.insert("Other", "Bestelakoa");
        m.insert("Username", "Erabiltzaile izena");
        m.insert("Storage", "Biltegiratzea");
        m.insert("set new password", "ezarri pasahitz berria");
        m.insert("Default", "Lehenetsia");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_forms() -> &'static str {
    *PLURAL_FORMS
}