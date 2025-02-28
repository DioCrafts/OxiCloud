use rust_i18n::i18n;

i18n!("da", {
    "WebDAV Authentication": "WebDAV-godkendelse",
    "Address: ": "Adresse:",
    "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.": "Bruger oplysningerne vil blive sendt til denne adresse. Plugin'et registrerer responsen og fortolker HTTP-statuskode 401 og 403 som ugyldige oplysninger, men alle andre besvarelser som gyldige oplysninger."
});

// Define plural forms rule
#[cfg(feature = "plural-forms")]
pub fn plural_forms(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}