lazy_static! {
    pub static ref TRANSLATIONS: phf::Map<&'static str, Vec<&'static str>> = phf::phf_map! {
        "_%n folder_::_%n folders_" => vec![""],
        "_%n file_::_%n files_" => vec![""],
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}