use std::collections::HashMap;
use once_cell::sync::Lazy;

/// UG (Uyghur) translations
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Unable to load list from App Store", "ئەپ بازىرىدىن تىزىمنى يۈكلىيەلمىدى");
    m.insert("Authentication error", "سالاھىيەت دەلىللەش خاتالىقى");
    m.insert("Group already exists", "گۇرۇپپا مەۋجۇت");
    m.insert("Unable to add group", "گۇرۇپپا قوشقىلى بولمايدۇ");
    m.insert("Email saved", "تورخەت ساقلاندى");
    m.insert("Invalid email", "ئىناۋەتسىز تورخەت");
    m.insert("Unable to delete group", "گۇرۇپپىنى ئۆچۈرەلمىدى");
    m.insert("Unable to delete user", "ئىشلەتكۈچىنى ئۆچۈرەلمىدى");
    m.insert("Language changed", "تىل ئۆزگەردى");
    m.insert("Invalid request", "ئىناۋەتسىز ئىلتىماس");
    m.insert("Admins can't remove themself from the admin group", "باشقۇرغۇچى ئۆزىنى باشقۇرۇش گۇرۇپپىسىدىن چىقىرىۋېتەلمەيدۇ");
    m.insert("Unable to add user to group %s", "ئىشلەتكۈچىنى %s گۇرۇپپىغا قوشالمايدۇ");
    m.insert("Unable to remove user from group %s", "ئىشلەتكۈچىنى %s گۇرۇپپىدىن چىقىرىۋېتەلمەيدۇ");
    m.insert("Couldn't update app.", "ئەپنى يېڭىلىيالمايدۇ.");
    m.insert("Update to {appversion}", "{appversion} غا يېڭىلايدۇ");
    m.insert("Disable", "چەكلە");
    m.insert("Enable", "قوزغات");
    m.insert("Please wait....", "سەل كۈتۈڭ…");
    m.insert("Updating....", "يېڭىلاۋاتىدۇ…");
    m.insert("Error while updating app", "ئەپنى يېڭىلاۋاتقاندا خاتالىق كۆرۈلدى");
    m.insert("Error", "خاتالىق");
    m.insert("Update", "يېڭىلا");
    m.insert("Updated", "يېڭىلاندى");
    m.insert("Saving...", "ساقلاۋاتىدۇ…");
    m.insert("deleted", "ئۆچۈرۈلگەن");
    m.insert("undo", "يېنىۋال");
    m.insert("Unable to remove user", "ئىشلەتكۈچىنى چىقىرىۋېتەلمەيدۇ");
    m.insert("Groups", "گۇرۇپپا");
    m.insert("Group Admin", "گۇرۇپپا باشقۇرغۇچى");
    m.insert("Delete", "ئۆچۈر");
    m.insert("add group", "گۇرۇپپا قوش");
    m.insert("A valid username must be provided", "چوقۇم ئىناۋەتلىك ئىشلەتكۈچى ئىسمىدىن بىرنى تەمىنلەش كېرەك");
    m.insert("Error creating user", "ئىشلەتكۈچى قۇرۇۋاتقاندا خاتالىق كۆرۈلدى");
    m.insert("A valid password must be provided", "چوقۇم ئىناۋەتلىك ئىم تەمىنلەش كېرەك");
    m.insert("__language_name__", "ئۇيغۇرچە");
    m.insert("Security Warning", "بىخەتەرلىك ئاگاھلاندۇرۇش");
    m.insert("Setup Warning", "ئاگاھلاندۇرۇش تەڭشەك");
    m.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "سىزنىڭ تور مۇلازىمېتىرىڭىز ھۆججەت قەدەمداشلاشقا يول قويىدىغان قىلىپ توغرا تەڭشەلمەپتۇ، چۈنكى WebDAV نىڭ ئېغىزى بۇزۇلغاندەك تۇرىدۇ.");
    m.insert("Module 'fileinfo' missing", "بۆلەك «ھۆججەت ئۇچۇرى» يوقالغان");
    m.insert("Sharing", "ھەمبەھىر");
    m.insert("Security", "بىخەتەرلىك");
    m.insert("Log", "خاتىرە");
    m.insert("Log level", "خاتىرە دەرىجىسى");
    m.insert("More", "تېخىمۇ كۆپ");
    m.insert("Less", "ئاز");
    m.insert("Version", "نەشرى");
    m.insert("Add your App", "ئەپىڭىزنى قوشۇڭ");
    m.insert("More Apps", "تېخىمۇ كۆپ ئەپلەر");
    m.insert("Select an App", "بىر ئەپ تاللاڭ");
    m.insert("User Documentation", "ئىشلەتكۈچى قوللانمىسى");
    m.insert("Administrator Documentation", "باشقۇرغۇچى قوللانمىسى");
    m.insert("Online Documentation", "توردىكى قوللانما");
    m.insert("Forum", "مۇنبەر");
    m.insert("Password", "ئىم");
    m.insert("Your password was changed", "ئىمىڭىز مۇۋەپپەقىيەتلىك ئۆزگەرتىلدى");
    m.insert("Unable to change your password", "ئىمنى ئۆزگەرتكىلى بولمايدۇ.");
    m.insert("Current password", "نۆۋەتتىكى ئىم");
    m.insert("New password", "يېڭى ئىم");
    m.insert("Change password", "ئىم ئۆزگەرت");
    m.insert("Email", "تورخەت");
    m.insert("Your email address", "تورخەت ئادرېسىڭىز");
    m.insert("Fill in an email address to enable password recovery", "ئىم ئەسلىگە كەلتۈرۈشتە ئىشلىتىدىغان تور خەت ئادرېسىنى تولدۇرۇڭ");
    m.insert("Language", "تىل");
    m.insert("Help translate", "تەرجىمىگە ياردەم");
    m.insert("WebDAV", "WebDAV");
    m.insert("Encryption", "شىفىرلاش");
    m.insert("Login Name", "تىزىمغا كىرىش ئاتى");
    m.insert("Create", "قۇر");
    m.insert("Default Storage", "كۆڭۈلدىكى ساقلىغۇچ");
    m.insert("Unlimited", "چەكسىز");
    m.insert("Other", "باشقا");
    m.insert("Username", "ئىشلەتكۈچى ئاتى");
    m.insert("Storage", "ساقلىغۇچ");
    m.insert("set new password", "يېڭى ئىم تەڭشە");
    m.insert("Default", "كۆڭۈلدىكى");
    m
});

/// Plural form configuration for Uyghur language
pub static PLURAL_FORMS: &str = "nplurals=1; plural=0;";

/// Get translated string for a given key
pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

/// Format a translation with arguments
pub fn format_translation(key: &str, args: &[&str]) -> String {
    if let Some(translation) = get_translation(key) {
        args.iter().enumerate().fold(translation.to_string(), |acc, (i, arg)| {
            acc.replace(&format!("%s", i + 1), arg).replace(&format!("%s"), arg)
        })
    } else {
        key.to_string()
    }
}