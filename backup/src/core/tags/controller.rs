/**
 * Copyright (c) 2013 Thomas Tanghus (thomas@tanghus.net)
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use std::collections::HashSet;
use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use log::error;

use crate::server::Server;
use crate::auth::Auth;
use crate::l10n::L10n;
use crate::tags::TagManager;

#[derive(Serialize)]
struct JsonResponse<T> {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(flatten)]
    data: Option<T>,
}

#[derive(Serialize)]
struct TagsResponse {
    tags: Vec<String>,
}

#[derive(Serialize)]
struct IdsResponse {
    ids: Vec<String>,
}

#[derive(Serialize)]
struct IdResponse {
    id: String,
}

#[derive(Deserialize)]
struct TagRequest {
    tag: String,
}

#[derive(Deserialize)]
struct TagsRequest {
    tags: Vec<String>,
}

pub struct Controller {
    server: web::Data<Server>,
    auth: web::Data<Auth>,
    l10n: web::Data<L10n>,
}

impl Controller {
    pub fn new(server: web::Data<Server>, auth: web::Data<Auth>, l10n: web::Data<L10n>) -> Self {
        Self { server, auth, l10n }
    }

    async fn get_tagger(&self, tag_type: &str) -> Result<Box<dyn TagManager>> {
        self.auth.check_logged_in()?;
        self.auth.check_csrf_token()?;

        match self.server.get_tag_manager().load(tag_type).await {
            Ok(tagger) => Ok(tagger),
            Err(e) => {
                error!("Controller::get_tagger Exception: {}", e);
                let message = self.l10n.t("core", "Error loading tags");
                Err(actix_web::error::ErrorInternalServerError(message))
            }
        }
    }

    pub async fn get_tags(&self, path: web::Path<(String,)>) -> Result<HttpResponse> {
        let tag_type = &path.0;
        let tagger = self.get_tagger(tag_type).await?;
        
        let tags = tagger.get_tags().await?;
        Ok(HttpResponse::Ok().json(JsonResponse {
            status: "success".to_string(),
            message: None,
            data: Some(TagsResponse { tags }),
        }))
    }

    pub async fn get_favorites(&self, path: web::Path<(String,)>) -> Result<HttpResponse> {
        let tag_type = &path.0;
        let tagger = self.get_tagger(tag_type).await?;
        
        let ids = tagger.get_favorites().await?;
        Ok(HttpResponse::Ok().json(JsonResponse {
            status: "success".to_string(),
            message: None,
            data: Some(IdsResponse { ids }),
        }))
    }

    pub async fn get_ids_for_tag(&self, path: web::Path<(String,)>, query: web::Query<TagRequest>) -> Result<HttpResponse> {
        let tag_type = &path.0;
        let tagger = self.get_tagger(tag_type).await?;
        
        let ids = tagger.get_ids_for_tag(&query.tag).await?;
        Ok(HttpResponse::Ok().json(JsonResponse {
            status: "success".to_string(),
            message: None,
            data: Some(IdsResponse { ids }),
        }))
    }

    pub async fn add_tag(&self, path: web::Path<(String,)>, form: web::Form<TagRequest>) -> Result<HttpResponse> {
        let tag_type = &path.0;
        let tagger = self.get_tagger(tag_type).await?;
        
        // Remove HTML tags
        let cleaned_tag = html_escape::decode_html_entities(&form.tag)
            .to_string()
            .replace(|c: char| c.is_control(), "");
        
        match tagger.add(&cleaned_tag).await {
            Ok(Some(id)) => Ok(HttpResponse::Ok().json(JsonResponse {
                status: "success".to_string(),
                message: None,
                data: Some(IdResponse { id }),
            })),
            Ok(None) => {
                let message = self.l10n.t("core", "Tag already exists");
                Ok(HttpResponse::BadRequest().json(JsonResponse::<()> {
                    status: "error".to_string(),
                    message: Some(message),
                    data: None,
                }))
            },
            Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
        }
    }

    pub async fn delete_tags(&self, path: web::Path<(String,)>, form: web::Form<TagsRequest>) -> Result<HttpResponse> {
        let tag_type = &path.0;
        let tagger = self.get_tagger(tag_type).await?;
        
        let tags = &form.tags;
        
        if tagger.delete(tags).await? {
            Ok(HttpResponse::Ok().json(JsonResponse::<()> {
                status: "success".to_string(),
                message: None,
                data: None,
            }))
        } else {
            let message = self.l10n.t("core", "Error deleting tag(s)");
            Ok(HttpResponse::BadRequest().json(JsonResponse::<()> {
                status: "error".to_string(),
                message: Some(message),
                data: None,
            }))
        }
    }

    pub async fn tag_as(&self, path: web::Path<(String, String)>, form: web::Form<TagRequest>) -> Result<HttpResponse> {
        let (tag_type, id) = (path.0.clone(), path.1.clone());
        let tagger = self.get_tagger(&tag_type).await?;
        
        if tagger.tag_as(&id, &form.tag).await? {
            Ok(HttpResponse::Ok().json(JsonResponse::<()> {
                status: "success".to_string(),
                message: None,
                data: None,
            }))
        } else {
            let message = self.l10n.t("core", "Error tagging");
            Ok(HttpResponse::BadRequest().json(JsonResponse::<()> {
                status: "error".to_string(),
                message: Some(message),
                data: None,
            }))
        }
    }

    pub async fn untag(&self, path: web::Path<(String, String)>, form: web::Form<TagRequest>) -> Result<HttpResponse> {
        let (tag_type, id) = (path.0.clone(), path.1.clone());
        let tagger = self.get_tagger(&tag_type).await?;
        
        if tagger.untag(&id, &form.tag).await? {
            Ok(HttpResponse::Ok().json(JsonResponse::<()> {
                status: "success".to_string(),
                message: None,
                data: None,
            }))
        } else {
            let message = self.l10n.t("core", "Error untagging");
            Ok(HttpResponse::BadRequest().json(JsonResponse::<()> {
                status: "error".to_string(),
                message: Some(message),
                data: None,
            }))
        }
    }

    pub async fn favorite(&self, path: web::Path<(String, String)>) -> Result<HttpResponse> {
        let (tag_type, id) = (path.0.clone(), path.1.clone());
        let tagger = self.get_tagger(&tag_type).await?;
        
        if tagger.add_to_favorites(&id).await? {
            Ok(HttpResponse::Ok().json(JsonResponse::<()> {
                status: "success".to_string(),
                message: None,
                data: None,
            }))
        } else {
            let message = self.l10n.t("core", "Error favoriting");
            Ok(HttpResponse::BadRequest().json(JsonResponse::<()> {
                status: "error".to_string(),
                message: Some(message),
                data: None,
            }))
        }
    }

    pub async fn unfavorite(&self, path: web::Path<(String, String)>) -> Result<HttpResponse> {
        let (tag_type, id) = (path.0.clone(), path.1.clone());
        let tagger = self.get_tagger(&tag_type).await?;
        
        if tagger.remove_from_favorites(&id).await? {
            Ok(HttpResponse::Ok().json(JsonResponse::<()> {
                status: "success".to_string(),
                message: None,
                data: None,
            }))
        } else {
            let message = self.l10n.t("core", "Error unfavoriting");
            Ok(HttpResponse::BadRequest().json(JsonResponse::<()> {
                status: "error".to_string(),
                message: Some(message),
                data: None,
            }))
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tags/{type}")
            .route("", web::get().to(Controller::get_tags))
            .route("", web::post().to(Controller::add_tag))
            .route("", web::delete().to(Controller::delete_tags))
            .route("/favorites", web::get().to(Controller::get_favorites))
            .route("/ids", web::get().to(Controller::get_ids_for_tag))
            .route("/{id}", web::put().to(Controller::tag_as))
            .route("/{id}", web::delete().to(Controller::untag))
            .route("/{id}/favorite", web::put().to(Controller::favorite))
            .route("/{id}/favorite", web::delete().to(Controller::unfavorite))
    );
}