use rust_i18n::t;

pub fn init_sv_translation() -> Result<(), Box<dyn std::error::Error>> {
    let translations = maplit::hashmap! {
        "Couldn't delete %s permanently".to_string() => "Kunde inte radera %s permanent".to_string(),
        "Couldn't restore %s".to_string() => "Kunde inte återställa %s".to_string(),
        "Error".to_string() => "Fel".to_string(),
        "restored".to_string() => "återställd".to_string(),
        "Nothing in here. Your trash bin is empty!".to_string() => "Ingenting här. Din papperskorg är tom!".to_string(),
        "Name".to_string() => "Namn".to_string(),
        "Restore".to_string() => "Återskapa".to_string(),
        "Deleted".to_string() => "Raderad".to_string(),
        "Delete".to_string() => "Radera".to_string(),
        "Deleted Files".to_string() => "Raderade filer".to_string(),
    };

    // Register translations
    for (key, value) in translations {
        rust_i18n::set_translation("sv", &key, &value)?;
    }

    // Set plural forms
    rust_i18n::set_plural_rule("sv", |n| if n != 1 { 1 } else { 0 })?;

    Ok(())
}