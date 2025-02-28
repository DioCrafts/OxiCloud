// Script to change recovery key password
//
// Copyright (c) 2013, Bjoern Schiessle <schiessle@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or later.
// See the COPYING-README file.

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::apps::encryption::{Crypt, Session};
use crate::core::file_system::{FileProxy, View};
use crate::core::l10n::L10n;
use crate::core::user::User;
use crate::core::json_response::{error_response, success_response};
use crate::core::app::check_app_enabled;
use crate::core::auth::check_logged_in;

#[derive(Error, Debug)]
pub enum UpdatePrivateKeyError {
    #[error("User not logged in")]
    NotLoggedIn,
    
    #[error("App not enabled")]
    AppNotEnabled,
    
    #[error("Invalid request")]
    InvalidRequest,
    
    #[error("Failed to update private key password")]
    UpdateFailed,
    
    #[error("File system error: {0}")]
    FileSystemError(String),
}

#[derive(Deserialize)]
pub struct UpdatePrivateKeyRequest {
    old_password: String,
    new_password: String,
}

#[derive(Serialize)]
pub struct UpdatePrivateKeyResponse {
    message: String,
}

/// Updates the private key password for the current user
pub async fn update_private_key_password(
    request: web::Json<UpdatePrivateKeyRequest>,
    l10n: web::Data<L10n>,
) -> Result<HttpResponse, UpdatePrivateKeyError> {
    check_logged_in()?;
    check_app_enabled("files_encryption")?;
    
    let old_password = &request.old_password;
    let new_password = &request.new_password;
    
    let view = View::new("/");
    let mut session = Session::new(&view);
    let user = User::get_current_user().ok_or(UpdatePrivateKeyError::NotLoggedIn)?;
    
    let proxy_status = FileProxy::is_enabled();
    FileProxy::set_enabled(false);
    
    let key_path = format!("/{}/files_encryption/{}.private.key", user, user);
    
    let result = (|| {
        let encrypted_key = view.file_get_contents(&key_path)
            .map_err(|e| UpdatePrivateKeyError::FileSystemError(e.to_string()))?;
            
        let decrypted_key = Crypt::decrypt_private_key(&encrypted_key, old_password)
            .ok_or(UpdatePrivateKeyError::UpdateFailed)?;
        
        let encrypted_key = Crypt::symmetric_encrypt_file_content(&decrypted_key, new_password)
            .map_err(|e| UpdatePrivateKeyError::FileSystemError(e.to_string()))?;
            
        view.file_put_contents(&key_path, &encrypted_key)
            .map_err(|e| UpdatePrivateKeyError::FileSystemError(e.to_string()))?;
        
        session.set_private_key(&decrypted_key);
        
        Ok(())
    })();
    
    FileProxy::set_enabled(proxy_status);
    
    match result {
        Ok(_) => {
            session.set_initialized(Session::INIT_SUCCESSFUL);
            let message = l10n.t("Private key password successfully updated.");
            Ok(success_response(UpdatePrivateKeyResponse { message }))
        },
        Err(_) => {
            let message = l10n.t("Could not update the private key password. Maybe the old password was not correct.");
            Ok(error_response(UpdatePrivateKeyResponse { message }))
        }
    }
}