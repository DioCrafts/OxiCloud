/*
 * Copyright (c) 2013, Bjoern Schiessle <schiessle@owncloud.com>
 * This file is licensed under the Affero General Public License version 3 or later.
 * See the COPYING-README file.
 *
 * Script to change recovery key password
 */

use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::apps::files_encryption::crypt;
use crate::apps::files_encryption::util::Util;
use crate::core::l10n::L10n;
use crate::files::view::{FilesystemView, View};
use crate::user;
use crate::ocs::json;
use crate::middleware::{check_admin_user, check_app_enabled, check_call};
use crate::files::proxy::FileProxy;

#[derive(Error, Debug)]
pub enum ChangeRecoveryPasswordError {
    #[error("Could not change the password. Maybe the old password was not correct.")]
    IncorrectPassword,
    #[error("File system error: {0}")]
    FileSystemError(String),
    #[error("Encryption error: {0}")]
    EncryptionError(String),
}

#[derive(Deserialize)]
pub struct ChangeRecoveryPasswordRequest {
    old_password: String,
    new_password: String,
}

#[derive(Serialize)]
pub struct ChangeRecoveryPasswordResponse {
    message: String,
}

pub async fn change_recovery_password(
    request: web::Json<ChangeRecoveryPasswordRequest>,
    l10n: web::Data<L10n>,
) -> Result<HttpResponse, ChangeRecoveryPasswordError> {
    check_admin_user()?;
    check_app_enabled("files_encryption")?;
    check_call()?;

    let view = View::new("/");
    let util = Util::new(FilesystemView::new("/"), user::get_user());

    let proxy_status = FileProxy::is_enabled();
    FileProxy::set_enabled(false);

    let key_id = util.get_recovery_key_id();
    let key_path = format!("/owncloud_private_key/{}.private.key", key_id);

    let encrypted_recovery_key = view.file_get_contents(&key_path)
        .map_err(|e| ChangeRecoveryPasswordError::FileSystemError(e.to_string()))?;
    
    let decrypted_recovery_key = crypt::decrypt_private_key(&encrypted_recovery_key, &request.old_password)
        .ok_or(ChangeRecoveryPasswordError::IncorrectPassword)?;

    let encrypted_recovery_key = crypt::symmetric_encrypt_file_content(&decrypted_recovery_key, &request.new_password)
        .map_err(|e| ChangeRecoveryPasswordError::EncryptionError(e.to_string()))?;
    
    view.file_put_contents(&key_path, encrypted_recovery_key)
        .map_err(|e| ChangeRecoveryPasswordError::FileSystemError(e.to_string()))?;

    FileProxy::set_enabled(proxy_status);

    let response = ChangeRecoveryPasswordResponse {
        message: l10n.t("Password successfully changed."),
    };

    Ok(json::success(response))
}