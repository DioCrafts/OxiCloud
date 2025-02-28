// Copyright (c) 2012 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use actix_web::{web, HttpResponse, Responder};
use std::collections::HashMap;

pub fn configure_routes(config: &mut web::ServiceConfig) {
    // Post installation check
    config.route("/post-setup-check", web::get().to(handlers::setup::post_setup_check));

    // Core ajax actions
    // Search
    config.route("/search/ajax/search.php", web::get().to(include_handler("search/ajax/search.php")));
    
    // AppConfig
    config.route("/core/ajax/appconfig.php", web::get().to(include_handler("core/ajax/appconfig.php")));
    
    // Share
    config.route("/core/ajax/share.php", web::get().to(include_handler("core/ajax/share.php")));
    
    // Translations
    config.route("/core/ajax/translations.php", web::get().to(include_handler("core/ajax/translations.php")));
    
    // Tags
    config.route("/tags/{type}", web::get().to(handlers::tags::get_tags));
    config.route("/tags/{type}/favorites", web::get().to(handlers::tags::get_favorites));
    config.route("/tags/{type}/ids", web::get().to(handlers::tags::get_ids_for_tag));
    config.route("/tags/{type}/favorite/{id}/", web::post().to(handlers::tags::favorite));
    config.route("/tags/{type}/unfavorite/{id}/", web::post().to(handlers::tags::un_favorite));
    config.route("/tags/{type}/tag/{id}/", web::post().to(handlers::tags::tag_as));
    config.route("/tags/{type}/untag/{id}/", web::post().to(handlers::tags::un_tag));
    config.route("/tags/{type}/add", web::post().to(handlers::tags::add_tag));
    config.route("/tags/{type}/delete", web::post().to(handlers::tags::delete_tags));
    
    // oC JS config
    config.route("/core/js/config.js", web::get().to(include_handler("core/js/config.php")));
    
    // Routing
    config.route("/core/routes.json", web::get().to(handlers::router::js_routes));
    config.route("/core/preview.png", web::get().to(include_handler("core/ajax/preview.php")));
    
    // Lost password
    config.route("/lostpassword/", web::get().to(handlers::lost_password::index));
    config.route("/lostpassword/", web::post().to(handlers::lost_password::send_email));
    config.route("/lostpassword/reset/{token}/{user}", web::get().to(handlers::lost_password::reset));
    config.route("/lostpassword/reset/{token}/{user}", web::post().to(handlers::lost_password::reset_password));

    // Avatar routes
    config.route("/avatar/tmp", web::get().to(handlers::avatar::get_tmp_avatar));
    config.route("/avatar/{user}/{size}", web::get().to(handlers::avatar::get_avatar));
    config.route("/avatar/", web::post().to(handlers::avatar::post_avatar));
    config.route("/avatar/", web::delete().to(handlers::avatar::delete_avatar));
    config.route("/avatar/cropped", web::post().to(handlers::avatar::post_cropped_avatar));

    // Not specifically routed
    config.route("/apps/{app}/{file:.*\\.css}", web::get().to(handlers::app::load_css_file));
    config.route("/apps/{app}/", web::get().to(handlers::app::load_app_script_file));
    config.route("/apps/{app}/{file:.*\\.php}", web::get().to(handlers::app::load_app_script_file));

    // used for heartbeat
    config.route("/heartbeat", web::get().to(|| async { HttpResponse::Ok().finish() }));
}

mod handlers {
    use actix_web::{web, HttpResponse, Responder, Result};
    use serde::{Deserialize, Serialize};

    pub mod setup {
        use super::*;
        
        pub async fn post_setup_check() -> Result<impl Responder> {
            // Implementation would call OC_Setup::post_setup_check
            Ok(HttpResponse::Ok().finish())
        }
    }

    pub mod tags {
        use super::*;
        
        #[derive(Deserialize)]
        pub struct PathParams {
            r#type: String,
            id: Option<String>,
        }

        pub async fn get_tags(path: web::Path<PathParams>) -> Result<impl Responder> {
            // Implementation would call OC\Core\Tags\Controller::get_tags
            Ok(HttpResponse::Ok().finish())
        }

        pub async fn get_favorites(path: web::Path<PathParams>) -> Result<impl Responder> {
            // Implementation
            Ok(HttpResponse::Ok().finish())
        }

        pub async fn get_ids_for_tag(path: web::Path<PathParams>) -> Result<impl Responder> {
            // Implementation
            Ok(HttpResponse::Ok().finish())
        }

        pub async fn favorite(path: web::Path<PathParams>) -> Result<impl Responder> {
            // Implementation
            Ok(HttpResponse::Ok().finish())
        }

        pub async fn un_favorite(path: web::Path<PathParams>) -> Result<impl Responder> {
            // Implementation
            Ok(HttpResponse::Ok().finish())
        }

        pub async fn tag_as(path: web::Path<PathParams>) -> Result<impl Responder> {
            // Implementation
            Ok(HttpResponse::Ok().finish())
        }

        pub async fn un_tag(path: web::Path<PathParams>) -> Result<impl Responder> {
            // Implementation
            Ok(HttpResponse::Ok().finish())
        }

        pub async fn add_tag(path: web::Path<PathParams>) -> Result<impl Responder> {
            // Implementation
            Ok(HttpResponse::Ok().finish())
        }

        pub async fn delete_tags(path: web::Path<PathParams>) -> Result<impl Responder> {
            // Implementation
            Ok(HttpResponse::Ok().finish())
        }
    }

    pub mod router {
        use super::*;
        
        pub async fn js_routes() -> Result<impl Responder> {
            // Implementation would call OC_Router::js_routes
            Ok(HttpResponse::Ok().finish())
        }
    }

    pub mod lost_password {
        use super::*;
        
        #[derive(Deserialize)]
        pub struct ResetParams {
            token: String,
            user: String,
        }

        pub async fn index() -> Result<impl Responder> {
            // Implementation
            Ok(HttpResponse::Ok().finish())
        }

        pub async fn send_email(form: web::Form<HashMap<String, String>>) -> Result<impl Responder> {
            // Implementation
            Ok(HttpResponse::Ok().finish())
        }

        pub async fn reset(path: web::Path<ResetParams>) -> Result<impl Responder> {
            // Implementation
            Ok(HttpResponse::Ok().finish())
        }

        pub async fn reset_password(
            path: web::Path<ResetParams>,
            form: web::Form<HashMap<String, String>>,
        ) -> Result<impl Responder> {
            // Implementation
            Ok(HttpResponse::Ok().finish())
        }
    }

    pub mod avatar {
        use super::*;
        
        #[derive(Deserialize)]
        pub struct AvatarParams {
            user: String,
            size: u32,
        }

        pub async fn get_tmp_avatar() -> Result<impl Responder> {
            // Implementation
            Ok(HttpResponse::Ok().finish())
        }

        pub async fn get_avatar(path: web::Path<AvatarParams>) -> Result<impl Responder> {
            // Implementation
            Ok(HttpResponse::Ok().finish())
        }

        pub async fn post_avatar(payload: web::Payload) -> Result<impl Responder> {
            // Implementation
            Ok(HttpResponse::Ok().finish())
        }

        pub async fn delete_avatar() -> Result<impl Responder> {
            // Implementation
            Ok(HttpResponse::Ok().finish())
        }

        pub async fn post_cropped_avatar(payload: web::Payload) -> Result<impl Responder> {
            // Implementation
            Ok(HttpResponse::Ok().finish())
        }
    }

    pub mod app {
        use super::*;
        
        #[derive(Deserialize)]
        pub struct AppParams {
            app: String,
            file: Option<String>,
        }

        pub async fn load_css_file(path: web::Path<AppParams>) -> Result<impl Responder> {
            // Implementation would call OC::load_css_file
            Ok(HttpResponse::Ok().finish())
        }

        pub async fn load_app_script_file(path: web::Path<AppParams>) -> Result<impl Responder> {
            // Implementation would call OC::load_app_script_file
            Ok(HttpResponse::Ok().finish())
        }
    }
}

// Helper function to handle PHP file includes
fn include_handler(file_path: &'static str) -> impl Fn() -> impl futures::Future<Output = actix_web::Result<HttpResponse>> + 'static {
    move || async move {
        // Implementation would include/process PHP file
        // In a real implementation, this would delegate to a handler that processes the PHP file
        Ok(HttpResponse::Ok().body(format!("Included: {}", file_path)))
    }
}