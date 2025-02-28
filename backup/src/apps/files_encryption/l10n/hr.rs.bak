use rust_i18n::i18n;

i18n!("hr", {
    "Saving..." => "Spremanje...",
});

// Define plural forms for Croatian
// nplurals=3; plural=n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2;
#[allow(dead_code)]
fn plural_form(n: usize) -> usize {
    if n % 10 == 1 && n % 100 != 11 {
        0
    } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
        1
    } else {
        2
    }
}