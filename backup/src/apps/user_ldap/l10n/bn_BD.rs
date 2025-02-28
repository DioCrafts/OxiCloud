use std::collections::HashMap;
use once_cell::sync::Lazy;

// Translations map
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Error", "সমস্যা");
    translations.insert("Save", "সংরক্ষণ");
    translations.insert("Help", "সহায়িকা");
    translations.insert("Host", "হোস্ট");
    translations.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "SSL আবশ্যক  না হলে  আপনি এই প্রটোকলটি মুছে ফেলতে পারেন । এরপর শুরু করুন এটা দিয়ে ldaps://");
    translations.insert("Port", "পোর্ট");
    translations.insert("User DN", "ব্যবহারকারি  DN");
    translations.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. পরিচয় গোপন রেখে অধিগমনের জন্য  DN এবং কূটশব্দটি ফাঁকা রাখুন।");
    translations.insert("Password", "কূটশব্দ");
    translations.insert("For anonymous access, leave DN and Password empty.", "অজ্ঞাতকুলশীল অধিগমনের জন্য DN এবং কূটশব্দটি ফাঁকা রাখুন।");
    translations.insert("You can specify Base DN for users and groups in the Advanced tab", "সুচারু ট্যঅবে গিয়ে আপনি ব্যবহারকারি এবং গোষ্ঠীসমূহের জন্য ভিত্তি DN নির্ধারণ করতে পারেন।");
    translations.insert("User Login Filter", "ব্যবহারকারির প্রবেশ ছাঁকনী");
    translations.insert("Case insensitve LDAP server (Windows)", "বর্ণ অসংবেদী LDAP  সার্ভার (উইন্ডোজ)");
    translations.insert("Turn off SSL certificate validation.", "SSL সনদপত্র যাচাইকরণ বন্ধ রাক।");
    translations.insert("in seconds. A change empties the cache.", "সেকেন্ডে। কোন পরিবর্তন ক্যাসে খালি করবে।");
    translations.insert("User Display Name Field", "ব্যবহারকারীর প্রদর্শিতব্য নামের ক্ষেত্র");
    translations.insert("Base User Tree", "ভিত্তি ব্যবহারকারি বৃক্ষাকারে");
    translations.insert("Group Display Name Field", "গোষ্ঠীর প্রদর্শিতব্য নামের ক্ষেত্র");
    translations.insert("Base Group Tree", "ভিত্তি গোষ্ঠী বৃক্ষাকারে");
    translations.insert("Group-Member association", "গোষ্ঠী-সদস্য সংস্থাপন");
    translations.insert("in bytes", "বাইটে");
    translations.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "ব্যবহারকারী নামের জন্য ফাঁকা রাখুন (পূর্বনির্ধারিত)। অন্যথায়, LDAP/AD বৈশিষ্ট্য নির্ধারণ করুন।");
    translations
});

// Plural forms map
pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

// Plural translations
pub static PLURAL_TRANSLATIONS: Lazy<HashMap<&'static str, Vec<&'static str>>> = Lazy::new(|| {
    let mut plural_translations = HashMap::new();
    plural_translations.insert("_%s group found_::_%s groups found_", vec!["", ""]);
    plural_translations.insert("_%s user found_::_%s users found_", vec!["", ""]);
    plural_translations
});

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_translation(key: &str, count: usize) -> Option<&'static str> {
    PLURAL_TRANSLATIONS.get(key).and_then(|forms| {
        let plural_index = if count != 1 { 1 } else { 0 };
        forms.get(plural_index).copied()
    })
}