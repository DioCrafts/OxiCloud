// Admin interface for file management
//
// Originally from ownCloud - ajax frontend
// 
// @author Robin Appelman
// @copyright 2010 Robin Appelman icewind1991@gmail.com
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
// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use crate::config::Config;
use crate::files::Files;
use crate::template::Template;
use crate::user::User;
use crate::util::{Util, OcUtil};
use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::cmp::min;

#[derive(Deserialize)]
pub struct FilesAdminForm {
    max_upload_size: Option<String>,
    max_zip_input_size: Option<String>,
    submit_files_admin_settings: Option<bool>,
    allow_zip_download: Option<bool>,
}

pub async fn admin(
    form: Option<web::Form<FilesAdminForm>>,
    config: web::Data<Config>,
    user: web::Data<User>,
    util: web::Data<Util>,
    files: web::Data<Files>,
    server_root: web::Data<String>,
) -> Result<HttpResponse> {
    // Check if user has admin rights
    user.check_admin_user()?;

    let htaccess_working = env::var("htaccessWorking").map_or(false, |v| v == "true");

    let upload_max_filesize = util.computer_file_size(&env::var("upload_max_filesize").unwrap_or_default())?;
    let post_max_size = util.computer_file_size(&env::var("post_max_size").unwrap_or_default())?;
    let mut max_upload_filesize = util.human_file_size(min(upload_max_filesize, post_max_size));

    if let Some(form) = form {
        if OcUtil::is_call_registered() {
            // Handle max upload size setting
            if let Some(ref max_size) = form.max_upload_size {
                if let Ok(computer_size) = util.computer_file_size(max_size) {
                    if let Ok(Some(set_max_size)) = files.set_upload_limit(computer_size) {
                        max_upload_filesize = util.human_file_size(set_max_size);
                    }
                }
            }

            // Handle max zip input size setting
            if let Some(ref max_zip_size) = form.max_zip_input_size {
                if let Ok(computer_size) = util.computer_file_size(max_zip_size) {
                    config.set_system_value("maxZipInputSize", computer_size)?;
                }
            }

            // Handle zip download setting
            if form.submit_files_admin_settings.is_some() {
                config.set_system_value("allowZipDownload", form.allow_zip_download.is_some())?;
            }
        }
    }

    let max_zip_input_size_default = util.computer_file_size("800 MB")?;
    let max_zip_input_size = util.human_file_size(
        config.get_system_value("maxZipInputSize", max_zip_input_size_default)?
    );
    let allow_zip_download = config.get_system_value("allowZipDownload", true)?;

    // Set active navigation entry
    crate::app::App::set_active_navigation_entry("files_administration")?;

    let htaccess_writable = std::path::Path::new(&format!("{}/.htaccess", server_root.as_str())).is_writable();

    // Create template with all required variables
    let mut tmpl = Template::new("files", "admin")?;
    tmpl.assign("uploadChangable", htaccess_working && htaccess_writable);
    tmpl.assign("uploadMaxFilesize", max_upload_filesize);
    // max possible makes only sense on a 32 bit system
    tmpl.assign("displayMaxPossibleUploadSize", std::mem::size_of::<usize>() == 4);
    tmpl.assign("maxPossibleUploadSize", util.human_file_size(isize::MAX as u64));
    tmpl.assign("allowZipDownload", allow_zip_download);
    tmpl.assign("maxZipInputSize", max_zip_input_size);

    // Return the rendered template
    Ok(HttpResponse::Ok().content_type("text/html").body(tmpl.fetch_page()?))
}