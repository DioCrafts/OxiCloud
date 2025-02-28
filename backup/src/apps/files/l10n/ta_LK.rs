use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("No file was uploaded. Unknown error", "ஒரு கோப்பும் பதிவேற்றப்படவில்லை. அறியப்படாத வழு");
        m.insert("There is no error, the file uploaded with success", "இங்கு வழு இல்லை, கோப்பு வெற்றிகரமாக பதிவேற்றப்பட்டது");
        m.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "பதிவேற்றப்பட்ட கோப்பானது HTML  படிவத்தில் குறிப்பிடப்பட்டுள்ள MAX_FILE_SIZE  directive ஐ விட கூடியது");
        m.insert("The uploaded file was only partially uploaded", "பதிவேற்றப்பட்ட கோப்பானது பகுதியாக மட்டுமே பதிவேற்றப்பட்டுள்ளது");
        m.insert("No file was uploaded", "எந்த கோப்பும் பதிவேற்றப்படவில்லை");
        m.insert("Missing a temporary folder", "ஒரு தற்காலிகமான கோப்புறையை காணவில்லை");
        m.insert("Failed to write to disk", "வட்டில் எழுத முடியவில்லை");
        m.insert("Files", "கோப்புகள்");
        m.insert("Upload cancelled.", "பதிவேற்றல் இரத்து செய்யப்பட்டுள்ளது");
        m.insert("File upload is in progress. Leaving the page now will cancel the upload.", "கோப்பு பதிவேற்றம் செயல்பாட்டில் உள்ளது. இந்தப் பக்கத்திலிருந்து வெறியேறுவதானது பதிவேற்றலை இரத்து செய்யும்.");
        m.insert("{new_name} already exists", "{new_name} ஏற்கனவே உள்ளது");
        m.insert("Share", "பகிர்வு");
        m.insert("Rename", "பெயர்மாற்றம்");
        m.insert("Pending", "நிலுவையிலுள்ள");
        m.insert("replaced {new_name} with {old_name}", "{new_name} ஆனது {old_name} இனால் மாற்றப்பட்டது");
        m.insert("undo", "முன் செயல் நீக்கம் ");
        m.insert("_%n folder_::_%n folders_", "");
        m.insert("_%n file_::_%n files_", "");
        m.insert("_Uploading %n file_::_Uploading %n files_", "");
        m.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "செல்லுபடியற்ற பெயர்,'\\', '/', '<', '>', ':', '\"', '|', '?' மற்றும் '*' ஆகியன அனுமதிக்கப்படமாட்டாது.");
        m.insert("Error", "வழு");
        m.insert("Name", "பெயர்");
        m.insert("Size", "அளவு");
        m.insert("Modified", "மாற்றப்பட்டது");
        m.insert("Upload", "பதிவேற்றுக");
        m.insert("File handling", "கோப்பு கையாளுதல்");
        m.insert("Maximum upload size", "பதிவேற்றக்கூடிய ஆகக்கூடிய அளவு ");
        m.insert("max. possible: ", "ஆகக் கூடியது:");
        m.insert("Needed for multi-file and folder downloads.", "பல்வேறுப்பட்ட கோப்பு மற்றும் கோப்புறைகளை பதிவிறக்க தேவையானது.");
        m.insert("Enable ZIP-download", "ZIP பதிவிறக்கலை இயலுமைப்படுத்துக");
        m.insert("0 is unlimited", "0 ஆனது எல்லையற்றது");
        m.insert("Maximum input size for ZIP files", "ZIP கோப்புகளுக்கான ஆகக்கூடிய உள்ளீட்டு அளவு");
        m.insert("Save", "சேமிக்க ");
        m.insert("New", "புதிய");
        m.insert("Text file", "கோப்பு உரை");
        m.insert("Folder", "கோப்புறை");
        m.insert("From link", "இணைப்பிலிருந்து");
        m.insert("Cancel upload", "பதிவேற்றலை இரத்து செய்க");
        m.insert("Nothing in here. Upload something!", "இங்கு ஒன்றும் இல்லை. ஏதாவது பதிவேற்றுக!");
        m.insert("Download", "பதிவிறக்குக");
        m.insert("Unshare", "பகிரப்படாதது");
        m.insert("Delete", "நீக்குக");
        m.insert("Upload too large", "பதிவேற்றல் மிகப்பெரியது");
        m.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "நீங்கள் பதிவேற்ற முயற்சிக்கும் கோப்புகளானது இந்த சேவையகத்தில் கோப்பு பதிவேற்றக்கூடிய ஆகக்கூடிய அளவிலும் கூடியது.");
        m.insert("Files are being scanned, please wait.", "கோப்புகள் வருடப்படுகின்றன, தயவுசெய்து காத்திருங்கள்.");
        m.insert("Current scanning", "தற்போது வருடப்படுபவை");
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