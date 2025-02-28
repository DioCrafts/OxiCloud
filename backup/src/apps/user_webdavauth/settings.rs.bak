use std::error::Error;
use rocket::{State, post, get, request::Form, response::content::Html};
use rocket_csrf::{CsrfToken, CsrfConfig};
use owncloud_core::{
    config::Config,
    auth::AdminChecker,
    template::Template,
};

/**
 * ownCloud - user_webdavauth
 *
 * @author Frank Karlitschek
 * @copyright 2012 Frank Karlitschek frank@owncloud.org
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU AFFERO GENERAL PUBLIC LICENSE for more details.
 *
 * You should have received a copy of the GNU Affero General Public
 * License along with this library.  If not, see <http://www.gnu.org/licenses/>.
 */

#[derive(FromForm)]
struct WebdavSettings {
    webdav_url: Option<String>,
    csrf_token: CsrfToken,
}

#[post("/settings", data = "<form>")]
async fn handle_settings_post(
    form: Form<WebdavSettings>,
    admin_checker: AdminChecker,
    config: State<Config>,
    csrf_config: State<CsrfConfig>,
) -> Result<Html<String>, Box<dyn Error>> {
    // Check if user is admin
    admin_checker.check_admin()?;
    
    // CSRF check
    csrf_config.verify_token(&form.csrf_token)?;
    
    // Update configuration if webdav_url is provided
    if let Some(webdav_url) = &form.webdav_url {
        // Strip tags from input (similar to PHP's strip_tags)
        let sanitized_url = sanitize_html(webdav_url);
        config.set_value("user_webdavauth_url", sanitized_url).await?;
    }
    
    // Return the settings page
    render_settings_page(config).await
}

#[get("/settings")]
async fn settings_page(
    admin_checker: AdminChecker,
    config: State<Config>,
) -> Result<Html<String>, Box<dyn Error>> {
    // Check if user is admin
    admin_checker.check_admin()?;
    
    // Render the settings page
    render_settings_page(config).await
}

async fn render_settings_page(config: State<Config>) -> Result<Html<String>, Box<dyn Error>> {
    let webdav_url = config.get_value::<String>("user_webdavauth_url").await?;
    
    let mut template = Template::new("user_webdavauth", "settings");
    template.assign("webdav_url", webdav_url);
    
    let page_content = template.fetch_page().await?;
    Ok(Html(page_content))
}

fn sanitize_html(input: &str) -> String {
    // Simple implementation of strip_tags
    // In production, use a proper HTML sanitization library
    input.replace('<', "&lt;").replace('>', "&gt;")
}

// Register routes
pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![settings_page, handle_settings_post]
}