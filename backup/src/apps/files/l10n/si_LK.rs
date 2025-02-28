use std::collections::HashMap;
use rust_i18n::i18n;

i18n!("si_LK");

pub fn initialize_si_lk_translations() -> (HashMap<&'static str, &'static str>, &'static str) {
    let mut translations = HashMap::new();
    
    translations.insert("No file was uploaded. Unknown error", "ගොනුවක් උඩුගත නොවුනි. නොහැඳිනු දෝෂයක්");
    translations.insert("There is no error, the file uploaded with success", "දෝෂයක් නොමැත. සාර්ථකව ගොනුව උඩුගත කෙරුණි");
    translations.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "උඩුගත කළ ගොනුවේ විශාලත්වය HTML පෝරමයේ නියම කළ ඇති MAX_FILE_SIZE විශාලත්වයට වඩා වැඩිය");
    translations.insert("The uploaded file was only partially uploaded", "උඩුගත කළ ගොනුවේ කොටසක් පමණක් උඩුගත විය");
    translations.insert("No file was uploaded", "ගොනුවක් උඩුගත නොවුණි");
    translations.insert("Missing a temporary folder", "තාවකාලික ෆොල්ඩරයක් අතුරුදහන්");
    translations.insert("Failed to write to disk", "තැටිගත කිරීම අසාර්ථකයි");
    translations.insert("Files", "ගොනු");
    translations.insert("Upload cancelled.", "උඩුගත කිරීම අත් හරින්න ලදී");
    translations.insert("File upload is in progress. Leaving the page now will cancel the upload.", "උඩුගතකිරීමක් සිදුවේ. පිටුව හැර යාමෙන් එය නැවතෙනු ඇත");
    translations.insert("Share", "බෙදා හදා ගන්න");
    translations.insert("Rename", "නැවත නම් කරන්න");
    translations.insert("undo", "නිෂ්ප්‍රභ කරන්න");
    translations.insert("_%n folder_::_%n folders_", "");
    translations.insert("_%n file_::_%n files_", "");
    translations.insert("_Uploading %n file_::_Uploading %n files_", "");
    translations.insert("Error", "දෝෂයක්");
    translations.insert("Name", "නම");
    translations.insert("Size", "ප්‍රමාණය");
    translations.insert("Modified", "වෙනස් කළ");
    translations.insert("Upload", "උඩුගත කරන්න");
    translations.insert("File handling", "ගොනු පරිහරණය");
    translations.insert("Maximum upload size", "උඩුගත කිරීමක උපරිම ප්‍රමාණය");
    translations.insert("max. possible: ", "හැකි උපරිමය:");
    translations.insert("Needed for multi-file and folder downloads.", "බහු-ගොනු හා ෆොල්ඩර බාගත කිරීමට අවශ්‍යයි");
    translations.insert("Enable ZIP-download", "ZIP-බාගත කිරීම් සක්‍රිය කරන්න");
    translations.insert("0 is unlimited", "0 යනු සීමාවක් නැති බවය");
    translations.insert("Maximum input size for ZIP files", "ZIP ගොනු සඳහා දැමිය හැකි උපරිම විශාලතවය");
    translations.insert("Save", "සුරකින්න");
    translations.insert("New", "නව");
    translations.insert("Text file", "පෙළ ගොනුව");
    translations.insert("Folder", "ෆෝල්ඩරය");
    translations.insert("From link", "යොමුවෙන්");
    translations.insert("Cancel upload", "උඩුගත කිරීම අත් හරින්න");
    translations.insert("Nothing in here. Upload something!", "මෙහි කිසිවක් නොමැත. යමක් උඩුගත කරන්න");
    translations.insert("Download", "බාන්න");
    translations.insert("Unshare", "නොබෙදු");
    translations.insert("Delete", "මකා දමන්න");
    translations.insert("Upload too large", "උඩුගත කිරීම විශාල වැඩිය");
    translations.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "ඔබ උඩුගත කිරීමට තැත් කරන ගොනු මෙම සේවාදායකයා උඩුගත කිරීමට ඉඩදී ඇති උපරිම ගොනු විශාලත්වයට වඩා වැඩිය");
    translations.insert("Files are being scanned, please wait.", "ගොනු පරික්ෂා කෙරේ. මඳක් රැඳී සිටින්න");
    translations.insert("Current scanning", "වර්තමාන පරික්ෂාව");
    
    let plural_forms = "nplurals=2; plural=(n != 1);";
    
    (translations, plural_forms)
}