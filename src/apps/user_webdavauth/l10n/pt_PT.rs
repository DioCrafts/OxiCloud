use rust_i18n::t;

// Portuguese (Portugal) language file for user_webdavauth
rust_i18n::set_translator!(| locale: &str, message: &str, args: &[(&str, &str)]| {
    let translations = match locale {
        "pt_PT" => {
            match message {
                "WebDAV Authentication" => "Autenticação WebDAV",
                "Address: " => "Endereço:",
                "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials." => 
                    "As credenciais do utilizador vão ser enviadas para endereço URL. Este plugin verifica a resposta e vai interpretar os códigos de estado HTTP 401 e 403 como credenciais inválidas, e todas as outras respostas como válidas.",
                _ => message,
            }
        },
        _ => message,
    };

    let mut result = translations.to_string();
    for (key, value) in args {
        result = result.replace(&format!("{{{}}}", key), value);
    }
    result
});

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}