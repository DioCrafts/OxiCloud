use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Hindi translations
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Sunday", "रविवार");
    map.insert("Monday", "सोमवार");
    map.insert("Tuesday", "मंगलवार");
    map.insert("Wednesday", "बुधवार");
    map.insert("Thursday", "बृहस्पतिवार");
    map.insert("Friday", "शुक्रवार");
    map.insert("Saturday", "शनिवार");
    map.insert("January", "जनवरी");
    map.insert("February", "फरवरी");
    map.insert("March", "मार्च");
    map.insert("April", "अप्रैल");
    map.insert("May", "मई");
    map.insert("June", "जून");
    map.insert("July", "जुलाई");
    map.insert("August", "अगस्त");
    map.insert("September", "सितम्बर");
    map.insert("October", "अक्टूबर");
    map.insert("November", "नवंबर");
    map.insert("December", "दिसम्बर");
    map.insert("Settings", "सेटिंग्स");
    map.insert("Share", "साझा करें");
    map.insert("Error", "त्रुटि");
    map.insert("Password", "पासवर्ड");
    map.insert("Send", "भेजें");
    map.insert("No people found", "कोई व्यक्ति नहीं मिले ");
    map.insert("Sending ...", "भेजा जा रहा है");
    map.insert("Email sent", "ईमेल भेज दिया गया है ");
    map.insert("Warning", "चेतावनी ");
    map.insert("Add", "डाले");
    map.insert("Use the following link to reset your password: {link}", "आगे दिये गये लिंक का उपयोग पासवर्ड बदलने के लिये किजीये: {link}");
    map.insert("You will receive a link to reset your password via Email.", "पासवर्ड बदलने कि लिंक आपको ई-मेल द्वारा भेजी जायेगी|");
    map.insert("Username", "प्रयोक्ता का नाम");
    map.insert("Your password was reset", "आपका पासवर्ड बदला गया है");
    map.insert("New password", "नया पासवर्ड");
    map.insert("Personal", "यक्तिगत");
    map.insert("Users", "उपयोगकर्ता");
    map.insert("Apps", "Apps");
    map.insert("Help", "सहयोग");
    map.insert("Cloud not found", "क्लौड नहीं मिला ");
    map.insert("Security Warning", "सुरक्षा चेतावनी ");
    map.insert("Create an <strong>admin account</strong>", "व्यवस्थापक खाता बनाएँ");
    map.insert("Advanced", "उन्नत");
    map.insert("Data folder", "डाटा फोल्डर");
    map.insert("Configure the database", "डेटाबेस कॉन्फ़िगर करें ");
    map.insert("will be used", "उपयोग होगा");
    map.insert("Database user", "डेटाबेस उपयोगकर्ता");
    map.insert("Database password", "डेटाबेस पासवर्ड");
    map.insert("Database name", "डेटाबेस का नाम");
    map.insert("Finish setup", "सेटअप समाप्त करे");
    map.insert("Log out", "लोग  आउट");
    map.insert("remember", "याद रखें");
    map
});

/// Plural form information for Hindi language
pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

/// Pluralization mappings for different phrases
pub static PLURAL_TRANSLATIONS: Lazy<HashMap<&'static str, Vec<&'static str>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("_%n minute ago_::_%n minutes ago_", vec!["", ""]);
    map.insert("_%n hour ago_::_%n hours ago_", vec!["", ""]);
    map.insert("_%n day ago_::_%n days ago_", vec!["", ""]);
    map.insert("_%n month ago_::_%n months ago_", vec!["", ""]);
    map.insert("_{count} file conflict_::_{count} file conflicts_", vec!["", ""]);
    map
});

/// Get a translation for a key
pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

/// Get a pluralized translation
pub fn get_plural_translation(key: &str, n: usize) -> Option<&'static str> {
    let plural_idx = if n != 1 { 1 } else { 0 };
    PLURAL_TRANSLATIONS.get(key).and_then(|forms| forms.get(plural_idx).copied())
}