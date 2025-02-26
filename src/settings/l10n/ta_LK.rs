use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Unable to load list from App Store", "செயலி சேமிப்பிலிருந்து பட்டியலை ஏற்றமுடியாதுள்ளது");
        m.insert("Authentication error", "அத்தாட்சிப்படுத்தலில் வழு");
        m.insert("Group already exists", "குழு ஏற்கனவே உள்ளது");
        m.insert("Unable to add group", "குழுவை சேர்க்க முடியாது");
        m.insert("Email saved", "மின்னஞ்சல் சேமிக்கப்பட்டது");
        m.insert("Invalid email", "செல்லுபடியற்ற மின்னஞ்சல்");
        m.insert("Unable to delete group", "குழுவை நீக்க முடியாது");
        m.insert("Unable to delete user", "பயனாளரை நீக்க முடியாது");
        m.insert("Language changed", "மொழி மாற்றப்பட்டது");
        m.insert("Invalid request", "செல்லுபடியற்ற வேண்டுகோள்");
        m.insert("Unable to add user to group %s", "குழு %s இல் பயனாளரை சேர்க்க முடியாது");
        m.insert("Unable to remove user from group %s", "குழு %s இலிருந்து பயனாளரை நீக்கமுடியாது");
        m.insert("Disable", "இயலுமைப்ப");
        m.insert("Enable", "இயலுமைப்படுத்துக");
        m.insert("Error", "வழு");
        m.insert("Update", "இற்றைப்படுத்தல்");
        m.insert("Saving...", "சேமிக்கப்படுகிறது...");
        m.insert("undo", "முன் செயல் நீக்கம் ");
        m.insert("Groups", "குழுக்கள்");
        m.insert("Group Admin", "குழு நிர்வாகி");
        m.insert("Delete", "நீக்குக");
        m.insert("__language_name__", "_மொழி_பெயர்_");
        m.insert("Security Warning", "பாதுகாப்பு எச்சரிக்கை");
        m.insert("More", "மேலதிக");
        m.insert("Less", "குறைவான");
        m.insert("Developed by the <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud community</a>, the <a href=\"https://github.com/owncloud\" target=\"_blank\">source code</a> is licensed under the <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.", "Developed by the <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud community</a>, the <a href=\"https://github.com/owncloud\" target=\"_blank\">source code</a> is licensed under the <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.");
        m.insert("Add your App", "உங்களுடைய செயலியை சேர்க்க");
        m.insert("More Apps", "மேலதிக செயலிகள்");
        m.insert("Select an App", "செயலி ஒன்றை தெரிவுசெய்க");
        m.insert("See application page at apps.owncloud.com", "apps.owncloud.com இல் செயலி பக்கத்தை பார்க்க");
        m.insert("<span class=\"licence\"></span>-licensed by <span class=\"author\"></span>", "<span class=\"அனுமதிப்பத்திரம்\"></span>-அனுமதி பெற்ற <span class=\"ஆசிரியர்\"></span>");
        m.insert("You have used <strong>%s</strong> of the available <strong>%s</strong>", "நீங்கள் <strong>%s</strong> இலுள்ள <strong>%s</strong>பயன்படுத்தியுள்ளீர்கள்");
        m.insert("Password", "கடவுச்சொல்");
        m.insert("Your password was changed", "உங்களுடைய கடவுச்சொல் மாற்றப்பட்டுள்ளது");
        m.insert("Unable to change your password", "உங்களுடைய கடவுச்சொல்லை மாற்றமுடியாது");
        m.insert("Current password", "தற்போதைய கடவுச்சொல்");
        m.insert("New password", "புதிய கடவுச்சொல்");
        m.insert("Change password", "கடவுச்சொல்லை மாற்றுக");
        m.insert("Email", "மின்னஞ்சல்");
        m.insert("Your email address", "உங்களுடைய மின்னஞ்சல் முகவரி");
        m.insert("Fill in an email address to enable password recovery", "கடவுச்சொல் மீள் பெறுவதை இயலுமைப்படுத்துவதற்கு மின்னஞ்சல் முகவரியை இயலுமைப்படுத்துக");
        m.insert("Language", "மொழி");
        m.insert("Help translate", "மொழிபெயர்க்க உதவி");
        m.insert("Encryption", "மறைக்குறியீடு");
        m.insert("Create", "உருவாக்குக");
        m.insert("Other", "மற்றவை");
        m.insert("Username", "பயனாளர் பெயர்");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}