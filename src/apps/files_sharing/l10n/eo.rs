use std::collections::HashMap;
use rust_i18n::t;

/// Esperanto (eo) translation file
pub fn register_translations() -> (HashMap<String, String>, String) {
    let mut translations = HashMap::new();
    
    translations.insert(
        "This share is password-protected".to_string(),
        "Ĉi tiu kunhavigo estas protektata per pasvorto".to_string(),
    );
    translations.insert(
        "Password".to_string(),
        "Pasvorto".to_string(),
    );
    translations.insert(
        "Sorry, this link doesn't seem to work anymore.".to_string(),
        "Pardonu, ĉi tiu ligilo ŝajne ne plu funkcias.".to_string(),
    );
    translations.insert(
        "Reasons might be:".to_string(),
        "Kialoj povas esti:".to_string(),
    );
    translations.insert(
        "the item was removed".to_string(),
        "la ero foriĝis".to_string(),
    );
    translations.insert(
        "the link expired".to_string(),
        "la ligilo eksvalidiĝis".to_string(),
    );
    translations.insert(
        "sharing is disabled".to_string(),
        "kunhavigo malkapablas".to_string(),
    );
    translations.insert(
        "For more info, please ask the person who sent this link.".to_string(),
        "Por plia informo, bonvolu peti al la persono, kiu sendis ĉi tiun ligilon.".to_string(),
    );
    translations.insert(
        "%s shared the folder %s with you".to_string(),
        "%s kunhavigis la dosierujon %s kun vi".to_string(),
    );
    translations.insert(
        "%s shared the file %s with you".to_string(),
        "%s kunhavigis la dosieron %s kun vi".to_string(),
    );
    translations.insert(
        "Download".to_string(),
        "Elŝuti".to_string(),
    );
    translations.insert(
        "Upload".to_string(),
        "Alŝuti".to_string(),
    );
    translations.insert(
        "Cancel upload".to_string(),
        "Nuligi alŝuton".to_string(),
    );
    translations.insert(
        "No preview available for".to_string(),
        "Ne haveblas antaŭvido por".to_string(),
    );
    translations.insert(
        "Direct link".to_string(),
        "Direkta ligilo".to_string(),
    );

    let plural_forms = "nplurals=2; plural=(n != 1);".to_string();
    
    (translations, plural_forms)
}