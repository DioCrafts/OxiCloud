// Ajax frontend for translations
//
// This module provides a way to retrieve translations via an ajax call
//
// # License
// 
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library. If not, see <http://www.gnu.org/licenses/>.
//
// @author Jakob Sack
// @copyright 2011 Jakob Sack kde@jakobsack.de

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::app::clean_app_id;
use crate::l10n::L10n;
use crate::utils::json_response;

#[derive(Deserialize)]
pub struct TranslationRequest {
    app: Option<String>,
}

#[derive(Serialize)]
pub struct TranslationResponse {
    data: std::collections::HashMap<String, String>,
    plural_form: String,
}

pub async fn handle_translations(form: web::Form<TranslationRequest>) -> impl Responder {
    let app = match &form.app {
        Some(app_name) => app_name.clone(),
        None => String::new(),
    };

    let app = clean_app_id(&app);
    
    // Get localization for the app
    let l10n = L10n::get(&app);
    
    // Get translations and plural form
    let translations = l10n.get_translations();
    let plural_form = l10n.get_plural_form_string();
    
    // Construct the response
    let response = TranslationResponse {
        data: translations,
        plural_form,
    };
    
    json_response::success(response)
}