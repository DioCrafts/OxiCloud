/**
 * Copyright (c) 2012 Bart Visscher <bartv@thisnet.nl>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */
use sha2::{Digest, Sha256};
use std::collections::HashMap;

// Definimos las dependencias que simulan el comportamiento de las clases de OC
use crate::app::AppManager;
use crate::config::Config;
use crate::defaults::Defaults;
use crate::helper::Helper;
use crate::l10n::L10N;
use crate::mail::Mailer;
use crate::preferences::Preferences;
use crate::template::Template;
use crate::user::User;
use crate::util::Util;

pub struct Controller;

impl Controller {
    fn display_lost_password_page(error: bool, requested: bool) -> Template {
        let is_encrypted = AppManager::is_enabled("files_encryption");
        Template::print_guest_page(
            "core/lostpassword",
            "lostpassword",
            HashMap::from([
                ("error", error),
                ("requested", requested),
                ("isEncrypted", is_encrypted),
            ]),
        )
    }
    
    fn display_reset_password_page(success: bool, args: &HashMap<String, String>) -> Template {
        let mut route_args = HashMap::new();
        route_args.insert("token".to_string(), args.get("token").unwrap_or(&"".to_string()).clone());
        route_args.insert("user".to_string(), args.get("user").unwrap_or(&"".to_string()).clone());
        
        Template::print_guest_page(
            "core/lostpassword",
            "resetpassword",
            HashMap::from([
                ("success", success),
                ("args", route_args),
            ]),
        )
    }

    fn check_token(user: &str, token: &str) -> bool {
        let stored_hash = Preferences::get_value(user, "owncloud", "lostpassword");
        
        match stored_hash {
            Some(hash) => {
                let token_hash = format!("{:x}", Sha256::digest(token.as_bytes()));
                hash == token_hash
            },
            None => false,
        }
    }

    pub fn index(_args: &HashMap<String, String>) -> Template {
        Self::display_lost_password_page(false, false)
    }

    pub fn send_email(_args: &HashMap<String, String>, post_data: &HashMap<String, String>) -> Template {
        let is_encrypted = AppManager::is_enabled("files_encryption");

        let continue_process = if !is_encrypted || post_data.contains_key("continue") {
            true
        } else {
            false
        };

        let user = post_data.get("user").unwrap_or(&"".to_string());

        if User::user_exists(user) && continue_process {
            let random_bytes = Util::generate_random_bytes(30);
            let password_salt = Config::get_value("passwordsalt").unwrap_or_else(|| "".to_string());
            let combined = format!("{}{}", hex::encode(&random_bytes), password_salt);
            
            let token = format!("{:x}", Sha256::digest(combined.as_bytes()));
            
            // Hash the token again to prevent timing attacks
            let token_hash = format!("{:x}", Sha256::digest(token.as_bytes()));
            Preferences::set_value(user, "owncloud", "lostpassword", &token_hash);
            
            let email = Preferences::get_value(user, "settings", "email").unwrap_or_else(|| "".to_string());
            
            if !email.is_empty() {
                let mut route_args = HashMap::new();
                route_args.insert("user".to_string(), user.clone());
                route_args.insert("token".to_string(), token);
                
                let link = Helper::link_to_route("core_lostpassword_reset", &route_args);
                let absolute_link = Helper::make_url_absolute(&link);

                let mut tmpl = Template::new("core/lostpassword", "email");
                tmpl.assign("link", absolute_link, false);
                let msg = tmpl.fetch_page();
                
                let l = L10N::get("core");
                let from = Util::get_default_email_address("lostpassword-noreply");
                
                let defaults = Defaults::new();
                match Mailer::send(
                    &email,
                    user,
                    &l.t(&format!("{} password reset", defaults.get_name())),
                    &msg,
                    &from,
                    &defaults.get_name(),
                ) {
                    Ok(_) => {
                        return Self::display_lost_password_page(false, true);
                    },
                    Err(_) => {
                        return Template::print_error_page(
                            "A problem occurs during sending the e-mail please contact your administrator."
                        );
                    }
                }
            } else {
                return Self::display_lost_password_page(true, false);
            }
        } else {
            return Self::display_lost_password_page(true, false);
        }
    }

    pub fn reset(args: &HashMap<String, String>) -> Template {
        // Someone wants to reset their password:
        let user = args.get("user").unwrap_or(&"".to_string());
        let token = args.get("token").unwrap_or(&"".to_string());
        
        if Self::check_token(user, token) {
            Self::display_reset_password_page(false, args)
        } else {
            // Someone lost their password
            Self::display_lost_password_page(false, false)
        }
    }

    pub fn reset_password(args: &HashMap<String, String>, post_data: &HashMap<String, String>) -> Template {
        let user = args.get("user").unwrap_or(&"".to_string());
        let token = args.get("token").unwrap_or(&"".to_string());
        
        if Self::check_token(user, token) {
            if let Some(password) = post_data.get("password") {
                if User::set_password(user, password) {
                    Preferences::delete_key(user, "owncloud", "lostpassword");
                    User::unset_magic_in_cookie();
                    Self::display_reset_password_page(true, args)
                } else {
                    Self::display_reset_password_page(false, args)
                }
            } else {
                Self::reset(args)
            }
        } else {
            // Someone lost their password
            Self::display_lost_password_page(false, false)
        }
    }
}