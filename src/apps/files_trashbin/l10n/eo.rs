use rust_i18n::t;

pub fn register_translations() {
    rust_i18n::set_locale("eo");

    rust_i18n::add_translation("eo", "Couldn't delete %s permanently", "Ne povis foriĝi %s por ĉiam");
    rust_i18n::add_translation("eo", "Couldn't restore %s", "Ne povis restaŭriĝi %s");
    rust_i18n::add_translation("eo", "Error", "Eraro");
    rust_i18n::add_translation("eo", "restored", "restaŭrita");
    rust_i18n::add_translation("eo", "Nothing in here. Your trash bin is empty!", "Nenio estas ĉi tie. Via rubujo malplenas!");
    rust_i18n::add_translation("eo", "Name", "Nomo");
    rust_i18n::add_translation("eo", "Restore", "Restaŭri");
    rust_i18n::add_translation("eo", "Deleted", "Forigita");
    rust_i18n::add_translation("eo", "Delete", "Forigi");
    rust_i18n::add_translation("eo", "Deleted Files", "Forigitaj dosieroj");

    rust_i18n::set_plural_rule("eo", |n: f64| -> usize {
        if n != 1.0 { 1 } else { 0 }
    });
}