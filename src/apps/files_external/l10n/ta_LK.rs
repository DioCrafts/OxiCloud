use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Access granted", "அனுமதி வழங்கப்பட்டது");
    m.insert("Error configuring Dropbox storage", "Dropbox சேமிப்பை தகவமைப்பதில் வழு");
    m.insert("Grant access", "அனுமதியை வழங்கல்");
    m.insert("Please provide a valid Dropbox app key and secret.", "தயவுசெய்து ஒரு செல்லுபடியான Dropbox செயலி சாவி மற்றும் இரகசியத்தை வழங்குக. ");
    m.insert("Error configuring Google Drive storage", "Google இயக்க சேமிப்பகத்தை தகமைப்பதில் வழு");
    m.insert("External Storage", "வெளி சேமிப்பு");
    m.insert("Folder name", "கோப்புறை பெயர்");
    m.insert("Configuration", "தகவமைப்பு");
    m.insert("Options", "தெரிவுகள்");
    m.insert("Applicable", "பயன்படத்தக்க");
    m.insert("None set", "தொகுப்பில்லா");
    m.insert("All Users", "பயனாளர்கள் எல்லாம்");
    m.insert("Groups", "குழுக்கள்");
    m.insert("Users", "பயனாளர்");
    m.insert("Delete", "நீக்குக");
    m.insert("Enable User External Storage", "பயனாளர் வெளி சேமிப்பை இயலுமைப்படுத்துக");
    m.insert("Allow users to mount their own external storage", "பயனாளர் அவர்களுடைய சொந்த வெளியக சேமிப்பை ஏற்ற அனுமதிக்க");
    m.insert("SSL root certificates", "SSL வேர் சான்றிதழ்கள்");
    m.insert("Import Root Certificate", "வேர் சான்றிதழை இறக்குமதி செய்க");
    m
});

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";