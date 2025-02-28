use std::collections::HashMap;
use rust_i18n::translation_hashmap;

pub fn translations() -> HashMap<&'static str, &'static str> {
    translation_hashmap! {
        "Sunday" => "ఆదివారం",
        "Monday" => "సోమవారం",
        "Tuesday" => "మంగళవారం",
        "Wednesday" => "బుధవారం",
        "Thursday" => "గురువారం",
        "Friday" => "శుక్రవారం",
        "Saturday" => "శనివారం",
        "January" => "జనవరి",
        "February" => "ఫిబ్రవరి",
        "March" => "మార్చి",
        "April" => "ఏప్రిల్",
        "May" => "మే",
        "June" => "జూన్",
        "July" => "జూలై",
        "August" => "ఆగస్ట్",
        "September" => "సెప్టెంబర్",
        "October" => "అక్టోబర్",
        "November" => "నవంబర్",
        "December" => "డిసెంబర్",
        "Settings" => "అమరికలు",
        "seconds ago" => "క్షణాల క్రితం",
        "today" => "ఈరోజు",
        "yesterday" => "నిన్న",
        "last month" => "పోయిన నెల",
        "months ago" => "నెలల క్రితం",
        "last year" => "పోయిన సంవత్సరం",
        "years ago" => "సంవత్సరాల క్రితం",
        "Yes" => "అవును",
        "No" => "కాదు",
        "Ok" => "సరే",
        "Cancel" => "రద్దుచేయి",
        "Error" => "పొరపాటు",
        "Password" => "సంకేతపదం",
        "Send" => "పంపించు",
        "Expiration date" => "కాలం చెల్లు తేదీ",
        "delete" => "తొలగించు",
        "Delete" => "తొలగించు",
        "Add" => "చేర్చు",
        "Username" => "వాడుకరి పేరు",
        "New password" => "కొత్త సంకేతపదం",
        "Users" => "వాడుకరులు",
        "Help" => "సహాయం",
        "Log out" => "నిష్క్రమించు",
        "Lost your password?" => "మీ సంకేతపదం పోయిందా?"
    }
}

pub fn plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn get_plural_translations() -> HashMap<&'static str, Vec<&'static str>> {
    let mut plurals = HashMap::new();
    
    plurals.insert("_%n minute ago_::_%n minutes ago_", vec!["", ""]);
    plurals.insert("_%n hour ago_::_%n hours ago_", vec!["", ""]);
    plurals.insert("_%n day ago_::_%n days ago_", vec!["", ""]);
    plurals.insert("_%n month ago_::_%n months ago_", vec!["", ""]);
    plurals.insert("_{count} file conflict_::_{count} file conflicts_", vec!["", ""]);
    
    plurals
}