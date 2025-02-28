use rust_i18n::t;

pub fn init_translations() -> rust_i18n::Translations {
    let mut translations = rust_i18n::Translations::new();
    
    translations.insert("Couldn't delete %s permanently", "Impossibile eliminare %s definitivamente");
    translations.insert("Couldn't restore %s", "Impossibile ripristinare %s");
    translations.insert("Error", "Errore");
    translations.insert("restored", "ripristinati");
    translations.insert("Nothing in here. Your trash bin is empty!", "Qui non c'è niente. Il tuo cestino è vuoto.");
    translations.insert("Name", "Nome");
    translations.insert("Restore", "Ripristina");
    translations.insert("Deleted", "Eliminati");
    translations.insert("Delete", "Elimina");
    translations.insert("Deleted Files", "File eliminati");
    
    translations.set_plural_form("nplurals=2; plural=(n != 1);");
    
    translations
}