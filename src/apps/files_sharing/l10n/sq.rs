use rust_i18n::i18n;

i18n!("sq");

pub fn get_translations() -> rust_i18n::Translations {
    let mut translations = rust_i18n::Translations::new();
    
    translations.insert(
        "The password is wrong. Try again.",
        "Kodi është i gabuar. Provojeni përsëri.",
    );
    translations.insert("Password", "Kodi");
    translations.insert(
        "Sorry, this link doesn't seem to work anymore.",
        "Ju kërkojmë ndjesë, kjo lidhje duket sikur nuk punon më.",
    );
    translations.insert("Reasons might be:", "Arsyet mund të jenë:");
    translations.insert("the item was removed", "elementi është eliminuar");
    translations.insert("the link expired", "lidhja ka skaduar");
    translations.insert("sharing is disabled", "ndarja është çaktivizuar");
    translations.insert(
        "For more info, please ask the person who sent this link.",
        "Për më shumë informacione, ju lutem pyesni personin që iu dërgoi këtë lidhje.",
    );
    translations.insert(
        "%s shared the folder %s with you",
        "%s ndau me ju dosjen %s",
    );
    translations.insert(
        "%s shared the file %s with you",
        "%s ndau me ju skedarin %s",
    );
    translations.insert("Download", "Shkarko");
    translations.insert("Upload", "Ngarko");
    translations.insert("Cancel upload", "Anulo ngarkimin");
    translations.insert(
        "No preview available for",
        "Shikimi paraprak nuk është i mundur për",
    );
    
    translations.set_plural_form("nplurals=2; plural=(n != 1);");
    
    translations
}

pub fn init() {
    rust_i18n::set_locale("sq");
    rust_i18n::add_translations(get_translations());
}