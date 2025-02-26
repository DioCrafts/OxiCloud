use rust_i18n::t;

pub fn register_translations() {
    rust_i18n::set_locale("cs_CZ");

    rust_i18n::translations! {
        cs_CZ => {
            "Failed to clear the mappings." => "Selhalo zrušení mapování.",
            "Failed to delete the server configuration" => "Selhalo smazání nastavení serveru",
            "The configuration is valid and the connection could be established!" => "Nastavení je v pořádku a spojení bylo navázáno.",
            "The configuration is valid, but the Bind failed. Please check the server settings and credentials." => "Konfigurace je v pořádku, ale spojení selhalo. Zkontrolujte, prosím, nastavení serveru a přihlašovací údaje.",
            "Deletion failed" => "Mazání selhalo",
            "Take over settings from recent server configuration?" => "Převzít nastavení z nedávné konfigurace serveru?",
            "Keep settings?" => "Ponechat nastavení?",
            "Cannot add server configuration" => "Nelze přidat nastavení serveru",
            "mappings cleared" => "mapování zrušeno",
            "Success" => "Úspěch",
            "Error" => "Chyba",
            "Select groups" => "Vyberte skupiny",
            "Connection test succeeded" => "Test spojení byl úspěšný",
            "Connection test failed" => "Test spojení selhal",
            "Do you really want to delete the current Server Configuration?" => "Opravdu si přejete smazat současné nastavení serveru?",
            "Confirm Deletion" => "Potvrdit smazání",
            "_%s group found_::_%s groups found_" => { "0" => "%s group found", "1" => "%s groups found", "2" => "%s groups found" },
            "_%s user found_::_%s users found_" => { "0" => "%s user found", "1" => "%s users found", "2" => "%s users found" },
            "Save" => "Uložit",
            "Test Configuration" => "Vyzkoušet nastavení",
            "Help" => "Nápověda",
            "Add Server Configuration" => "Přidat nastavení serveru",
            "Host" => "Počítač",
            "You can omit the protocol, except you require SSL. Then start with ldaps://" => "Můžete vynechat protokol, vyjma pokud požadujete SSL. Tehdy začněte s ldaps://",
            "Port" => "Port",
            "User DN" => "Uživatelské DN",
            "The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty." => "DN klientského uživatele, ke kterému tvoříte vazbu, např. uid=agent,dc=example,dc=com. Pro anonymní přístup ponechte DN a heslo prázdné.",
            "Password" => "Heslo",
            "For anonymous access, leave DN and Password empty." => "Pro anonymní přístup ponechte údaje DN and heslo prázdné.",
            "One Base DN per line" => "Jedna základní DN na řádku",
            "You can specify Base DN for users and groups in the Advanced tab" => "V rozšířeném nastavení můžete určit základní DN pro uživatele a skupiny",
            "Back" => "Zpět",
            "Continue" => "Pokračovat",
            "<b>Warning:</b> Apps user_ldap and user_webdavauth are incompatible. You may experience unexpected behavior. Please ask your system administrator to disable one of them." => "<b>Varování:</b> Aplikace user_ldap a user_webdavauth jsou vzájemně nekompatibilní. Můžete zaznamenat neočekávané chování. Požádejte prosím vašeho systémového administrátora o zakázání jednoho z nich.",
            "<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it." => "<b>Varování:</b> není nainstalován LDAP modul pro PHP, podpůrná vrstva nebude fungovat. Požádejte, prosím, správce systému, aby jej nainstaloval.",
            "Connection Settings" => "Nastavení spojení",
            "Configuration Active" => "Nastavení aktivní",
            "When unchecked, this configuration will be skipped." => "Pokud není zaškrtnuto, bude toto nastavení přeskočeno.",
            "User Login Filter" => "Filtr přihlášení uživatelů",
            "Defines the filter to apply, when login is attempted. %%uid replaces the username in the login action. Example: \"uid=%%uid\"" => "Určuje použitý filtr při pokusu o přihlášení. %%uid nahrazuje uživatelské jméno v činnosti přihlášení. Příklad: \"uid=%%uid\"",
            "Backup (Replica) Host" => "Záložní (kopie) hostitel",
            "Give an optional backup host. It must be a replica of the main LDAP/AD server." => "Zadejte volitelného záložního hostitele. Musí to být kopie hlavního serveru LDAP/AD.",
            "Backup (Replica) Port" => "Záložní (kopie) port",
            "Disable Main Server" => "Zakázat hlavní server",
            "Only connect to the replica server." => "Připojit jen k záložnímu serveru.",
            "Case insensitve LDAP server (Windows)" => "LDAP server nerozlišující velikost znaků (Windows)",
            "Turn off SSL certificate validation." => "Vypnout ověřování SSL certifikátu.",
            "Not recommended, use it for testing only! If connection only works with this option, import the LDAP server's SSL certificate in your %s server." => "Nedoporučuje se, určeno pouze k testovacímu použití. Pokud spojení funguje jen s touto volbou, importujte SSL certifikát vašeho LDAP serveru na server %s.",
            "Cache Time-To-Live" => "TTL vyrovnávací paměti",
            "in seconds. A change empties the cache." => "v sekundách. Změna vyprázdní vyrovnávací paměť.",
            "Directory Settings" => "Nastavení adresáře",
            "User Display Name Field" => "Pole zobrazovaného jména uživatele",
            "The LDAP attribute to use to generate the user's display name." => "LDAP atribut použitý k vytvoření zobrazovaného jména uživatele.",
            "Base User Tree" => "Základní uživatelský strom",
            "One User Base DN per line" => "Jedna uživatelská základní DN na řádku",
            "User Search Attributes" => "Atributy vyhledávání uživatelů",
            "Optional; one attribute per line" => "Volitelné, jeden atribut na řádku",
            "Group Display Name Field" => "Pole zobrazovaného jména skupiny",
            "The LDAP attribute to use to generate the groups's display name." => "LDAP atribut použitý k vytvoření zobrazovaného jména skupiny.",
            "Base Group Tree" => "Základní skupinový strom",
            "One Group Base DN per line" => "Jedna skupinová základní DN na řádku",
            "Group Search Attributes" => "Atributy vyhledávání skupin",
            "Group-Member association" => "Asociace člena skupiny",
            "Special Attributes" => "Speciální atributy",
            "Quota Field" => "Pole pro kvótu",
            "Quota Default" => "Výchozí kvóta",
            "in bytes" => "v bajtech",
            "Email Field" => "Pole e-mailu",
            "User Home Folder Naming Rule" => "Pravidlo pojmenování domovské složky uživatele",
            "Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute." => "Ponechte prázdné pro uživatelské jméno (výchozí). Jinak uveďte LDAP/AD parametr.",
            "Internal Username" => "Interní uživatelské jméno",
            "By default the internal username will be created from the UUID attribute. It makes sure that the username is unique and characters do not need to be converted. The internal username has the restriction that only these characters are allowed: [ a-zA-Z0-9_.@- ].  Other characters are replaced with their ASCII correspondence or simply omitted. On collisions a number will be added/increased. The internal username is used to identify a user internally. It is also the default name for the user home folder. It is also a part of remote URLs, for instance for all *DAV services. With this setting, the default behavior can be overridden. To achieve a similar behavior as before ownCloud 5 enter the user display name attribute in the following field. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users." => "Ve výchozím nastavení bude uživatelské jméno vytvořeno z UUID atributu. To zajistí unikátnost uživatelského jména a není potřeba provádět konverzi znaků. Interní uživatelské jméno je omezeno na znaky: [ a-zA-Z0-9_.@- ]. Ostatní znaky jsou nahrazeny jejich ASCII ekvivalentem nebo jednoduše vynechány. V případě kolize uživatelských jmen bude přidáno/navýšeno číslo. Interní uživatelské jméno je používáno k interní identifikaci uživatele. Je také výchozím názvem uživatelského domovského adresáře. Je také součástí URL pro vzdálený přístup, například všech *DAV služeb. S tímto nastavením může být výchozí chování změněno. Pro dosažení podobného chování jako před ownCloudem 5 uveďte atribut zobrazovaného jména do pole níže. Ponechte prázdné pro výchozí chování. Změna bude mít vliv jen na nově namapované (přidané) uživatele z LDAP.",
            "Internal Username Attribute:" => "Atribut interního uživatelského jména:",
            "Override UUID detection" => "Nastavit ručně UUID atribut",
            "By default, the UUID attribute is automatically detected. The UUID attribute is used to doubtlessly identify LDAP users and groups. Also, the internal username will be created based on the UUID, if not specified otherwise above. You can override the setting and pass an attribute of your choice. You must make sure that the attribute of your choice can be fetched for both users and groups and it is unique. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users and groups." => "Ve výchozím nastavení je UUID atribut nalezen automaticky. UUID atribut je používán pro nezpochybnitelnou identifikaci uživatelů a skupin z LDAP. Navíc je na základě UUID tvořeno také interní uživatelské jméno, pokud není nastaveno jinak. Můžete výchozí nastavení přepsat a použít atribut, který sami zvolíte. Musíte se ale ujistit, že atribut, který vyberete, bude uveden jak u uživatelů, tak i u skupin a je unikátní. Ponechte prázdné pro výchozí chování. Změna bude mít vliv jen na nově namapované (přidané) uživatele a skupiny z LDAP.",
            "UUID Attribute for Users:" => "UUID atribut pro uživatele:",
            "UUID Attribute for Groups:" => "UUID atribut pro skupiny:",
            "Username-LDAP User Mapping" => "Mapování uživatelských jmen z LDAPu",
            "Usernames are used to store and assign (meta) data. In order to precisely identify and recognize users, each LDAP user will have a internal username. This requires a mapping from username to LDAP user. The created username is mapped to the UUID of the LDAP user. Additionally the DN is cached as well to reduce LDAP interaction, but it is not used for identification. If the DN changes, the changes will be found. The internal username is used all over. Clearing the mappings will have leftovers everywhere. Clearing the mappings is not configuration sensitive, it affects all LDAP configurations! Never clear the mappings in a production environment, only in a testing or experimental stage." => "Uživatelská jména jsou používány pro uchovávání a přiřazování (meta)dat. Pro správnou identifikaci a rozpoznání uživatelů bude mít každý uživatel z LDAP interní uživatelské jméno. To vyžaduje mapování uživatelských jmen na uživatele LDAP. Vytvořené uživatelské jméno je mapováno na UUID uživatele v LDAP. Navíc je cachována DN pro zmenšení interakce s LDAP, ale není používána pro identifikaci. Pokud se DN změní, bude to správně rozpoznáno. Interní uživatelské jméno se používá celé. Vyčištění mapování zanechá zbytky všude. Vyčištění navíc není specifické konfiguraci, bude mít vliv na všechny LDAP konfigurace! Nikdy nečistěte mapování v produkčním prostředí, jen v testovací nebo experimentální fázi.",
            "Clear Username-LDAP User Mapping" => "Zrušit mapování uživatelských jmen LDAPu",
            "Clear Groupname-LDAP Group Mapping" => "Zrušit mapování názvů skupin LDAPu"
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation_loaded() {
        register_translations();
        assert_eq!(t!("Error"), "Chyba");
    }
}