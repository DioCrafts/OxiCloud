use phf::phf_map;
use rust_i18n::translation_hashmap;

// Translations for my_MM (Burmese - Myanmar)
pub static TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "Help" => "အကူအညီ",
    "Users" => "သုံးစွဲသူ",
    "Admin" => "အက်ဒမင်",
    "web services under your control" => "သင်၏ထိန်းချုပ်မှု့အောက်တွင်ရှိသော Web services",
    "ZIP download is turned off." => "ZIP ဒေါင်းလုတ်ကိုပိတ်ထားသည်",
    "Files need to be downloaded one by one." => "ဖိုင်များသည် တစ်ခုပြီး တစ်ခုဒေါင်းလုတ်ချရန်လိုအပ်သည်",
    "Back to Files" => "ဖိုင်သို့ပြန်သွားမည်",
    "Selected files too large to generate zip file." => "zip ဖိုင်အဖြစ်ပြုလုပ်ရန် ရွေးချယ်ထားသောဖိုင်များသည် အရမ်းကြီးလွန်းသည်",
    "Authentication error" => "ခွင့်ပြုချက်မအောင်မြင်",
    "Files" => "ဖိုင်များ",
    "Text" => "စာသား",
    "Images" => "ပုံရိပ်များ",
    "Could not find category \"%s\"" => "\"%s\"ခေါင်းစဉ်ကို ရှာမတွေ့ပါ",
    "seconds ago" => "စက္ကန့်အနည်းငယ်က",
    "_%n minute ago_::_%n minutes ago_" => "စက္ကန့်အနည်းငယ်က",
    "_%n hour ago_::_%n hours ago_" => "စက္ကန့်အနည်းငယ်က",
    "today" => "ယနေ့",
    "yesterday" => "မနေ့က",
    "_%n day go_::_%n days ago_" => "စက္ကန့်အနည်းငယ်က",
    "last month" => "ပြီးခဲ့သောလ",
    "_%n month ago_::_%n months ago_" => "စက္ကန့်အနည်းငယ်က",
    "last year" => "မနှစ်က",
    "years ago" => "နှစ် အရင်က",
};

// Plural form for my_MM (Burmese)
pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";

pub fn get_plural_index(n: usize) -> usize {
    0 // Burmese has only one plural form
}

pub fn init() -> translation_hashmap::TranslationBundle {
    let mut bundle = translation_hashmap::TranslationBundle::new();
    for (key, value) in TRANSLATIONS.entries() {
        bundle.insert(key.to_string(), value.to_string());
    }
    bundle
}