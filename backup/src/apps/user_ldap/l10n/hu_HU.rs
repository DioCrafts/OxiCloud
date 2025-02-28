use std::collections::HashMap;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Failed to clear the mappings.", "Nem sikerült törölni a hozzárendeléseket.");
    translations.insert("Failed to delete the server configuration", "Nem sikerült törölni a kiszolgáló konfigurációját");
    translations.insert("The configuration is valid and the connection could be established!", "A konfiguráció érvényes, és a kapcsolat létrehozható!");
    translations.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "A konfiguráció érvényes, de a kapcsolat nem hozható létre. Kérem ellenőrizze a kiszolgáló beállításait, és az elérési adatokat.");
    translations.insert("No action specified", "Nincs megadva parancs");
    translations.insert("No configuration specified", "Nincs megadva konfiguráció");
    translations.insert("No data specified", "Nincs adat megadva");
    translations.insert(" Could not set configuration %s", "A(z) %s konfiguráció nem állítható be");
    translations.insert("Deletion failed", "A törlés nem sikerült");
    translations.insert("Take over settings from recent server configuration?", "Vegyük át a beállításokat az előző konfigurációból?");
    translations.insert("Keep settings?", "Tartsuk meg a beállításokat?");
    translations.insert("Cannot add server configuration", "Az új  kiszolgáló konfigurációja nem hozható létre");
    translations.insert("mappings cleared", "Töröltük a hozzárendeléseket");
    translations.insert("Success", "Sikeres végrehajtás");
    translations.insert("Error", "Hiba");
    translations.insert("Select groups", "Csoportok kiválasztása");
    translations.insert("Select object classes", "Objektumosztályok kiválasztása");
    translations.insert("Select attributes", "Attribútumok kiválasztása");
    translations.insert("Connection test succeeded", "A kapcsolatellenőrzés eredménye: sikerült");
    translations.insert("Connection test failed", "A kapcsolatellenőrzés eredménye: nem sikerült");
    translations.insert("Do you really want to delete the current Server Configuration?", "Tényleg törölni szeretné a kiszolgáló beállításait?");
    translations.insert("Confirm Deletion", "A törlés megerősítése");
    translations.insert("Invalid Host", "Érvénytelen gépnév");
    translations.insert("Could not find the desired feature", "A kívánt funkció nem található");
    translations.insert("Save", "Mentés");
    translations.insert("Test Configuration", "A beállítások tesztelése");
    translations.insert("Help", "Súgó");
    translations.insert("Limit the access to %s to groups meeting this criteria:", "Korlátozzuk %s elérését a következő feltételeknek megfelelő csoportokra:");
    translations.insert("only those object classes:", "csak ezek az objektumosztályok:");
    translations.insert("only from those groups:", "csak ezek a csoportok:");
    translations.insert("Edit raw filter instead", "Inkább közvetlenül megadom a szűrési kifejezést:");
    translations.insert("Raw LDAP filter", "Az LDAP szűrőkifejezés");
    translations.insert("The filter specifies which LDAP groups shall have access to the %s instance.", "A szűrő meghatározza, hogy mely LDAP csoportok lesznek jogosultak %s elérésére.");
    translations.insert("groups found", "csoport van");
    translations.insert("What attribute shall be used as login name:", "Melyik attribútumot használjuk login névként:");
    translations.insert("LDAP Username:", "LDAP felhasználónév:");
    translations.insert("LDAP Email Address:", "LDAP e-mail cím:");
    translations.insert("Other Attributes:", "Más attribútumok:");
    translations.insert("Add Server Configuration", "Új kiszolgáló beállításának hozzáadása");
    translations.insert("Host", "Kiszolgáló");
    translations.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "A protokoll előtag elhagyható, kivéve, ha SSL-t kíván használni. Ebben az esetben kezdje így:  ldaps://");
    translations.insert("Port", "Port");
    translations.insert("User DN", "A kapcsolódó felhasználó DN-je");
    translations.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "Annak a felhasználónak a DN-je, akinek a nevében bejelentkezve kapcsolódunk a kiszolgálóhoz, pl. uid=agent,dc=example,dc=com. Bejelentkezés nélküli eléréshez ne töltse ki a DN és Jelszó mezőket!");
    translations.insert("Password", "Jelszó");
    translations.insert("For anonymous access, leave DN and Password empty.", "Bejelentkezés nélküli eléréshez ne töltse ki a DN és Jelszó mezőket!");
    translations.insert("One Base DN per line", "Soronként egy DN-gyökér");
    translations.insert("You can specify Base DN for users and groups in the Advanced tab", "A Haladó fülre kattintva külön DN-gyökér állítható be a felhasználók és a csoportok számára");
    translations.insert("Limit the access to %s to users meeting this criteria:", "Korlátozzuk %s elérését a következő feltételeknek megfelelő felhasználókra:");
    translations.insert("The filter specifies which LDAP users shall have access to the %s instance.", "A szűrő meghatározza, hogy mely LDAP felhasználók lesznek jogosultak %s elérésére.");
    translations.insert("users found", "felhasználó van");
    translations.insert("Back", "Vissza");
    translations.insert("Continue", "Folytatás");
    translations.insert("<b>Warning:</b> Apps user_ldap and user_webdavauth are incompatible. You may experience unexpected behavior. Please ask your system administrator to disable one of them.", "<b>Figyelem:</b> a user_ldap és user_webdavauth alkalmazások nem kompatibilisek. Együttes használatuk váratlan eredményekhez vezethet. Kérje meg a rendszergazdát, hogy a kettő közül kapcsolja ki az egyiket.");
    translations.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", "<b>Figyelmeztetés:</b> Az LDAP PHP modul nincs telepítve, ezért ez az alrendszer nem fog működni. Kérje meg a rendszergazdát, hogy telepítse!");
    translations.insert("Connection Settings", "Kapcsolati beállítások");
    translations.insert("Configuration Active", "A beállítás aktív");
    translations.insert("When unchecked, this configuration will be skipped.", "Ha nincs kipipálva, ez a beállítás kihagyódik.");
    translations.insert("User Login Filter", "Szűrő a bejelentkezéshez");
    translations.insert("Defines the filter to apply, when login is attempted. %%uid replaces the username in the login action. Example: \"uid=%%uid\"", "Ez a szűrő érvényes a bejelentkezés megkísérlésekor. Ekkor az %%uid változó helyére a bejelentkezési név kerül. Például: \"uid=%%uid\"");
    translations.insert("Backup (Replica) Host", "Másodkiszolgáló (replika)");
    translations.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.", "Adjon meg egy opcionális másodkiszolgálót. Ez a fő LDAP/AD kiszolgáló szinkron másolata (replikája) kell legyen.");
    translations.insert("Backup (Replica) Port", "A másodkiszolgáló (replika) portszáma");
    translations.insert("Disable Main Server", "A fő szerver kihagyása");
    translations.insert("Only connect to the replica server.", "Csak a másodlagos (másolati) kiszolgálóhoz kapcsolódjunk.");
    translations.insert("Case insensitve LDAP server (Windows)", "Az LDAP-kiszolgáló nem tesz különbséget a kis- és nagybetűk között (Windows)");
    translations.insert("Turn off SSL certificate validation.", "Ne ellenőrizzük az SSL-tanúsítvány érvényességét");
    translations.insert("Not recommended, use it for testing only! If connection only works with this option, import the LDAP server's SSL certificate in your %s server.", "Használata nem javasolt (kivéve tesztelési céllal). Ha a kapcsolat csak ezzel a beállítással működik, akkor importálja az LDAP-kiszolgáló SSL tanúsítványát a(z) %s kiszolgálóra!");
    translations.insert("Cache Time-To-Live", "A gyorsítótár tárolási időtartama");
    translations.insert("in seconds. A change empties the cache.", "másodpercben. A változtatás törli a cache tartalmát.");
    translations.insert("Directory Settings", "Címtár beállítások");
    translations.insert("User Display Name Field", "A felhasználónév mezője");
    translations.insert("The LDAP attribute to use to generate the user's display name.", "Ebből az LDAP attribútumból képződik a felhasználó megjelenítendő neve.");
    translations.insert("Base User Tree", "A felhasználói fa gyökere");
    translations.insert("One User Base DN per line", "Soronként egy felhasználói fa gyökerét adhatjuk meg");
    translations.insert("User Search Attributes", "A felhasználók lekérdezett attribútumai");
    translations.insert("Optional; one attribute per line", "Nem kötelező megadni, soronként egy attribútum");
    translations.insert("Group Display Name Field", "A csoport nevének mezője");
    translations.insert("The LDAP attribute to use to generate the groups's display name.", "Ebből az LDAP attribútumból képződik a csoport megjelenítendő neve.");
    translations.insert("Base Group Tree", "A csoportfa gyökere");
    translations.insert("One Group Base DN per line", "Soronként egy csoportfa gyökerét adhatjuk meg");
    translations.insert("Group Search Attributes", "A csoportok lekérdezett attribútumai");
    translations.insert("Group-Member association", "A csoporttagság attribútuma");
    translations.insert("Special Attributes", "Különleges attribútumok");
    translations.insert("Quota Field", "Kvóta mező");
    translations.insert("Quota Default", "Alapértelmezett kvóta");
    translations.insert("in bytes", "bájtban");
    translations.insert("Email Field", "Email mező");
    translations.insert("User Home Folder Naming Rule", "A home könyvtár elérési útvonala");
    translations.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "Hagyja üresen, ha a felhasználónevet kívánja használni. Ellenkező esetben adjon meg egy LDAP/AD attribútumot!");
    translations.insert("Internal Username", "Belső felhasználónév");
    translations.insert("By default the internal username will be created from the UUID attribute. It makes sure that the username is unique and characters do not need to be converted. The internal username has the restriction that only these characters are allowed: [ a-zA-Z0-9_.@- ].  Other characters are replaced with their ASCII correspondence or simply omitted. On collisions a number will be added/increased. The internal username is used to identify a user internally. It is also the default name for the user home folder. It is also a part of remote URLs, for instance for all *DAV services. With this setting, the default behavior can be overridden. To achieve a similar behavior as before ownCloud 5 enter the user display name attribute in the following field. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users.", "Alapértelmezetten a belső felhasználónév az UUID tulajdonságból jön létre. Ez biztosítja a felhasználónév egyediségét és hogy a nem kell konvertálni a karaktereket benne. A belső felhasználónévnél a megkötés az, hogy csak a következő karakterek engdélyezettek benne: [ a-zA-Z0-9_.@- ]. Ezeken a karaktereken kivül minden karakter le lesz cserélve az adott karakter ASCII kódtáblában használható párjára vagy ha ilyen nincs akkor egyszerűen ki lesz hagyva. Ha így mégis ütköznének a nevek akkor hozzá lesz füzve egy folyamatosan növekvő számláló rész. A  belső felhasználónevet lehet használni a felhasználó azonosítására a programon belül. Illetve ez lesz az alapáértelmezett neve a felhasználó kezdő könyvtárának az ownCloud-ban. Illetve...............................");
    translations.insert("Internal Username Attribute:", "A belső felhasználónév attribútuma:");
    translations.insert("Override UUID detection", "Az UUID-felismerés felülbírálása");
    translations.insert("By default, the UUID attribute is automatically detected. The UUID attribute is used to doubtlessly identify LDAP users and groups. Also, the internal username will be created based on the UUID, if not specified otherwise above. You can override the setting and pass an attribute of your choice. You must make sure that the attribute of your choice can be fetched for both users and groups and it is unique. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users and groups.", "Az UUID attribútum alapértelmezetten felismerésre kerül. Az UUID attribútum segítségével az LDAP felhasználók és csoportok egyértelműen azonosíthatók. A belső felhasználónév is azonos lesz az UUID-vel, ha fentebb nincs másként definiálva. Ezt a beállítást felülbírálhatja és bármely attribútummal helyettesítheti. Ekkor azonban gondoskodnia kell arról, hogy a kiválasztott attribútum minden felhasználó és csoport esetén lekérdezhető és egyedi értékkel bír. Ha a mezőt üresen hagyja, akkor az alapértelmezett attribútum lesz érvényes. Egy esetleges módosítás csak az újonnan hozzárendelt (ill. létrehozott) felhasználókra és csoportokra lesz érvényes.");
    translations.insert("UUID Attribute for Users:", "A felhasználók UUID attribútuma:");
    translations.insert("UUID Attribute for Groups:", "A csoportok UUID attribútuma:");
    translations.insert("Username-LDAP User Mapping", "Felhasználó - LDAP felhasználó hozzárendelés");
    translations.insert("Usernames are used to store and assign (meta) data. In order to precisely identify and recognize users, each LDAP user will have a internal username. This requires a mapping from username to LDAP user. The created username is mapped to the UUID of the LDAP user. Additionally the DN is cached as well to reduce LDAP interaction, but it is not used for identification. If the DN changes, the changes will be found. The internal username is used all over. Clearing the mappings will have leftovers everywhere. Clearing the mappings is not configuration sensitive, it affects all LDAP configurations! Never clear the mappings in a production environment, only in a testing or experimental stage.", "A felhasználónevek segítségével történik a (meta)adatok tárolása és hozzárendelése. A felhasználók pontos azonosítása céljából minden LDAP felhasználóhoz egy belső felhasználónevet rendelünk. Ezt a felhasználónevet az LDAP felhasználó UUID attribútumához rendeljük hozzá. Ezen túlmenően a DN is tárolásra kerül a gyorsítótárban, hogy csökkentsük az LDAP lekérdezések számát, de a DN-t nem használjuk azonosításra. Ha a DN megváltozik, akkor a rendszer ezt észleli. A belső felhasználóneveket a rendszer igen sok helyen használja, ezért a hozzárendelések törlése sok érvénytelen adatrekordot eredményez az adatbázisban. A hozzárendelések törlése nem függ a konfigurációtól, minden LDAP konfigurációt érint! Ténylegesen működő szolgáltatás esetén sose törölje a hozzárendeléseket, csak tesztelési vagy kísérleti célú szerveren!");
    translations.insert("Clear Username-LDAP User Mapping", "A felhasználó - LDAP felhasználó hozzárendelés törlése");
    translations.insert("Clear Groupname-LDAP Group Mapping", "A csoport - LDAP csoport hozzárendelés törlése");
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn get_plural_translations() -> HashMap<&'static str, Vec<&'static str>> {
    let mut plural_translations = HashMap::new();
    plural_translations.insert("_%s group found_::_%s groups found_", vec!["%s csoport van", "%s csoport van"]);
    plural_translations.insert("_%s user found_::_%s users found_", vec!["%s felhasználó van", "%s felhasználó van"]);
    
    plural_translations
}