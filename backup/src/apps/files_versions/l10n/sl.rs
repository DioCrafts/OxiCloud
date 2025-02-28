use rust_i18n::t;

#[rustfmt::skip]
pub fn register_translations() {
    rust_i18n::set_translations_for!(
        "sl",
        {
            "Could not revert: %s": "Ni mogoče povrniti: %s",
            "Versions": "Različice",
            "Restore": "Obnovi"
        },
        plural_forms: "nplurals=4; plural=(n%100==1 ? 0 : n%100==2 ? 1 : n%100==3 || n%100==4 ? 2 : 3);"
    );
}