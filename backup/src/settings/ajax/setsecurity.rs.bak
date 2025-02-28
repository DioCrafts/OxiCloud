/**
 * Copyright (c) 2013, Lukas Reschke <lukas@statuscode.ch>
 * This file is licensed under the Affero General Public License version 3 or later.
 * See the COPYING-README file.
 */

use actix_web::{post, web, HttpResponse, Result};
use serde::Deserialize;

use crate::util::check_admin_user;
use crate::util::json_call_check;
use crate::config::set_value;

#[derive(Deserialize)]
struct SecurityForm {
    #[serde(rename = "enforceHTTPS")]
    enforce_https: bool,
}

#[post("/settings/ajax/setsecurity")]
async fn set_security(
    form: web::Form<SecurityForm>,
    req: web::HttpRequest,
) -> Result<HttpResponse> {
    check_admin_user(&req)?;
    json_call_check(&req)?;

    set_value("forcessl", form.enforce_https)?;

    Ok(HttpResponse::Ok().body("true"))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(set_security);
}