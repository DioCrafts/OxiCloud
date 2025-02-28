use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("Could not move %s - File with this name already exists", "%s কে স্থানান্তর করা সম্ভব হলো না -  এই নামের ফাইল বিদ্যমান");
        map.insert("Could not move %s", "%s  কে স্থানান্তর করা সম্ভব হলো না");
        map.insert("File name cannot be empty.", "ফাইলের নামটি ফাঁকা রাখা যাবে না।");
        map.insert("No file was uploaded. Unknown error", "কোন ফাইল আপলোড করা হয় নি। সমস্যার কারণটি অজ্ঞাত।");
        map.insert("There is no error, the file uploaded with success", "কোন সমস্যা হয় নি, ফাইল আপলোড সুসম্পন্ন হয়েছে।");
        map.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "আপলোড করা  ফাইলটি php.ini তে বর্ণিত  upload_max_filesize নির্দেশিত আয়তন অতিক্রম করছেঃ");
        map.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "আপলোড করা ফাইলটি  HTML  ফর্মে উল্লিখিত MAX_FILE_SIZE নির্ধারিত ফাইলের সর্বোচ্চ আকার  অতিক্রম করতে চলেছে ");
        map.insert("The uploaded file was only partially uploaded", "আপলোড করা ফাইলটি আংশিক আপলোড করা হয়েছে");
        map.insert("No file was uploaded", "কোন ফাইল আপলোড করা হয় নি");
        map.insert("Missing a temporary folder", "অস্থায়ী ফোল্ডারটি হারানো গিয়েছে");
        map.insert("Failed to write to disk", "ডিস্কে লিখতে ব্যর্থ");
        map.insert("Invalid directory.", "ভুল ডিরেক্টরি");
        map.insert("Files", "ফাইল");
        map.insert("Not enough space available", "যথেষ্ঠ পরিমাণ স্থান নেই");
        map.insert("Upload cancelled.", "আপলোড বাতিল করা হয়েছে।");
        map.insert("File upload is in progress. Leaving the page now will cancel the upload.", "ফাইল আপলোড চলমান। এই পৃষ্ঠা পরিত্যাগ করলে আপলোড বাতিল করা হবে।");
        map.insert("{new_name} already exists", "{new_name} টি বিদ্যমান");
        map.insert("Share", "ভাগাভাগি কর");
        map.insert("Rename", "পূনঃনামকরণ");
        map.insert("Pending", "মুলতুবি");
        map.insert("replaced {new_name} with {old_name}", "{new_name} কে {old_name} নামে প্রতিস্থাপন করা হয়েছে");
        map.insert("undo", "ক্রিয়া প্রত্যাহার");
        map.insert("_%n folder_::_%n folders_", "");
        map.insert("_%n file_::_%n files_", "");
        map.insert("_Uploading %n file_::_Uploading %n files_", "");
        map.insert("'.' is an invalid file name.", "টি একটি অননুমোদিত নাম।");
        map.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "নামটি সঠিক নয়,  '\\', '/', '<', '>', ':', '\"', '|', '?' এবং  '*'  অনুমোদিত নয়।");
        map.insert("Error", "সমস্যা");
        map.insert("Name", "রাম");
        map.insert("Size", "আকার");
        map.insert("Modified", "পরিবর্তিত");
        map.insert("Upload", "আপলোড");
        map.insert("File handling", "ফাইল হ্যার্ডলিং");
        map.insert("Maximum upload size", "আপলোডের সর্বোচ্চ আকার");
        map.insert("max. possible: ", "অনুমোদিত  সর্বোচ্চ  আকার");
        map.insert("Needed for multi-file and folder downloads.", "একাধিক ফাইল এবং ফোল্ডার  ডাউনলোড করার জন্য আবশ্যক।");
        map.insert("Enable ZIP-download", "ZIP ডাউনলোড সক্রিয় কর");
        map.insert("0 is unlimited", "০ এর অর্থ অসীম");
        map.insert("Maximum input size for ZIP files", "ZIP ফাইলের ইনপুটের সর্বোচ্চ আকার");
        map.insert("Save", "সংরক্ষণ");
        map.insert("New", "নতুন");
        map.insert("Text file", "টেক্সট ফাইল");
        map.insert("Folder", "ফোল্ডার");
        map.insert("From link", " লিংক থেকে");
        map.insert("Cancel upload", "আপলোড বাতিল কর");
        map.insert("Nothing in here. Upload something!", "এখানে কিছুই নেই। কিছু আপলোড করুন !");
        map.insert("Download", "ডাউনলোড");
        map.insert("Unshare", "ভাগাভাগি বাতিল ");
        map.insert("Delete", "মুছে");
        map.insert("Upload too large", "আপলোডের আকারটি অনেক বড়");
        map.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "আপনি এই সার্ভারে আপলোড করার জন্য অনুমোদিত ফাইলের সর্বোচ্চ আকারের চেয়ে বৃহদাকার ফাইল আপলোড করার চেষ্টা করছেন ");
        map.insert("Files are being scanned, please wait.", "ফাইলগুলো স্ক্যান করা হচ্ছে, দয়া করে অপেক্ষা করুন।");
        map.insert("Current scanning", "বর্তমান স্ক্যানিং");
        map
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

#[allow(dead_code)]
pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

#[allow(dead_code)]
pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}