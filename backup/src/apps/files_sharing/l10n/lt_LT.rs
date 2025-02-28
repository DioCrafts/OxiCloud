use rust_i18n::t;

pub fn register_translations() {
    // Lithuanian (Lithuania) translations
    rust_i18n::set_locale("lt_LT");

    rust_i18n::translations! {
        "lt_LT" => {
            "This share is password-protected" => "Turinys apsaugotas slaptažodžiu",
            "The password is wrong. Try again." => "Netinka slaptažodis: Bandykite dar kartą.",
            "Password" => "Slaptažodis",
            "Sorry, this link doesn't seem to work anymore." => "Atleiskite, panašu, kad nuoroda yra neveiksni.",
            "Reasons might be:" => "Galimos priežastys:",
            "the item was removed" => "elementas buvo pašalintas",
            "the link expired" => "baigėsi nuorodos galiojimo laikas",
            "sharing is disabled" => "dalinimasis yra išjungtas",
            "For more info, please ask the person who sent this link." => "Dėl tikslesnės informacijos susisiekite su asmeniu atsiuntusiu nuorodą.",
            "%s shared the folder %s with you" => "%s pasidalino su jumis %s aplanku",
            "%s shared the file %s with you" => "%s pasidalino su jumis %s failu",
            "Download" => "Atsisiųsti",
            "Upload" => "Įkelti",
            "Cancel upload" => "Atšaukti siuntimą",
            "No preview available for" => "Peržiūra nėra galima",
            "Direct link" => "Tiesioginė nuoroda",
        }
    };

    // Configure plural forms for Lithuanian
    rust_i18n::set_plural_rule("lt_LT", |n| {
        if n % 10 == 1 && n % 100 != 11 {
            0
        } else if n % 10 >= 2 && (n % 100 < 10 || n % 100 >= 20) {
            1
        } else {
            2
        }
    });
}