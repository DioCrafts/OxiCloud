use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Deletion failed", "මකාදැමීම අසාර්ථකයි");
        m.insert("Success", "සාර්ථකයි");
        m.insert("Error", "දෝෂයක්");
        m.insert("Save", "සුරකින්න");
        m.insert("Help", "උදව්");
        m.insert("Host", "සත්කාරකය");
        m.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", 
                 "SSL අවශ්‍යය වන විට පමණක් හැර, අන් අවස්ථාවන්හිදී ප්‍රොටොකෝලය අත් හැරිය හැක. භාවිතා කරන විට ldaps:// ලෙස ආරම්භ කරන්න");
        m.insert("Port", "තොට");
        m.insert("Password", "මුර පදය");
        m.insert("User Login Filter", "පරිශීලක පිවිසුම් පෙරහන");
        m
    };

    pub static ref PLURAL_FORMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%s group found_::_%s groups found_", vec!["", ""]);
        m.insert("_%s user found_::_%s users found_", vec!["", ""]);
        m
    };
}

pub fn get_plural_form() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}