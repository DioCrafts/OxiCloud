// # Files Settings Module
//
// This module provides functionality for handling file listings and directory navigation.
//
// * Originally authored by Robin Appelman
// * Copyright 2010 Robin Appelman icewind1991@gmail.com
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

use chrono::{DateTime, Local};
use ocp::{
    template::Template,
    user,
    util,
    web::{get_query_param, Response},
};
use oc::files::filesystem;
use std::path::PathBuf;
use anyhow::{Result, Context};

/// File representation structure
#[derive(Debug, Clone)]
struct FileInfo {
    name: String,
    path: String,
    size: u64,
    mtime: i64,
    date: String,
    mime_type: String,
    is_dir: bool,
    permissions: u32,
}

/// Breadcrumb entry for navigation
#[derive(Debug, Clone)]
struct BreadcrumbEntry {
    dir: String,
    name: String,
}

/// Process file listings and render the files index page
pub async fn render_files_page() -> Result<Response> {
    // Check if user is logged in
    user::check_logged_in()?;

    // Load styles and scripts
    util::add_style("files", "files");
    util::add_script("files", "files");

    // Get the current directory from query parameters
    let dir = get_query_param("dir").unwrap_or_else(|| String::from(""));

    // Get directory content and format dates
    let mut files = Vec::new();
    let config_date_format = &*CONFIG_DATEFORMAT;
    
    for item in filesystem::get_directory_content(&dir).context("Failed to get directory content")? {
        let date = DateTime::<Local>::from_timestamp(item.mtime, 0)
            .map(|dt| dt.format(config_date_format).to_string())
            .unwrap_or_else(|| String::from("Unknown date"));
        
        files.push(FileInfo {
            name: item.name.clone(),
            path: item.path.clone(),
            size: item.size,
            mtime: item.mtime,
            date,
            mime_type: item.mime_type.clone(),
            is_dir: item.is_dir,
            permissions: item.permissions,
        });
    }

    // Create breadcrumb navigation
    let mut breadcrumb = Vec::new();
    let mut path_to_here = String::from("/");
    
    for segment in dir.split('/').filter(|s| !s.is_empty()) {
        path_to_here.push_str(&format!("{}/", segment));
        breadcrumb.push(BreadcrumbEntry {
            dir: path_to_here.clone(),
            name: segment.to_string(),
        });
    }

    // Create and render template
    let mut tmpl = Template::new("files", "index", "user")?;
    tmpl.assign("files", &files);
    tmpl.assign("breadcrumb", &breadcrumb);
    
    tmpl.render()
}