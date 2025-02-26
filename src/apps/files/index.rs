//! # Files Index Controller
//!
//! Original ownCloud - ajax frontend
//!
//! This library is free software; you can redistribute it and/or
//! modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
//! License as published by the Free Software Foundation; either
//! version 3 of the License, or any later version.
//!
//! This library is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//!
//! You should have received a copy of the GNU Affero General Public
//! License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use regex::Regex;
use serde::{Deserialize, Serialize};
use tera::Tera;

use crate::apps::files::helper;
use crate::core::app_config;
use crate::core::config;
use crate::core::template::Template;
use crate::core::user;
use crate::core::util;
use crate::files::filesystem;
use crate::files::cache::upgrade;
use crate::files::view::View;

#[derive(Deserialize)]
pub struct QueryParams {
    dir: Option<String>,
}

#[derive(Serialize)]
struct TemplateData {
    file_list: String,
    breadcrumb: String,
    dir: String,
    is_creatable: bool,
    permissions: i32,
    files: Vec<helper::FileInfo>,
    trash: bool,
    trash_empty: bool,
    upload_max_filesize: u64,
    upload_max_human_filesize: String,
    allow_zip_download: i32,
    used_space_percent: i32,
    is_public: bool,
    public_upload_enabled: String,
    encrypted_files: bool,
    mail_notification_enabled: String,
    allow_share_with_link: String,
    encryption_init_status: i32,
    disable_sharing: bool,
    ajax_load: bool,
    empty_content: bool,
    file_header: bool,
}

pub async fn index(req: HttpRequest, query: web::Query<QueryParams>) -> impl Responder {
    // Check if user is logged in
    if !user::check_logged_in(&req) {
        return HttpResponse::Unauthorized().finish();
    }

    // Load styles and scripts
    util::add_style("files", "files");
    util::add_style("files", "upload");
    util::add_script("files", "file-upload");
    util::add_script("files", "jquery.iframe-transport");
    util::add_script("files", "jquery.fileupload");
    util::add_script("files", "jquery-visibility");
    util::add_script("files", "filelist");

    app_config::set_active_navigation_entry("files_index");

    // Get directory from query params
    let dir = match &query.dir {
        Some(d) => d.clone(),
        None => String::from(""),
    };

    // Redirect if directory does not exist
    if !filesystem::is_dir(&format!("{}/", dir)) {
        let script_name = util::get_script_name();
        return HttpResponse::Found()
            .header("Location", script_name)
            .finish();
    }

    // Check if browser is IE8
    let user_agent = req.headers().get("User-Agent")
        .map(|h| h.to_str().unwrap_or(""))
        .unwrap_or("");
    
    let is_ie8 = Regex::new(r"MSIE (.*?);")
        .unwrap()
        .captures(user_agent)
        .map_or(false, |caps| {
            caps.get(1)
                .map_or(false, |m| m.as_str().parse::<i32>().unwrap_or(9) <= 8)
        });

    // Handle IE8 redirect for dir parameter
    if is_ie8 && query.dir.is_some() {
        let formatted_dir = if dir.is_empty() { "/" } else { &dir };
        let files_link = util::link_to("files", "index.php");
        let encoded_path = util::encode_path(formatted_dir);
        
        return HttpResponse::Found()
            .header("Location", format!("{}#?dir={}", files_link, encoded_path))
            .finish();
    }

    let user = user::get_user(&req).unwrap();
    
    let mut ajax_load = false;
    let mut files = Vec::new();
    let mut need_upgrade = false;
    let mut free_space: u64 = 0;

    // Check if cache needs upgrade
    if upgrade::need_upgrade(&user) {
        need_upgrade = true;
        free_space = 0;
    } else {
        if is_ie8 {
            // For IE8, client will handle ajax loading
            ajax_load = true;
        } else {
            files = helper::get_files(&dir).unwrap_or_default();
        }
        free_space = filesystem::free_space(&dir).unwrap_or(0);
    }

    // Make breadcrumb
    let breadcrumb = helper::make_breadcrumb(&dir);

    // Make file list and breadcrumb markup
    let mut list_template = Template::new("files", "part.list");
    list_template.assign("files", &files);
    list_template.assign("baseURL", &format!("{}?dir=", util::link_to("files", "index.php")));
    list_template.assign("downloadURL", &util::link_to_route("download", &[("file", "/")]));
    list_template.assign("isPublic", &false);
    
    let mut breadcrumb_template = Template::new("files", "part.breadcrumb");
    breadcrumb_template.assign("breadcrumb", &breadcrumb);
    breadcrumb_template.assign("baseURL", &format!("{}?dir=", util::link_to("files", "index.php")));

    let permissions = helper::get_dir_permissions(&dir);

    if need_upgrade {
        util::add_script("files", "upgrade");
        let upgrade_template = Template::new("files", "upgrade");
        return HttpResponse::Ok().body(upgrade_template.render());
    } else {
        // Storage information
        let storage_info = helper::get_storage_info(&dir);
        let max_upload_filesize = util::max_upload_filesize(&dir);
        let public_upload_enabled = app_config::get_value("core", "shareapi_allow_public_upload", "yes");
        
        // Default encryption status code (INIT_SUCCESSFUL)
        let mut encryption_init_status = 2;
        let mut public_upload_enabled = public_upload_enabled;
        
        if app_config::is_enabled("files_encryption") {
            public_upload_enabled = String::from("no");
            let mut view = View::new("/");
            let session = encryption::Session::new(&mut view);
            encryption_init_status = session.get_initialized();
        }

        let trash_enabled = app_config::is_enabled("files_trashbin");
        let trash_empty = if trash_enabled {
            trashbin::is_empty(&user)
        } else {
            true
        };

        let is_creatable = filesystem::is_creatable(&format!("{}/", dir));
        let file_header = files.len() > 0;
        let empty_content = (is_creatable && !file_header) || ajax_load;

        util::add_script("files", "fileactions");
        util::add_script("files", "files");
        util::add_script("files", "keyboardshortcuts");
        
        let mut template = Template::new("files", "index");
        
        let normalized_dir = filesystem::normalize_path(&dir);
        
        let data = TemplateData {
            file_list: list_template.render(),
            breadcrumb: breadcrumb_template.render(),
            dir: normalized_dir,
            is_creatable,
            permissions,
            files,
            trash: trash_enabled,
            trash_empty,
            upload_max_filesize: max_upload_filesize,
            upload_max_human_filesize: util::human_file_size(max_upload_filesize),
            allow_zip_download: config::get_system_value("allowZipDownload", true) as i32,
            used_space_percent: storage_info.get("relative").unwrap_or(&0).to_owned() as i32,
            is_public: false,
            public_upload_enabled,
            encrypted_files: util::encrypted_files(),
            mail_notification_enabled: app_config::get_value("core", "shareapi_allow_mail_notification", "yes"),
            allow_share_with_link: app_config::get_value("core", "shareapi_allow_links", "yes"),
            encryption_init_status,
            disable_sharing: false,
            ajax_load,
            empty_content,
            file_header,
        };
        
        template.render_with_data(&data)
    }
}