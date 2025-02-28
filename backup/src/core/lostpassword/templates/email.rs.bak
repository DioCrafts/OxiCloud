use rocket::response::content;
use rocket_i18n::I18n;

#[get("/email")]
pub fn email(link: String, i18n: I18n) -> content::Html<String> {
    let template = i18n.t("Use the following link to reset your password: {link}");
    let email_content = template.replace("{link}", &link);
    
    content::Html(email_content)
}