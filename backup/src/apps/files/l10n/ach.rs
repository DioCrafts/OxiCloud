use rust_i18n::locale::Locale;

pub fn get_ach_translations() -> Locale {
    let mut locale = Locale::new("ach");
    
    locale.add_plural_form("nplurals=2; plural=(n > 1);");
    
    locale.add_translation("_%n folder_::_%n folders_", vec!["", ""]);
    locale.add_translation("_%n file_::_%n files_", vec!["", ""]);
    locale.add_translation("_Uploading %n file_::_Uploading %n files_", vec!["", ""]);
    
    locale
}