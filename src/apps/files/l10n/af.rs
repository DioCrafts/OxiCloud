use rust_i18n::i18n;

i18n!("af");

#[rust_i18n::locale("af")]
pub fn plural_forms(n: usize) -> usize {
    if n != 1 {
        1
    } else {
        0
    }
}

#[rustfmt::skip]
pub fn translations() -> rust_i18n::Translations {
    let mut t = rust_i18n::Translations::new();
    t.add("_%n folder_::_%n folders_", vec!["", ""]);
    t.add("_%n file_::_%n files_", vec!["", ""]);
    t.add("_Uploading %n file_::_Uploading %n files_", vec!["", ""]);
    t
}