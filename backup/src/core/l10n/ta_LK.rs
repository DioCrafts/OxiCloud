use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Sunday", "ஞாயிற்றுக்கிழமை");
    map.insert("Monday", "திங்கட்கிழமை");
    map.insert("Tuesday", "செவ்வாய்க்கிழமை");
    map.insert("Wednesday", "புதன்கிழமை");
    map.insert("Thursday", "வியாழக்கிழமை");
    map.insert("Friday", "வெள்ளிக்கிழமை");
    map.insert("Saturday", "சனிக்கிழமை");
    map.insert("January", "தை");
    map.insert("February", "மாசி");
    map.insert("March", "பங்குனி");
    map.insert("April", "சித்திரை");
    map.insert("May", "வைகாசி");
    map.insert("June", "ஆனி");
    map.insert("July", "ஆடி");
    map.insert("August", "ஆவணி");
    map.insert("September", "புரட்டாசி");
    map.insert("October", "ஐப்பசி");
    map.insert("November", "கார்த்திகை");
    map.insert("December", "மார்கழி");
    map.insert("Settings", "அமைப்புகள்");
    map.insert("seconds ago", "செக்கன்களுக்கு முன்");
    map.insert("today", "இன்று");
    map.insert("yesterday", "நேற்று");
    map.insert("last month", "கடந்த மாதம்");
    map.insert("months ago", "மாதங்களுக்கு முன்");
    map.insert("last year", "கடந்த வருடம்");
    map.insert("years ago", "வருடங்களுக்கு முன்");
    map.insert("Choose", "தெரிவுசெய்க ");
    map.insert("Yes", "ஆம்");
    map.insert("No", "இல்லை");
    map.insert("Ok", "சரி");
    map.insert("Cancel", "இரத்து செய்க");
    map.insert("Share", "பகிர்வு");
    map.insert("Error", "வழு");
    map.insert("Error while sharing", "பகிரும் போதான வழு");
    map.insert("Error while unsharing", "பகிராமல் உள்ளப்போதான வழு");
    map.insert("Error while changing permissions", "அனுமதிகள் மாறும்போதான வழு");
    map.insert("Shared with you and the group {group} by {owner}", "உங்களுடனும் குழுவுக்கிடையிலும் {குழு} பகிரப்பட்டுள்ளது {உரிமையாளர்}");
    map.insert("Shared with you by {owner}", "உங்களுடன் பகிரப்பட்டுள்ளது {உரிமையாளர்}");
    map.insert("Password protect", "கடவுச்சொல்லை பாதுகாத்தல்");
    map.insert("Password", "கடவுச்சொல்");
    map.insert("Set expiration date", "காலாவதி தேதியை குறிப்பிடுக");
    map.insert("Expiration date", "காலவதியாகும் திகதி");
    map.insert("Share via email:", "மின்னஞ்சலினூடான பகிர்வு: ");
    map.insert("No people found", "நபர்கள் யாரும் இல்லை");
    map.insert("group", "குழு");
    map.insert("Resharing is not allowed", "மீள்பகிர்வதற்கு அனுமதி இல்லை ");
    map.insert("Shared in {item} with {user}", "{பயனாளர்} உடன் {உருப்படி} பகிரப்பட்டுள்ளது");
    map.insert("Unshare", "பகிரப்படாதது");
    map.insert("can edit", "தொகுக்க முடியும்");
    map.insert("access control", "கட்டுப்பாடான அணுகல்");
    map.insert("create", "உருவவாக்கல்");
    map.insert("update", "இற்றைப்படுத்தல்");
    map.insert("delete", "நீக்குக");
    map.insert("share", "பகிர்தல்");
    map.insert("Password protected", "கடவுச்சொல் பாதுகாக்கப்பட்டது");
    map.insert("Error unsetting expiration date", "காலாவதியாகும் திகதியை குறிப்பிடாமைக்கான வழு");
    map.insert("Error setting expiration date", "காலாவதியாகும் திகதியை குறிப்பிடுவதில் வழு");
    map.insert("Warning", "எச்சரிக்கை");
    map.insert("The object type is not specified.", "பொருள் வகை குறிப்பிடப்படவில்லை.");
    map.insert("Delete", "நீக்குக");
    map.insert("Add", "சேர்க்க");
    map.insert("Use the following link to reset your password: {link}", "உங்கள் கடவுச்சொல்லை மீளமைக்க பின்வரும் இணைப்பை பயன்படுத்தவும் : {இணைப்பு}");
    map.insert("You will receive a link to reset your password via Email.", "நீங்கள் மின்னஞ்சல் மூலம் உங்களுடைய கடவுச்சொல்லை மீளமைப்பதற்கான இணைப்பை பெறுவீர்கள். ");
    map.insert("Username", "பயனாளர் பெயர்");
    map.insert("Your password was reset", "உங்களுடைய கடவுச்சொல் மீளமைக்கப்பட்டது");
    map.insert("To login page", "புகுபதிகைக்கான பக்கம்");
    map.insert("New password", "புதிய கடவுச்சொல்");
    map.insert("Reset password", "மீளமைத்த கடவுச்சொல்");
    map.insert("Personal", "தனிப்பட்ட");
    map.insert("Users", "பயனாளர்");
    map.insert("Apps", "செயலிகள்");
    map.insert("Admin", "நிர்வாகம்");
    map.insert("Help", "உதவி");
    map.insert("Access forbidden", "அணுக தடை");
    map.insert("Cloud not found", "Cloud காணப்படவில்லை");
    map.insert("Security Warning", "பாதுகாப்பு எச்சரிக்கை");
    map.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", "குறிப்பிட்ட எண்ணிக்கை பாதுகாப்பான புறப்பாக்கி / உண்டாக்கிகள் இல்லை, தயவுசெய்து PHP OpenSSL நீட்சியை இயலுமைப்படுத்துக. ");
    map.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "பாதுகாப்பான சீரற்ற எண்ணிக்கையான புறப்பாக்கி இல்லையெனின், தாக்குனரால் கடவுச்சொல் மீளமைப்பு அடையாளவில்லைகள் முன்மொழியப்பட்டு உங்களுடைய கணக்கை கைப்பற்றலாம்.");
    map.insert("Create an <strong>admin account</strong>", "<strong> நிர்வாக கணக்கொன்றை </strong> உருவாக்குக");
    map.insert("Advanced", "உயர்ந்த");
    map.insert("Data folder", "தரவு கோப்புறை");
    map.insert("Configure the database", "தரவுத்தளத்தை தகவமைக்க");
    map.insert("will be used", "பயன்படுத்தப்படும்");
    map.insert("Database user", "தரவுத்தள பயனாளர்");
    map.insert("Database password", "தரவுத்தள கடவுச்சொல்");
    map.insert("Database name", "தரவுத்தள பெயர்");
    map.insert("Database tablespace", "தரவுத்தள அட்டவணை");
    map.insert("Database host", "தரவுத்தள ஓம்புனர்");
    map.insert("Finish setup", "அமைப்பை முடிக்க");
    map.insert("Log out", "விடுபதிகை செய்க");
    map.insert("Automatic logon rejected!", "தன்னிச்சையான புகுபதிகை நிராகரிப்பட்டது!");
    map.insert("If you did not change your password recently, your account may be compromised!", "உங்களுடைய கடவுச்சொல்லை அண்மையில் மாற்றவில்லையின், உங்களுடைய கணக்கு சமரசமாகிவிடும்!");
    map.insert("Please change your password to secure your account again.", "உங்களுடைய கணக்கை மீண்டும் பாதுகாக்க தயவுசெய்து உங்களுடைய கடவுச்சொல்லை மாற்றவும்.");
    map.insert("Lost your password?", "உங்கள் கடவுச்சொல்லை தொலைத்துவிட்டீர்களா?");
    map.insert("remember", "ஞாபகப்படுத்துக");
    map.insert("Log in", "புகுபதிகை");
    map
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

// Estructura para manejar plurales
pub struct PluralTranslations {
    singular: &'static str,
    plural: &'static str,
}

pub static PLURAL_TRANSLATIONS: Lazy<HashMap<&str, PluralTranslations>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("_%n minute ago_::_%n minutes ago_", PluralTranslations {
        singular: "",
        plural: "",
    });
    map.insert("_%n hour ago_::_%n hours ago_", PluralTranslations {
        singular: "",
        plural: "",
    });
    map.insert("_%n day ago_::_%n days ago_", PluralTranslations {
        singular: "",
        plural: "",
    });
    map.insert("_%n month ago_::_%n months ago_", PluralTranslations {
        singular: "",
        plural: "",
    });
    map.insert("_{count} file conflict_::_{count} file conflicts_", PluralTranslations {
        singular: "",
        plural: "",
    });
    map
});