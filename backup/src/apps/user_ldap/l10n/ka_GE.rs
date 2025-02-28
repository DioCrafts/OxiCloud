use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Failed to delete the server configuration", "შეცდომა სერვერის კონფიგურაციის წაშლისას");
        m.insert("The configuration is valid and the connection could be established!", "კონფიგურაცია მართებულია და კავშირი დამყარდება!");
        m.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "კონფიგურაცია მართებულია, მაგრამ მიერთება ვერ მოხერხდა. გთხოვთ შეამოწმოთ სერვერის პარამეტრები და აუთენთიკაციის პარამეტრები.");
        m.insert("Deletion failed", "წაშლა ვერ განხორციელდა");
        m.insert("Take over settings from recent server configuration?", "დაბრუნდებით სერვერის წინა კონფიგურაციაში?");
        m.insert("Keep settings?", "დავტოვოთ პარამეტრები?");
        m.insert("Cannot add server configuration", "სერვერის პარამეტრების დამატება ვერ მოხერხდა");
        m.insert("Success", "დასრულდა");
        m.insert("Error", "შეცდომა");
        m.insert("Select groups", "ჯგუფების არჩევა");
        m.insert("Connection test succeeded", "კავშირის ტესტირება მოხერხდა");
        m.insert("Connection test failed", "კავშირის ტესტირება ვერ მოხერხდა");
        m.insert("Do you really want to delete the current Server Configuration?", "ნამდვილად გინდათ წაშალოთ სერვერის მიმდინარე პარამეტრები?");
        m.insert("Confirm Deletion", "წაშლის დადასტურება");
        m.insert("Save", "შენახვა");
        m.insert("Test Configuration", "კავშირის ტესტირება");
        m.insert("Help", "დახმარება");
        m.insert("Add Server Configuration", "სერვერის პარამეტრების დამატება");
        m.insert("Host", "ჰოსტი");
        m.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "თქვენ შეგიძლიათ გამოტოვოთ პროტოკოლი. გარდა ამისა გჭირდებათ SSL. შემდეგ დაიწყეთ ldaps://");
        m.insert("Port", "პორტი");
        m.insert("User DN", "მომხმარებლის DN");
        m.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "მომხმარებლის DN რომელთანაც უნდა მოხდეს დაკავშირება მოხდება შემდეგნაირად მაგ: uid=agent,dc=example,dc=com. ხოლო ანონიმური დაშვებისთვის, დატოვეთ DN–ის და პაროლის ველები ცარიელი.");
        m.insert("Password", "პაროლი");
        m.insert("For anonymous access, leave DN and Password empty.", "ანონიმური დაშვებისთვის, დატოვეთ DN–ის და პაროლის ველები ცარიელი.");
        m.insert("One Base DN per line", "ერთი საწყისი DN ერთ ხაზზე");
        m.insert("You can specify Base DN for users and groups in the Advanced tab", "თქვენ შეგიძლიათ მიუთითოთ საწყისი DN მომხმარებლებისთვის და ჯგუფებისთვის Advanced ტაბში");
        m.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", "<b>გაფრთხილება:</b> PHP LDAP მოდული არ არის ინსტალირებული, ბექენდი არ იმუშავებს. თხოვეთ თქვენს ადმინისტრატორს დააინსტალიროს ის.");
        m.insert("Connection Settings", "კავშირის პარამეტრები");
        m.insert("Configuration Active", "კონფიგურაცია აქტიურია");
        m.insert("When unchecked, this configuration will be skipped.", "როცა გადანიშნულია, ეს კონფიგურაცია გამოტოვებული იქნება.");
        m.insert("User Login Filter", "მომხმარებლის ფილტრი");
        m.insert("Backup (Replica) Host", "ბექაფ  (რეპლიკა)  ჰოსტი");
        m.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.", "მიუთითეთ რაიმე ბექაფ ჰოსტი. ის უნდა იყოს ძირითადი LDAP/AD სერვერის რეპლიკა.");
        m.insert("Backup (Replica) Port", "ბექაფ (რეპლიკა) პორტი");
        m.insert("Disable Main Server", "გამორთეთ ძირითადი სერვერი");
        m.insert("Case insensitve LDAP server (Windows)", "LDAP server (Windows)");
        m.insert("Turn off SSL certificate validation.", "გამორთეთ SSL სერთიფიკატის ვალიდაცია.");
        m.insert("Cache Time-To-Live", "ქეშის სიცოცხლის ხანგრძლივობა");
        m.insert("in seconds. A change empties the cache.", "წამებში. ცვლილება ასუფთავებს ქეშს.");
        m.insert("Directory Settings", "დირექტორიის პარამეტრები");
        m.insert("User Display Name Field", "მომხმარებლის დისფლეის სახელის ფილდი");
        m.insert("Base User Tree", "ძირითად მომხმარებელთა სია");
        m.insert("One User Base DN per line", "ერთი მომხმარებლის საწყისი DN ერთ ხაზზე");
        m.insert("User Search Attributes", "მომხმარებლის ძებნის ატრიბუტი");
        m.insert("Optional; one attribute per line", "ოფციონალური; თითო ატრიბუტი თითო ხაზზე");
        m.insert("Group Display Name Field", "ჯგუფის დისფლეის სახელის ფილდი");
        m.insert("Base Group Tree", "ძირითად ჯგუფთა სია");
        m.insert("One Group Base DN per line", "ერთი ჯგუფის საწყისი DN ერთ ხაზზე");
        m.insert("Group Search Attributes", "ჯგუფური ძებნის ატრიბუტი");
        m.insert("Group-Member association", "ჯგუფის წევრობის ასოციაცია");
        m.insert("Special Attributes", "სპეციალური ატრიბუტები");
        m.insert("Quota Field", "ქვოტას ველი");
        m.insert("Quota Default", "საწყისი ქვოტა");
        m.insert("in bytes", "ბაიტებში");
        m.insert("Email Field", "იმეილის ველი");
        m.insert("User Home Folder Naming Rule", "მომხმარებლის Home დირექტორიის სახელების დარქმევის წესი");
        m.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "დატოვეთ ცარიელი მომხმარებლის სახელი (default). სხვა დანარჩენში მიუთითეთ LDAP/AD ატრიბუტი.");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";

    pub static ref PLURAL_TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
        m.insert("_%s group found_::_%s groups found_", vec![""]);
        m.insert("_%s user found_::_%s users found_", vec![""]);
        m
    };
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_translation(key: &str, n: usize) -> Option<&'static str> {
    if let Some(forms) = PLURAL_TRANSLATIONS.get(key) {
        // Implementación simplificada - normalmente se usaría una función de pluralización basada en PLURAL_FORMS
        if forms.is_empty() {
            None
        } else {
            // Para este idioma específico, siempre usamos la primera forma (según "nplurals=1")
            Some(forms[0])
        }
    } else {
        None
    }
}