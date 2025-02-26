use rust_i18n::i18n;

i18n!("kn", {
    "_%n folder_::_%n folders_": [""],
    "_%n file_::_%n files_": [""]
});

// Configure plural forms: "nplurals=1; plural=0;"
#[cfg(feature = "i18n")]
pub fn get_plural_form(n: usize) -> usize {
    0
}