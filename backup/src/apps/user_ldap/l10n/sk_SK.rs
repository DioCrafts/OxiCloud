use rust_i18n::i18n;

i18n!("sk_SK");

pub fn register_translations() {
    rust_i18n::set_locale("sk_SK");
    
    add_translation!("Failed to clear the mappings.", "Nepodarilo sa vymazať mapovania.");
    add_translation!("Failed to delete the server configuration", "Zlyhalo zmazanie nastavenia servera.");
    add_translation!("The configuration is valid and the connection could be established!", "Nastavenie je v poriadku a pripojenie je stabilné.");
    add_translation!("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "Nastavenie je v poriadku, ale pripojenie zlyhalo. Skontrolujte nastavenia servera a prihlasovacie údaje.");
    add_translation!("Deletion failed", "Odstránenie zlyhalo");
    add_translation!("Take over settings from recent server configuration?", "Prebrať nastavenia z nedávneho nastavenia servera?");
    add_translation!("Keep settings?", "Ponechať nastavenia?");
    add_translation!("Cannot add server configuration", "Nemožno pridať nastavenie servera");
    add_translation!("mappings cleared", "mapovanie vymazané");
    add_translation!("Success", "Úspešné");
    add_translation!("Error", "Chyba");
    add_translation!("Select groups", "Vybrať skupinu");
    add_translation!("Connection test succeeded", "Test pripojenia bol úspešný");
    add_translation!("Connection test failed", "Test pripojenia zlyhal");
    add_translation!("Do you really want to delete the current Server Configuration?", "Naozaj chcete zmazať súčasné nastavenie servera?");
    add_translation!("Confirm Deletion", "Potvrdiť vymazanie");
    add_plural_translation!("%s group found", "%s groups found", vec!["", "", ""]);
    add_plural_translation!("%s user found", "%s users found", vec!["", "", ""]);
    add_translation!("Save", "Uložiť");
    add_translation!("Test Configuration", "Test nastavenia");
    add_translation!("Help", "Pomoc");
    add_translation!("Add Server Configuration", "Pridať nastavenia servera.");
    add_translation!("Host", "Hostiteľ");
    add_translation!("You can omit the protocol, except you require SSL. Then start with ldaps://", "Môžete vynechať protokol, s výnimkou požadovania SSL. Vtedy začnite s ldaps://");
    add_translation!("Port", "Port");
    add_translation!("User DN", "Používateľské DN");
    add_translation!("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "DN klientského používateľa, ku ktorému tvoríte väzbu, napr. uid=agent,dc=example,dc=com. Pre anonymný prístup ponechajte údaje DN a Heslo prázdne.");
    add_translation!("Password", "Heslo");
    add_translation!("For anonymous access, leave DN and Password empty.", "Pre anonymný prístup ponechajte údaje DN a Heslo prázdne.");
    add_translation!("One Base DN per line", "Jedno základné DN na riadok");
    add_translation!("You can specify Base DN for users and groups in the Advanced tab", "V rozšírenom nastavení môžete zadať základné DN pre používateľov a skupiny");
    add_translation!("Back", "Späť");
    add_translation!("Continue", "Pokračovať");
    add_translation!("<b>Warning:</b> Apps user_ldap and user_webdavauth are incompatible. You may experience unexpected behavior. Please ask your system administrator to disable one of them.", "<b>Upozornenie:</b> Aplikácie user_ldap a user_webdavauth sú navzájom nekompatibilné. Môžete zaznamenať neočakávané správanie. Požiadajte prosím vášho systémového administrátora pre zakázanie jedného z nich.");
    add_translation!("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", "<b>Upozornenie:</b> nie je nainštalovaný LDAP modul pre PHP, backend vrstva nebude fungovať. Požádejte administrátora systému aby ho nainštaloval.");
    add_translation!("Connection Settings", "Nastavenie pripojenia");
    add_translation!("Configuration Active", "Nastavenia sú aktívne ");
    add_translation!("When unchecked, this configuration will be skipped.", "Ak nie je zaškrtnuté, nastavenie bude preskočené.");
    add_translation!("User Login Filter", "Filter prihlásenia používateľov");
    add_translation!("Defines the filter to apply, when login is attempted. %%uid replaces the username in the login action. Example: \"uid=%%uid\"", "Určuje použitý filter, pri pokuse o prihlásenie. %%uid nahradzuje používateľské meno v činnosti prihlásenia. Napríklad: \"uid=%%uid\"");
    add_translation!("Backup (Replica) Host", "Záložný server (kópia) hosť");
    add_translation!("Give an optional backup host. It must be a replica of the main LDAP/AD server.", "Zadajte záložný LDAP/AD. Musí to byť kópia hlavného LDAP/AD servera.");
    add_translation!("Backup (Replica) Port", "Záložný server (kópia) port");
    add_translation!("Disable Main Server", "Zakázať hlavný server");
    add_translation!("Only connect to the replica server.", "Pripojiť sa len k záložnému serveru.");
    add_translation!("Case insensitve LDAP server (Windows)", "LDAP server nerozlišuje veľkosť znakov (Windows)");
    add_translation!("Turn off SSL certificate validation.", "Vypnúť overovanie SSL certifikátu.");
    add_translation!("Not recommended, use it for testing only! If connection only works with this option, import the LDAP server's SSL certificate in your %s server.", "Neodporúčané, použite iba pri testovaní! Pokiaľ spojenie funguje iba z daným nastavením, importujte SSL certifikát LDAP servera do vášho %s servera.");
    add_translation!("Cache Time-To-Live", "Životnosť objektov v cache");
    add_translation!("in seconds. A change empties the cache.", "v sekundách. Zmena vyprázdni vyrovnávaciu pamäť.");
    add_translation!("Directory Settings", "Nastavenie priečinka");
    add_translation!("User Display Name Field", "Pole pre zobrazenia mena používateľa");
    add_translation!("The LDAP attribute to use to generate the user's display name.", "Atribút LDAP použitý na vygenerovanie zobrazovaného mena používateľa. ");
    add_translation!("Base User Tree", "Základný používateľský strom");
    add_translation!("One User Base DN per line", "Jedna používateľská základná DN na riadok");
    add_translation!("User Search Attributes", "Atribúty vyhľadávania používateľov");
    add_translation!("Optional; one attribute per line", "Voliteľné, jeden atribút na jeden riadok");
    add_translation!("Group Display Name Field", "Pole pre zobrazenie mena skupiny");
    add_translation!("The LDAP attribute to use to generate the groups's display name.", "Atribút LDAP použitý na vygenerovanie zobrazovaného mena skupiny.");
    add_translation!("Base Group Tree", "Základný skupinový strom");
    add_translation!("One Group Base DN per line", "Jedna skupinová základná DN na riadok");
    add_translation!("Group Search Attributes", "Atribúty vyhľadávania skupín");
    add_translation!("Group-Member association", "Priradenie člena skupiny");
    add_translation!("Special Attributes", "Špeciálne atribúty");
    add_translation!("Quota Field", "Pole kvóty");
    add_translation!("Quota Default", "Predvolená kvóta");
    add_translation!("in bytes", "v bajtoch");
    add_translation!("Email Field", "Pole email");
    add_translation!("User Home Folder Naming Rule", "Pravidlo pre nastavenie mena používateľského priečinka dát");
    add_translation!("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "Nechajte prázdne pre používateľské meno (predvolené). Inak uveďte atribút LDAP/AD.");
    add_translation!("Internal Username", "Interné používateľské meno");
    add_translation!("By default the internal username will be created from the UUID attribute. It makes sure that the username is unique and characters do not need to be converted. The internal username has the restriction that only these characters are allowed: [ a-zA-Z0-9_.@- ].  Other characters are replaced with their ASCII correspondence or simply omitted. On collisions a number will be added/increased. The internal username is used to identify a user internally. It is also the default name for the user home folder. It is also a part of remote URLs, for instance for all *DAV services. With this setting, the default behavior can be overridden. To achieve a similar behavior as before ownCloud 5 enter the user display name attribute in the following field. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users.", "V predvolenom nastavení bude interné používateľské meno vytvorené z UUID atribútu. Zabezpečí sa to, že používateľské meno bude jedinečné a znaky nemusia byť prevedené. Interné meno má obmedzenie, iba tieto znaky sú povolené: [a-zA-Z0-9_ @ -.]. Ostatné znaky sú nahradené ich ASCII alebo jednoducho vynechané. Pri kolíziách používateľských mien bude číslo pridané / odobrané. Interné používateľské meno sa používa na internú identifikáciu používateľa. Je tiež predvoleným názvom používateľského domovského priečinka v ownCloud. Je tiež súčasťou URL pre vzdialený prístup, napríklad pre všetky služby * DAV. S týmto nastavením sa dá prepísať predvolené správanie. Pre dosiahnutie podobného správania sa ako pred verziou ownCloud 5 zadajte atribút zobrazenia používateľského mena v tomto poli. Ponechajte prázdne pre predvolené správanie. Zmeny budú mať vplyv iba na novo namapovaných (pridaných) LDAP používateľov.");
    add_translation!("Internal Username Attribute:", "Atribút interného používateľského mena:");
    add_translation!("Override UUID detection", "Prepísať UUID detekciu");
    add_translation!("By default, the UUID attribute is automatically detected. The UUID attribute is used to doubtlessly identify LDAP users and groups. Also, the internal username will be created based on the UUID, if not specified otherwise above. You can override the setting and pass an attribute of your choice. You must make sure that the attribute of your choice can be fetched for both users and groups and it is unique. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users and groups.", "V predvolenom nastavení je UUID atribút detekovaný automaticky. UUID atribút je použitý na jedinečnú identifikáciu používateľov a skupín z LDAP. Naviac je na základe UUID vytvorené tiež interné použivateľské meno, ak nie je nastavené inak. Môžete predvolené nastavenie prepísať a použiť atribút ktorý si sami zvolíte. Musíte sa ale ubezpečiť, že atribút ktorý vyberiete bude uvedený pri použivateľoch, aj pri skupinách a je jedinečný. Ponechajte prázdne pre predvolené správanie. Zmena bude mať vplyv len na novo namapovaných (pridaných) používateľov a skupiny z LDAP.");
    add_translation!("UUID Attribute for Users:", "UUID atribút pre používateľov:");
    add_translation!("UUID Attribute for Groups:", "UUID atribút pre skupiny:");
    add_translation!("Username-LDAP User Mapping", "Mapovanie názvov LDAP používateľských mien");
    add_translation!("Usernames are used to store and assign (meta) data. In order to precisely identify and recognize users, each LDAP user will have a internal username. This requires a mapping from username to LDAP user. The created username is mapped to the UUID of the LDAP user. Additionally the DN is cached as well to reduce LDAP interaction, but it is not used for identification. If the DN changes, the changes will be found. The internal username is used all over. Clearing the mappings will have leftovers everywhere. Clearing the mappings is not configuration sensitive, it affects all LDAP configurations! Never clear the mappings in a production environment, only in a testing or experimental stage.", "Použivateľské mená sa používajú pre uchovávanie a priraďovanie (meta)dát. Pre správnu identifikáciu a rozpoznanie používateľov bude mať každý používateľ z LDAP interné používateľské meno. To je nevyhnutné pre namapovanie používateľských mien na používateľov v LDAP. Vytvorené používateľské meno je namapované na UUID používateľa v LDAP. Naviac je cachovaná DN pre obmedzenie interakcie s LDAP, ale nie je používaná pre identifikáciu. Ak sa DN zmení, bude to správne rozpoznané. Interné používateľské meno sa používa všade. Vyčistenie namapování vymaže zvyšky všade. Vyčistenie naviac nie je špecifické, bude mať vplyv na všetky LDAP konfigurácie! Nikdy nečistite namapovanie v produkčnom prostredí, len v testovacej alebo experimentálnej fáze.");
    add_translation!("Clear Username-LDAP User Mapping", "Zrušiť mapovanie LDAP používateľských mien");
    add_translation!("Clear Groupname-LDAP Group Mapping", "Zrušiť mapovanie názvov LDAP skupín");

    rust_i18n::set_plural_rule("sk_SK", |n| {
        if n == 1 {
            0
        } else if n >= 2 && n <= 4 {
            1
        } else {
            2
        }
    });
}

#[macro_export]
macro_rules! add_translation {
    ($key:expr, $value:expr) => {
        rust_i18n::add_translation("sk_SK", $key, $value);
    };
}

#[macro_export]
macro_rules! add_plural_translation {
    ($singular:expr, $plural:expr, $forms:expr) => {
        rust_i18n::add_translation_plural("sk_SK", $singular, $plural, $forms);
    };
}