use std::collections::HashMap;

// Equivalente a traits/interfaces para Theme y Translator
pub trait Theme {
    fn get_name(&self) -> String;
    fn get_slogan(&self) -> String;
    fn get_base_url(&self) -> String;
}

pub trait Translator {
    fn t(&self, text: &str, vars: Option<Vec<String>>) -> String;
}

pub struct OCHelper;

impl OCHelper {
    pub fn make_url_absolute(path: &str) -> String {
        // Implementación para hacer la URL absoluta
        format!("https://example.com/{}", path)
    }
}

pub fn image_path(_app: &str, image: &str) -> String {
    // Implementación para obtener la ruta de la imagen
    format!("images/{}", image)
}

pub fn render_mail_template(
    theme: &impl Theme, 
    translator: &impl Translator, 
    params: HashMap<String, String>
) -> String {
    let user_displayname = params.get("user_displayname").cloned().unwrap_or_default();
    let filename = params.get("filename").cloned().unwrap_or_default();
    let link = params.get("link").cloned().unwrap_or_default();
    let expiration = params.get("expiration").cloned();
    
    let logo_path = OCHelper::make_url_absolute(&image_path("", "logo-mail.gif"));
    
    let mut html = format!(r#"<table cellspacing="0" cellpadding="0" border="0" width="100%">
<tr><td>
<table cellspacing="0" cellpadding="0" border="0" width="600px">
<tr>
<td bgcolor="#1d2d44" width="20px">&nbsp;</td>
<td bgcolor="#1d2d44">
<img src="{}" alt="{}"/>
</td>
</tr>
<tr><td bgcolor="#f8f8f8" colspan="2">&nbsp;</td></tr>
<tr>
<td bgcolor="#f8f8f8" width="20px">&nbsp;</td>
<td bgcolor="#f8f8f8" style="font-weight:normal; font-size:0.8em; line-height:1.2em; font-family:verdana,'arial',sans;">
"#, logo_path, theme.get_name());

    // Añadir mensaje principal
    let message = translator.t(
        "Hey there,<br><br>just letting you know that %s shared »%s« with you.<br><a href=\"%s\">View it!</a><br><br>",
        Some(vec![user_displayname, filename, link]),
    );
    html.push_str(&message);

    // Añadir mensaje de expiración si existe
    if let Some(exp) = expiration {
        let expiration_msg = translator.t(
            "The share will expire on %s.<br><br>",
            Some(vec![exp]),
        );
        html.push_str(&expiration_msg);
    }

    // Añadir despedida
    let cheers = translator.t("Cheers!", None);
    html.push_str(&cheers);

    // Completar la plantilla
    html.push_str(&format!(r#"
</td>
</tr>
<tr><td bgcolor="#f8f8f8" colspan="2">&nbsp;</td></tr>
<tr>
<td bgcolor="#f8f8f8" width="20px">&nbsp;</td>
<td bgcolor="#f8f8f8" style="font-weight:normal; font-size:0.8em; line-height:1.2em; font-family:verdana,'arial',sans;">--<br>
{} -
{}
<br><a href="{}">{}</a>
</td>
</tr>
<tr>
<td bgcolor="#f8f8f8" colspan="2">&nbsp;</td>
</tr>
</table>
</td></tr>
</table>
"#, theme.get_name(), theme.get_slogan(), theme.get_base_url(), theme.get_base_url()));

    html
}