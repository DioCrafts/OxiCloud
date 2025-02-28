use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Deletion failed", "நீக்கம் தோல்வியடைந்தது");
        m.insert("Error", "வழு");
        m.insert("_%s group found_::_%s groups found_", "");
        m.insert("_%s user found_::_%s users found_", "");
        m.insert("Save", "சேமிக்க ");
        m.insert("Help", "உதவி");
        m.insert("Host", "ஓம்புனர்");
        m.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "நீங்கள் SSL சேவையை தவிர உடன்படு வரைமுறையை தவிர்க்க முடியும். பிறகு ldaps:.// உடன் ஆரம்பிக்கவும்");
        m.insert("Port", "துறை ");
        m.insert("User DN", "பயனாளர்  DN");
        m.insert("Password", "கடவுச்சொல்");
        m.insert("You can specify Base DN for users and groups in the Advanced tab", "நீங்கள் பயனாளர்களுக்கும் மேன்மை தத்தலில் உள்ள குழுவிற்கும் தள DN ஐ குறிப்பிடலாம் ");
        m.insert("Back", "பின்னுக்கு");
        m.insert("Case insensitve LDAP server (Windows)", "உணர்ச்சியான LDAP சேவையகம் (சாளரங்கள்)");
        m.insert("Turn off SSL certificate validation.", "SSL சான்றிதழின் செல்லுபடியை நிறுத்திவிடவும்");
        m.insert("in seconds. A change empties the cache.", "செக்கன்களில். ஒரு மாற்றம் இடைமாற்றுநினைவகத்தை வெற்றிடமாக்கும்.");
        m.insert("User Display Name Field", "பயனாளர் காட்சிப்பெயர் புலம்");
        m.insert("Base User Tree", "தள பயனாளர் மரம்");
        m.insert("Group Display Name Field", "குழுவின் காட்சி பெயர் புலம் ");
        m.insert("Base Group Tree", "தள குழு மரம்");
        m.insert("Group-Member association", "குழு உறுப்பினர் சங்கம்");
        m.insert("in bytes", "bytes களில் ");
        m.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "பயனாளர் பெயரிற்கு வெற்றிடமாக விடவும் (பொது இருப்பு). இல்லாவிடின் LDAP/AD பண்புக்கூறை குறிப்பிடவும்.");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}