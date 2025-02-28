/**
 * Copyright (c) 2013 Christopher Schäpers <christopher@schaepers.it>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use actix_web::{web, HttpResponse, Error};
use futures::StreamExt;
use image::{self, GenericImageView, ImageFormat};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Avatar-related errors
#[derive(Error, Debug)]
pub enum AvatarError {
    #[error("Image is not square")]
    NotSquareError,
    #[error("Unknown filetype")]
    UnknownFiletype,
    #[error("Invalid image")]
    InvalidImage,
    #[error("No image or file provided")]
    NoImageProvided,
    #[error("No temporary profile picture available, try again")]
    NoTempAvatar,
    #[error("No crop data provided")]
    NoCropData,
    #[error("File system error: {0}")]
    FileSystemError(#[from] std::io::Error),
    #[error("Image processing error: {0}")]
    ImageError(#[from] image::ImageError),
    #[error("Cache error: {0}")]
    CacheError(String),
    #[error("User not found: {0}")]
    UserNotFound(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Internal server error: {0}")]
    InternalError(String),
}

pub type AvatarResult<T> = Result<T, AvatarError>;

/// Response for JSON data
#[derive(Serialize)]
struct JsonResponse<T> {
    data: T,
}

/// Display name response
#[derive(Serialize)]
struct DisplayNameData {
    displayname: String,
}

/// Error message response
#[derive(Serialize)]
struct ErrorMessage {
    message: String,
}

/// Crop data for avatar cropping
#[derive(Deserialize)]
struct CropData {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

/// Form data for avatar upload with path
#[derive(Deserialize)]
struct AvatarPathForm {
    path: String,
}

/// User avatar service
pub struct Avatar {
    user: String,
    avatar_storage: Box<dyn AvatarStorage>,
    user_service: Box<dyn UserService>,
    file_system: Box<dyn FileSystemService>,
    cache_service: Box<dyn CacheService>,
    l10n_service: Box<dyn L10nService>,
}

/// Avatar storage interface
pub trait AvatarStorage: Send + Sync {
    fn get(&self, user: &str, size: u32) -> AvatarResult<Option<Vec<u8>>>;
    fn set(&self, user: &str, data: &[u8]) -> AvatarResult<()>;
    fn remove(&self, user: &str) -> AvatarResult<()>;
}

/// User service interface
pub trait UserService: Send + Sync {
    fn get_user(&self) -> AvatarResult<String>;
    fn get_display_name(&self, user: &str) -> AvatarResult<String>;
    fn check_logged_in(&self) -> AvatarResult<()>;
    fn check_csrf(&self) -> AvatarResult<()>;
}

/// File system service interface
pub trait FileSystemService: Send + Sync {
    fn get_file_contents(&self, user: &str, path: &str) -> AvatarResult<Vec<u8>>;
    fn is_file_blacklisted(&self, path: &Path) -> bool;
}

/// Cache service interface
pub trait CacheService: Send + Sync {
    fn set(&self, key: &str, data: Vec<u8>, timeout: u64) -> AvatarResult<()>;
    fn get(&self, key: &str) -> AvatarResult<Option<Vec<u8>>>;
    fn remove(&self, key: &str) -> AvatarResult<()>;
}

/// Localization service interface
pub trait L10nService: Send + Sync {
    fn translate(&self, text: &str) -> String;
}

impl Avatar {
    pub fn new(
        user: String,
        avatar_storage: Box<dyn AvatarStorage>,
        user_service: Box<dyn UserService>,
        file_system: Box<dyn FileSystemService>,
        cache_service: Box<dyn CacheService>,
        l10n_service: Box<dyn L10nService>,
    ) -> Self {
        Self {
            user,
            avatar_storage,
            user_service,
            file_system,
            cache_service,
            l10n_service,
        }
    }

    pub async fn get_avatar(&self, user: String, size: u32) -> AvatarResult<HttpResponse> {
        self.user_service.check_logged_in()?;
        self.user_service.check_csrf()?;

        let user = user.replace("\\", ""); // stripslashes equivalent
        let size = if size > 2048 {
            2048
        } else if size == 0 {
            64
        } else {
            size
        };

        match self.avatar_storage.get(&user, size)? {
            Some(image_data) => {
                let etag = format!("{:x}", crc32fast::hash(&image_data));
                Ok(HttpResponse::Ok()
                    .insert_header(("Cache-Control", "no-cache, no-store, must-revalidate"))
                    .insert_header(("Last-Modified", format!("{}", chrono::Utc::now().timestamp())))
                    .insert_header(("ETag", etag))
                    .content_type("image/png")
                    .body(image_data))
            }
            None => {
                let display_name = self.user_service.get_display_name(&user)?;
                let response = JsonResponse {
                    data: DisplayNameData { displayname: display_name },
                };
                Ok(HttpResponse::Ok().json(response))
            }
        }
    }

    pub async fn post_avatar(&self) -> AvatarResult<HttpResponse> {
        self.user_service.check_logged_in()?;
        self.user_service.check_csrf()?;

        let user = self.user_service.get_user()?;
        let l10n = &self.l10n_service;

        // We'll need to handle various ways to get the new avatar
        let new_avatar_result = self.get_new_avatar_data(&user).await;

        match new_avatar_result {
            Ok(new_avatar) => {
                match self.avatar_storage.set(&user, &new_avatar) {
                    Ok(_) => Ok(HttpResponse::Ok().json(())),
                    Err(AvatarError::NotSquareError) => {
                        // Handle non-square image
                        match image::load_from_memory(&new_avatar) {
                            Ok(img) => {
                                if img.width() > 0 && img.height() > 0 {
                                    self.cache_service.set("tmpavatar", new_avatar, 7200)?;
                                    Ok(HttpResponse::BadRequest().json("notsquare"))
                                } else {
                                    let message = l10n.translate("Invalid image");
                                    Ok(HttpResponse::BadRequest().json(JsonResponse {
                                        data: ErrorMessage { message },
                                    }))
                                }
                            }
                            Err(_) => {
                                let message = l10n.translate("Invalid image");
                                Ok(HttpResponse::BadRequest().json(JsonResponse {
                                    data: ErrorMessage { message },
                                }))
                            }
                        }
                    }
                    Err(e) => {
                        Ok(HttpResponse::InternalServerError().json(JsonResponse {
                            data: ErrorMessage { message: e.to_string() },
                        }))
                    }
                }
            }
            Err(e) => {
                Ok(HttpResponse::BadRequest().json(JsonResponse {
                    data: ErrorMessage { message: e.to_string() },
                }))
            }
        }
    }

    async fn get_new_avatar_data(&self, user: &str) -> AvatarResult<Vec<u8>> {
        // This function would need to be adapted based on how you handle form data
        // in your web framework. This is a simplified version.

        // TODO: Replace with actual form data handling in your framework
        let form_data: Option<AvatarPathForm> = None; // This would come from form extraction
        let has_files = false; // This would be determined by your file upload handling

        if let Some(form) = form_data {
            // Path provided in form
            self.file_system.get_file_contents(user, &form.path)
        } else if has_files {
            // File upload handling would go here
            // This is a placeholder for file upload handling
            Err(AvatarError::NoImageProvided)
        } else {
            Err(AvatarError::NoImageProvided)
        }
    }

    pub async fn delete_avatar(&self) -> AvatarResult<HttpResponse> {
        self.user_service.check_logged_in()?;
        self.user_service.check_csrf()?;

        let user = self.user_service.get_user()?;

        match self.avatar_storage.remove(&user) {
            Ok(_) => Ok(HttpResponse::Ok().json(())),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(JsonResponse {
                    data: ErrorMessage { message: e.to_string() },
                }))
            }
        }
    }

    pub async fn get_tmp_avatar(&self) -> AvatarResult<HttpResponse> {
        self.user_service.check_logged_in()?;
        self.user_service.check_csrf()?;

        match self.cache_service.get("tmpavatar")? {
            Some(tmp_avatar) => {
                let etag = format!("{:x}", crc32fast::hash(&tmp_avatar));
                Ok(HttpResponse::Ok()
                    .insert_header(("Cache-Control", "no-cache, no-store, must-revalidate"))
                    .insert_header(("Last-Modified", format!("{}", chrono::Utc::now().timestamp())))
                    .insert_header(("ETag", etag))
                    .content_type("image/png")
                    .body(tmp_avatar))
            }
            None => {
                let message = self.l10n_service.translate("No temporary profile picture available, try again");
                Ok(HttpResponse::BadRequest().json(JsonResponse {
                    data: ErrorMessage { message },
                }))
            }
        }
    }

    pub async fn post_cropped_avatar(&self, crop_data: Option<web::Json<CropData>>) -> AvatarResult<HttpResponse> {
        self.user_service.check_logged_in()?;
        self.user_service.check_csrf()?;

        let user = self.user_service.get_user()?;
        let l10n = &self.l10n_service;

        let crop = match crop_data {
            Some(data) => data,
            None => {
                let message = l10n.translate("No crop data provided");
                return Ok(HttpResponse::BadRequest().json(JsonResponse {
                    data: ErrorMessage { message },
                }));
            }
        };

        match self.cache_service.get("tmpavatar")? {
            Some(tmp_avatar) => {
                // Crop the image
                let img = image::load_from_memory(&tmp_avatar)?;
                let cropped = img.crop_imm(crop.x, crop.y, crop.w, crop.h);
                
                let mut buffer = Vec::new();
                let mut cursor = std::io::Cursor::new(&mut buffer);
                cropped.write_to(&mut cursor, ImageFormat::Png)?;

                // Set the new avatar
                match self.avatar_storage.set(&user, &buffer) {
                    Ok(_) => {
                        // Clean up
                        self.cache_service.remove("tmpavatar")?;
                        Ok(HttpResponse::Ok().json(()))
                    }
                    Err(e) => {
                        Ok(HttpResponse::InternalServerError().json(JsonResponse {
                            data: ErrorMessage { message: e.to_string() },
                        }))
                    }
                }
            }
            None => {
                let message = l10n.translate("No temporary profile picture available, try again");
                Ok(HttpResponse::BadRequest().json(JsonResponse {
                    data: ErrorMessage { message },
                }))
            }
        }
    }
}

// Web handlers to connect to the Avatar methods
pub async fn get_avatar(
    path: web::Path<(String, u32)>,
    avatar_service: web::Data<Avatar>,
) -> Result<HttpResponse, Error> {
    let (user, size) = path.into_inner();
    avatar_service.get_avatar(user, size)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))
}

pub async fn post_avatar(
    avatar_service: web::Data<Avatar>,
) -> Result<HttpResponse, Error> {
    avatar_service.post_avatar()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))
}

pub async fn delete_avatar(
    avatar_service: web::Data<Avatar>,
) -> Result<HttpResponse, Error> {
    avatar_service.delete_avatar()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))
}

pub async fn get_tmp_avatar(
    avatar_service: web::Data<Avatar>,
) -> Result<HttpResponse, Error> {
    avatar_service.get_tmp_avatar()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))
}

pub async fn post_cropped_avatar(
    crop_data: Option<web::Json<CropData>>,
    avatar_service: web::Data<Avatar>,
) -> Result<HttpResponse, Error> {
    avatar_service.post_cropped_avatar(crop_data)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))
}