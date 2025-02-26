use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    [
        ("Password successfully changed.", "La pasvorto sukcese ŝanĝiĝis."),
        ("Could not change the password. Maybe the old password was not correct.", "Ne eblis ŝanĝi la pasvorton. Eble la malnova pasvorto malĝustis."),
        ("Private key password successfully updated.", "La pasvorto de la malpublika klavo sukcese ĝisdatiĝis."),
        ("Missing requirements.", "Mankas neproj."),
        ("Saving...", "Konservante..."),
        ("Go directly to your ", "Iri direkte al via"),
        ("personal settings", "persona agordo"),
        ("Encryption", "Ĉifrado"),
        ("Enabled", "Kapabligita"),
        ("Disabled", "Malkapabligita"),
        ("Change Password", "Ŝarĝi pasvorton"),
        ("Your private key password no longer match your log-in password:", "La pasvorto de via malpublika klavo ne plu kongruas kun via ensaluta pasvorto:"),
        ("Old log-in password", "Malnova ensaluta pasvorto"),
        ("Current log-in password", "Nuna ensaluta pasvorto"),
        ("Update Private Key Password", "Ĝisdatigi la pasvorton de la malpublika klavo"),
    ]
    .iter()
    .cloned()
    .collect()
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";