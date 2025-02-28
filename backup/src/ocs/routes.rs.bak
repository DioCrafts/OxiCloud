// Copyright (c) 2012, Tom Needham <tom@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or later.
// See the COPYING-README file.

use actix_web::{web, HttpResponse};
use std::sync::Arc;

pub struct ApiConfig {
    core_module: Arc<dyn CoreModule>,
}

impl ApiConfig {
    pub fn new(core_module: Arc<dyn CoreModule>) -> Self {
        Self { core_module }
    }

    pub fn configure_routes(self, cfg: &mut web::ServiceConfig) {
        let core_module = self.core_module.clone();

        // Config
        cfg.route("/config", web::get().to(move |req| {
            let handlers = core_module.clone();
            async move { handlers.api_config(req).await }
        }))
        .route("/person/check", web::post().to(move |req| {
            let handlers = core_module.clone();
            async move { handlers.person_check(req).await }
        }))
        // Privatedata
        .route("/privatedata/getattribute", web::get().to(move |req| {
            let handlers = core_module.clone();
            async move { handlers.privatedata_get(req, None, None).await }
        }))
        .route("/privatedata/getattribute/{app}", web::get().to(move |req, app: web::Path<String>| {
            let handlers = core_module.clone();
            async move { handlers.privatedata_get(req, Some(app.into_inner()), None).await }
        }))
        .route("/privatedata/getattribute/{app}/{key}", web::get().to(move |req, path: web::Path<(String, String)>| {
            let handlers = core_module.clone();
            let (app, key) = path.into_inner();
            async move { handlers.privatedata_get(req, Some(app), Some(key)).await }
        }))
        .route("/privatedata/setattribute/{app}/{key}", web::post().to(move |req, path: web::Path<(String, String)>| {
            let handlers = core_module.clone();
            let (app, key) = path.into_inner();
            async move { handlers.privatedata_set(req, app, key).await }
        }))
        .route("/privatedata/deleteattribute/{app}/{key}", web::post().to(move |req, path: web::Path<(String, String)>| {
            let handlers = core_module.clone();
            let (app, key) = path.into_inner();
            async move { handlers.privatedata_delete(req, app, key).await }
        }))
        // Cloud
        .route("/cloud/capabilities", web::get().to(move |req| {
            let handlers = core_module.clone();
            async move { handlers.cloud_get_capabilities(req).await }
        }))
        .route("/cloud/users/{userid}", web::get().to(move |req, userid: web::Path<String>| {
            let handlers = core_module.clone();
            async move { handlers.cloud_get_user(req, userid.into_inner()).await }
        }))
        .route("/cloud/user", web::get().to(move |req| {
            let handlers = core_module.clone();
            async move { handlers.cloud_get_current_user(req).await }
        }));
    }
}

#[async_trait::async_trait]
pub trait CoreModule: Send + Sync {
    async fn api_config(&self, req: actix_web::HttpRequest) -> HttpResponse;
    async fn person_check(&self, req: actix_web::HttpRequest) -> HttpResponse;
    async fn privatedata_get(&self, req: actix_web::HttpRequest, app: Option<String>, key: Option<String>) -> HttpResponse;
    async fn privatedata_set(&self, req: actix_web::HttpRequest, app: String, key: String) -> HttpResponse;
    async fn privatedata_delete(&self, req: actix_web::HttpRequest, app: String, key: String) -> HttpResponse;
    async fn cloud_get_capabilities(&self, req: actix_web::HttpRequest) -> HttpResponse;
    async fn cloud_get_user(&self, req: actix_web::HttpRequest, userid: String) -> HttpResponse;
    async fn cloud_get_current_user(&self, req: actix_web::HttpRequest) -> HttpResponse;
}