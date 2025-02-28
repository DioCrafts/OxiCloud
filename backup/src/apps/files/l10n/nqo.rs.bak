use phf::phf_map;

// Define plural forms expression for nqo locale
pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";

// Define translations using static phf_map
pub static TRANSLATIONS: phf::Map<&'static str, &'static [&'static str]> = phf_map! {
    "_%n folder_::_%n folders_" => &[""],
    "_%n file_::_%n files_" => &[""],
    "_Uploading %n file_::_Uploading %n files_" => &[""],
};