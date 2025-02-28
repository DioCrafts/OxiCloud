use askama::Template;
use actix_web::{web, HttpResponse, Result};
use crate::core::helpers::OcHelper;

#[derive(Template)]
#[template(path = "resetpassword.html")]
struct ResetPasswordTemplate<'a> {
    args: web::Query<std::collections::HashMap<String, String>>,
    success: bool,
    l: &'a dyn Translator,
}

trait Translator {
    fn t(&self, text: &str) -> String;
}

async fn reset_password(
    query: web::Query<std::collections::HashMap<String, String>>,
    success: bool,
    l: web::Data<Box<dyn Translator>>,
) -> Result<HttpResponse> {
    let template = ResetPasswordTemplate {
        args: query,
        success,
        l: l.get_ref().as_ref(),
    };
    
    let body = template.render().map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Template rendering error: {}", e))
    })?;
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

// Template HTML (resetpassword.html):
// 
// <form action="{{ OcHelper::link_to_route('core_lostpassword_reset', args) }}" method="post">
//     <fieldset>
//         {% if success %}
//             <h1>{{ l.t("Your password was reset") }}</h1>
//             <p><a href="{{ OcHelper::link_to('', 'index.php') }}/">{{ l.t("To login page") }}</a></p>
//         {% else %}
//             <p class="infield">
//                 <label for="password" class="infield">{{ l.t("New password") }}</label>
//                 <input type="password" name="password" id="password" value="" required />
//             </p>
//             <input type="submit" id="submit" value="{{ l.t("Reset password") }}" />
//         {% endif %}
//     </fieldset>
// </form>