#[derive(Default)]
pub struct L10n {
    translations: std::collections::HashMap<String, Vec<String>>,
    plural_forms: String,
}

impl L10n {
    pub fn new() -> Self {
        let mut l10n = L10n {
            translations: std::collections::HashMap::new(),
            plural_forms: "nplurals=3; plural=(n==1) ? 0 : (n>=2 && n<=4) ? 1 : 2;".to_string(),
        };

        l10n.translations.insert(
            "_%s group found_::_%s groups found_".to_string(),
            vec!["".to_string(), "".to_string(), "".to_string()],
        );

        l10n.translations.insert(
            "_%s user found_::_%s users found_".to_string(),
            vec!["".to_string(), "".to_string(), "".to_string()],
        );

        l10n
    }

    pub fn get_translation(&self, key: &str, count: usize) -> Option<&String> {
        self.translations.get(key).and_then(|forms| {
            let plural_index = self.get_plural_index(count);
            forms.get(plural_index)
        })
    }

    fn get_plural_index(&self, n: usize) -> usize {
        // Implementación de la fórmula de pluralización para eslovaco
        if n == 1 {
            0
        } else if n >= 2 && n <= 4 {
            1
        } else {
            2
        }
    }
}