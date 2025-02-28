use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Authentication error", "සත්‍යාපන දෝෂයක්");
        m.insert("Group already exists", "කණ්ඩායම දැනටමත් තිබේ");
        m.insert("Unable to add group", "කාණඩයක් එක් කළ නොහැකි විය");
        m.insert("Email saved", "වි-තැපෑල සුරකින ලදී");
        m.insert("Invalid email", "අවලංගු වි-තැපෑල");
        m.insert("Unable to delete group", "කණ්ඩායම මැකීමට නොහැක");
        m.insert("Unable to delete user", "පරිශීලකයා මැකීමට නොහැක");
        m.insert("Language changed", "භාෂාව ාවනස් කිරීම");
        m.insert("Invalid request", "අවලංගු අයැදුමක්");
        m.insert("Unable to add user to group %s", "පරිශීලකයා %s කණ්ඩායමට එකතු කළ නොහැක");
        m.insert("Unable to remove user from group %s", "පරිශීලකයා %s කණ්ඩායමින් ඉවත් කළ නොහැක");
        m.insert("Disable", "අක්‍රිය කරන්න");
        m.insert("Enable", "සක්‍රිය කරන්න");
        m.insert("Error", "දෝෂයක්");
        m.insert("Update", "යාවත්කාල කිරීම");
        m.insert("Saving...", "සුරැකෙමින් පවතී...");
        m.insert("undo", "නිෂ්ප්‍රභ කරන්න");
        m.insert("Groups", "කණ්ඩායම්");
        m.insert("Group Admin", "කාණ්ඩ පරිපාලක");
        m.insert("Delete", "මකා දමන්න");
        m.insert("Security Warning", "ආරක්ෂක නිවේදනයක්");
        m.insert("Sharing", "හුවමාරු කිරීම");
        m.insert("Allow links", "යොමු සලසන්න");
        m.insert("Allow resharing", "යළි යළිත් හුවමාරුවට අවසර දෙමි");
        m.insert("Allow users to share items shared with them again", "හුවමාරු කළ  හුවමාරුවට අවසර දෙමි");
        m.insert("Allow users to share with anyone", "ඕනෑම අයෙකු හා හුවමාරුවට අවසර දෙමි");
        m.insert("Allow users to only share with users in their groups", "තම කණ්ඩායමේ අයෙකු හා පමණක් හුවමාරුවට අවසර දෙමි");
        m.insert("Log", "ලඝුව");
        m.insert("More", "වැඩි");
        m.insert("Less", "අඩු");
        m.insert("Developed by the <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud community</a>, the <a href=\"https://github.com/owncloud\" target=\"_blank\">source code</a> is licensed under the <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.", "නිපදන ලද්දේ <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud සමාජයෙන්</a>, the <a href=\"https://github.com/owncloud\" target=\"_blank\">මුල් කේතය </a>ලයිසන්ස් කර ඇත්තේ <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a> යටතේ.");
        m.insert("Add your App", "යෙදුමක් එක් කිරීම");
        m.insert("More Apps", "තවත් යෙදුම්");
        m.insert("Select an App", "යෙදුමක් තොරන්න");
        m.insert("Password", "මුර පදය");
        m.insert("Your password was changed", "ඔබගේ මුර පදය වෙනස් කෙරුණි");
        m.insert("Unable to change your password", "මුර පදය වෙනස් කළ නොහැකි විය");
        m.insert("Current password", "වත්මන් මුරපදය");
        m.insert("New password", "නව මුරපදය");
        m.insert("Change password", "මුරපදය වෙනස් කිරීම");
        m.insert("Email", "විද්‍යුත් තැපෑල");
        m.insert("Your email address", "ඔබගේ විද්‍යුත් තැපෑල");
        m.insert("Fill in an email address to enable password recovery", "මුරපද ප්‍රතිස්ථාපනය සඳහා විද්‍යුත් තැපැල් විස්තර ලබා දෙන්න");
        m.insert("Language", "භාෂාව");
        m.insert("Help translate", "පරිවර්ථන සහය");
        m.insert("Encryption", "ගුප්ත කේතනය");
        m.insert("Create", "තනන්න");
        m.insert("Other", "වෙනත්");
        m.insert("Username", "පරිශීලක නම");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub struct SiLK;

impl SiLK {
    pub fn get_translation(&self, key: &str) -> Option<&'static str> {
        TRANSLATIONS.get(key).copied()
    }

    pub fn get_plural_forms(&self) -> &'static str {
        &PLURAL_FORMS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translations() {
        let si_lk = SiLK;
        assert_eq!(si_lk.get_translation("Password"), Some("මුර පදය"));
        assert_eq!(si_lk.get_translation("NonExistent"), None);
    }

    #[test]
    fn test_plural_forms() {
        let si_lk = SiLK;
        assert_eq!(si_lk.get_plural_forms(), "nplurals=2; plural=(n != 1);");
    }
}