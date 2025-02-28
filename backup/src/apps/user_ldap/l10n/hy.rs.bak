use rust_i18n::i18n;

i18n!("hy");

#[export_i18n_keys]
pub fn exported_hy_strings() {
    i18n!("_{count} group found_::_{count} groups found_", count = 1);
    i18n!("_{count} user found_::_{count} users found_", count = 1);
    i18n!("Save");
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}