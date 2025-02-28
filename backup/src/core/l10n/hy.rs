use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Sunday", "Կիրակի");
        m.insert("Monday", "Երկուշաբթի");
        m.insert("Tuesday", "Երեքշաբթի");
        m.insert("Wednesday", "Չորեքշաբթի");
        m.insert("Thursday", "Հինգշաբթի");
        m.insert("Friday", "Ուրբաթ");
        m.insert("Saturday", "Շաբաթ");
        m.insert("January", "Հունվար");
        m.insert("February", "Փետրվար");
        m.insert("March", "Մարտ");
        m.insert("April", "Ապրիլ");
        m.insert("May", "Մայիս");
        m.insert("June", "Հունիս");
        m.insert("July", "Հուլիս");
        m.insert("August", "Օգոստոս");
        m.insert("September", "Սեպտեմբեր");
        m.insert("October", "Հոկտեմբեր");
        m.insert("November", "Նոյեմբեր");
        m.insert("December", "Դեկտեմբեր");
        m.insert("Delete", "Ջնջել");
        m
    };

    pub static ref PLURAL_TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n minute ago_::_%n minutes ago_", vec!["", ""]);
        m.insert("_%n hour ago_::_%n hours ago_", vec!["", ""]);
        m.insert("_%n day ago_::_%n days ago_", vec!["", ""]);
        m.insert("_%n month ago_::_%n months ago_", vec!["", ""]);
        m.insert("_{count} file conflict_::_{count} file conflicts_", vec!["", ""]);
        m
    };
}

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_translation(key: &str, count: usize) -> Option<&'static str> {
    let plural_index = if count != 1 { 1 } else { 0 };
    PLURAL_TRANSLATIONS.get(key).and_then(|forms| forms.get(plural_index).copied())
}