//! Copyright (c) 2012 Frank Karlitschek <frank@owncloud.org>
//!               2013 Bjoern Schiessle <schiessle@owncloud.com>
//! This file is licensed under the Affero General Public License version 3 or
//! later.
//! See the COPYING-README file.

//! Versions
//!
//! A module to handle the versioning of files.

use chrono::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::config;
use crate::db;
use crate::files::{FileInfo, FileSystem, FileView, UserStorage};
use crate::hooks::Hook;
use crate::user;
use crate::util;

/// Storage handles file versioning operations
pub struct Storage;

impl Storage {
    pub const DEFAULT_ENABLED: bool = true;
    pub const DEFAULT_MAX_SIZE: u8 = 50; // unit: percentage; 50% of available disk space/quota
    pub const VERSIONS_ROOT: &'static str = "files_versions/";

    /// Configuration for version retention policy
    pub fn max_versions_per_interval() -> HashMap<u8, VersionInterval> {
        let mut intervals = HashMap::new();
        
        // first 10sec, one version every 2sec
        intervals.insert(1, VersionInterval {
            interval_ends_after: 10,
            step: 2,
        });
        
        // next minute, one version every 10sec
        intervals.insert(2, VersionInterval {
            interval_ends_after: 60,
            step: 10,
        });
        
        // next hour, one version every minute
        intervals.insert(3, VersionInterval {
            interval_ends_after: 3600,
            step: 60,
        });
        
        // next 24h, one version every hour
        intervals.insert(4, VersionInterval {
            interval_ends_after: 86400,
            step: 3600,
        });
        
        // next 30days, one version per day
        intervals.insert(5, VersionInterval {
            interval_ends_after: 2592000,
            step: 86400,
        });
        
        // until the end one version per week
        intervals.insert(6, VersionInterval {
            interval_ends_after: -1,
            step: 604800,
        });
        
        intervals
    }

    /// Get the user ID and filename for a given file path
    pub fn get_uid_and_filename(filename: &str) -> Result<(String, String), String> {
        let uid = FileSystem::get_owner(filename)?;
        FileSystem::init_mount_points(&uid)?;
        
        let mut result_filename = filename.to_string();
        
        if uid != user::get_current_user()? {
            let info = FileSystem::get_file_info(filename)?;
            let owner_view = FileView::new(format!("/{}/files", uid));
            result_filename = owner_view.get_path(info.file_id)?;
        }
        
        Ok((uid, result_filename))
    }

    /// Get current size of all versions from a given user
    fn get_versions_size(user: &str) -> Result<Option<i64>, String> {
        let query = db::prepare("SELECT `size` FROM `*PREFIX*files_versions` WHERE `user`=?")?;
        let result = query.execute(&[&user])?;

        if result.rows_count() > 0 {
            let row = result.get_row(0)?;
            Ok(Some(row.get_i64("size")?))
        } else {
            Ok(None)
        }
    }

    /// Write to the database how much space is in use for versions
    fn set_versions_size(user: &str, size: i64) -> Result<(), String> {
        let query = match Self::get_versions_size(user)? {
            None => db::prepare("INSERT INTO `*PREFIX*files_versions` (`size`, `user`) VALUES (?, ?)")?,
            Some(_) => db::prepare("UPDATE `*PREFIX*files_versions` SET `size`=? WHERE `user`=?")?,
        };
        
        query.execute(&[&size, &user])?;
        Ok(())
    }

    /// Store a new version of a file
    pub fn store(filename: &str) -> Result<bool, String> {
        if config::get_system_value::<bool>("files_versions", Self::DEFAULT_ENABLED)? {
            // If the file gets streamed we need to remove the .part extension
            // to get the right target
            let filename = if filename.ends_with(".part") {
                &filename[..filename.len() - 5]
            } else {
                filename
            };

            let (uid, filename) = Self::get_uid_and_filename(filename)?;

            let files_view = FileView::new(format!("/{}/files", uid));
            let users_view = FileView::new(format!("/{}", uid));
            let versions_view = FileView::new(format!("/{}/files_versions", uid));

            // Check if filename is a directory
            if files_view.is_dir(&filename)? {
                return Ok(false);
            }

            // We should have a source file to work with, and the file shouldn't be empty
            let file_exists = files_view.file_exists(&filename)?;
            if !(file_exists && files_view.filesize(&filename)? > 0) {
                return Ok(false);
            }

            // Create all parent folders
            Self::create_missing_directories(&filename, &users_view)?;

            let mut versions_size = match Self::get_versions_size(&uid)? {
                Some(size) if size >= 0 => size,
                _ => Self::calculate_size(&uid)?,
            };

            // Assumption: we need filesize($filename) for the new version +
            // some more free space for the modified file which might be
            // 1.5 times as large as the current version -> 2.5
            let needed_space = (files_view.filesize(&filename)? as f64 * 2.5) as i64;

            versions_size = Self::expire(&filename, Some(versions_size), needed_space)?;

            // Store a new version of a file
            let mtime = users_view.filemtime(&format!("files{}", filename))?;
            users_view.copy(
                &format!("files{}", filename),
                &format!("files_versions{}.v{}", filename, mtime)
            )?;

            versions_size += users_view.filesize(&format!("files{}", filename))?;

            Self::set_versions_size(&uid, versions_size)?;
        }
        
        Ok(true)
    }

    /// Delete versions of a file
    pub fn delete(filename: &str) -> Result<(), String> {
        let (uid, filename) = Self::get_uid_and_filename(filename)?;
        let versions_fileview = FileView::new(format!("/{}/files_versions", uid));

        let abs_path = versions_fileview.get_local_file(&format!("{}.v", filename))?;
        
        if let Ok(versions) = Self::get_versions(&uid, &filename) {
            let mut versions_size = match Self::get_versions_size(&uid)? {
                Some(size) if size >= 0 => size,
                _ => Self::calculate_size(&uid)?,
            };
            
            for v in versions.values() {
                std::fs::remove_file(format!("{}{}", abs_path, v.version))?;
                Hook::emit("\\OCP\\Versions", "delete", json!({
                    "path": format!("{}{}", abs_path, v.version)
                }))?;
                versions_size -= v.size;
            }
            
            Self::set_versions_size(&uid, versions_size)?;
        }
        
        Ok(())
    }

    /// Rename versions of a file
    pub fn rename(old_path: &str, new_path: &str) -> Result<bool, String> {
        let (uid, oldpath) = Self::get_uid_and_filename(old_path)?;
        let (uidn, newpath) = Self::get_uid_and_filename(new_path)?;
        
        let versions_view = FileView::new(format!("/{}/files_versions", uid));
        let files_view = FileView::new(format!("/{}/files", uid));

        // If the file already exists then it was an upload of an existing file
        // over the web interface -> store() is the right function we need here
        if files_view.file_exists(&newpath)? {
            return Self::store(new_path);
        }

        Self::expire(&newpath)?;

        if files_view.is_dir(&oldpath)? && versions_view.is_dir(&oldpath)? {
            versions_view.rename(&oldpath, &newpath)?;
        } else if let Ok(versions) = Self::get_versions(&uid, &oldpath) {
            // Create missing dirs if necessary
            Self::create_missing_directories(&newpath, &FileView::new(format!("/{}", uidn)))?;

            for v in versions.values() {
                versions_view.rename(
                    &format!("{}.v{}", oldpath, v.version),
                    &format!("{}.v{}", newpath, v.version)
                )?;
            }
        }
        
        Ok(true)
    }

    /// Rollback to an old version of a file
    pub fn rollback(file: &str, revision: i64) -> Result<bool, String> {
        if config::get_system_value::<bool>("files_versions", Self::DEFAULT_ENABLED)? {
            let (uid, filename) = Self::get_uid_and_filename(file)?;
            let users_view = FileView::new(format!("/{}", uid));
            let files_view = FileView::new(format!("/{}/files", user::get_current_user()?));
            let mut version_created = false;

            // First create a new version
            let mtime = users_view.filemtime(&format!("files{}", filename))?;
            let version = format!("files_versions{}.v{}", filename, mtime);
            
            if !users_view.file_exists(&version)? {
                users_view.copy(
                    &format!("files{}", filename),
                    &version
                )?;
                version_created = true;
            }

            // Rollback
            if users_view.rename(
                &format!("files_versions{}.v{}", filename, revision),
                &format!("files{}", filename)
            ).is_ok() {
                files_view.touch(file, revision)?;
                Self::expire(file)?;
                return Ok(true);
            } else if version_created {
                users_view.unlink(&version)?;
            }
        }
        
        Ok(false)
    }

    /// Get a list of all available versions of a file in descending chronological order
    pub fn get_versions(uid: &str, filename: &str) -> Result<HashMap<String, Version>, String> {
        let mut versions = HashMap::new();
        // Fetch for old versions
        let view = FileView::new(format!("/{}/{}", uid, Self::VERSIONS_ROOT));

        let pathinfo = PathBuf::from(filename);
        let dirname = pathinfo.parent().unwrap_or_else(|| Path::new("")).to_string_lossy().to_string();
        let versioned_file = pathinfo.file_name().unwrap_or_default().to_string_lossy().to_string();

        let files = view.get_directory_content(&dirname)?;

        for file in files {
            if file.type_field == "file" {
                if let Some(pos) = file.path.rfind(".v") {
                    let current_file = &file.name[..file.name.rfind(".v").unwrap_or(0)];
                    
                    if current_file == versioned_file {
                        let version_str = &file.path[pos + 2..];
                        let version: i64 = version_str.parse().unwrap_or(0);
                        let key = format!("{}#{}", version, filename);
                        
                        versions.insert(key, Version {
                            cur: 0,
                            version,
                            human_readable_timestamp: Self::get_human_readable_timestamp(version),
                            preview: util::link_to_route("core_ajax_versions_preview", &[
                                ("file", filename),
                                ("version", &version.to_string()),
                                ("user", uid)
                            ]),
                            path: filename.to_string(),
                            size: file.size,
                        });
                    }
                }
            }
        }

        Ok(versions)
    }

    /// Translate a timestamp into a string like "5 days ago"
    fn get_human_readable_timestamp(timestamp: i64) -> String {
        let diff = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() as i64 - timestamp;

        if diff < 60 {
            // first minute
            format!("{} seconds ago", diff)
        } else if diff < 3600 {
            // first hour
            format!("{} minutes ago", diff / 60)
        } else if diff < 86400 {
            // first day
            format!("{} hours ago", diff / 3600)
        } else if diff < 604800 {
            // first week
            format!("{} days ago", diff / 86400)
        } else if diff < 2419200 {
            // first month
            format!("{} weeks ago", diff / 604800)
        } else if diff < 29030400 {
            // first year
            format!("{} months ago", diff / 2419200)
        } else {
            format!("{} years ago", diff / 29030400)
        }
    }

    /// Deletes used space for files versions in db if user was deleted
    pub fn delete_user(uid: &str) -> Result<(), String> {
        let query = db::prepare("DELETE FROM `*PREFIX*files_versions` WHERE `user`=?")?;
        query.execute(&[&uid])?;
        Ok(())
    }

    /// Get the size of all stored versions from a given user
    fn calculate_size(uid: &str) -> Result<i64, String> {
        if config::get_system_value::<bool>("files_versions", Self::DEFAULT_ENABLED)? {
            let versions_fileview = FileView::new(format!("/{}/files_versions", uid));
            let versions_root = versions_fileview.get_local_folder("")?;

            let walker = walkdir::WalkDir::new(&versions_root)
                .min_depth(1)
                .into_iter()
                .filter_map(|e| e.ok());

            let mut size = 0;

            for entry in walker {
                let path = entry.path();
                if path.to_string_lossy().contains(".v") {
                    let rel_path = path.strip_prefix(&versions_root).unwrap_or(path);
                    size += versions_fileview.filesize(rel_path.to_string_lossy().as_ref())?;
                }
            }

            return Ok(size);
        }
        
        Ok(0)
    }

    /// Returns all stored file versions from a given user
    fn get_all_versions(uid: &str) -> Result<VersionCollections, String> {
        let view = FileView::new(format!("/{}/", uid));
        let mut dirs = vec![Self::VERSIONS_ROOT.to_string()];
        let mut versions = HashMap::new();

        while let Some(dir) = dirs.pop() {
            let files = view.get_directory_content(&dir)?;

            for file in files {
                if file.type_field == "dir" {
                    dirs.push(file.path);
                } else {
                    if let Some(versions_begin) = file.path.rfind(".v") {
                        let rel_path_start = Self::VERSIONS_ROOT.len();
                        let version_str = &file.path[versions_begin + 2..];
                        let version: i64 = version_str.parse().unwrap_or(0);
                        let rel_path = &file.path[rel_path_start..versions_begin];
                        
                        let key = format!("{}#{}", version, rel_path);
                        versions.insert(key, VersionItem {
                            path: rel_path.to_string(),
                            timestamp: version,
                        });
                    }
                }
            }
        }

        // Sort versions
        let mut sorted_versions: Vec<_> = versions.iter().collect();
        sorted_versions.sort_by(|a, b| a.0.cmp(&b.0));

        let mut result = VersionCollections {
            all: HashMap::new(),
            by_file: HashMap::new(),
        };

        for (key, value) in sorted_versions {
            let size = view.filesize(&value.path)?;
            let filename = &value.path;

            let version_entry = VersionEntry {
                version: value.timestamp,
                path: filename.clone(),
                size,
            };

            result.all.insert(key.clone(), version_entry.clone());
            
            result.by_file
                .entry(filename.clone())
                .or_insert_with(HashMap::new)
                .insert(key.clone(), version_entry);
        }

        Ok(result)
    }

    /// Erase a file's versions which exceed the set quota
    fn expire(filename: &str, versions_size: Option<i64>, offset: i64) -> Result<i64, String> {
        if config::get_system_value::<bool>("files_versions", Self::DEFAULT_ENABLED)? {
            let (uid, filename) = Self::get_uid_and_filename(filename)?;
            let versions_fileview = FileView::new(format!("/{}/files_versions", uid));

            // Get available disk space for user
            let mut soft_quota = true;
            let mut quota = user::get_preference(&uid, "files", "quota")?;
            
            if quota.is_none() || quota == Some("default".to_string()) {
                quota = config::get_app_value("files", "default_quota")?;
            }
            
            let quota = if quota.is_none() || quota == Some("none".to_string()) {
                soft_quota = false;
                FileSystem::free_space("/")? as i64
            } else {
                util::computer_file_size(&quota.unwrap_or_default())? as i64
            };

            // Make sure that we have the current size of the version history
            let mut versions_size = versions_size.unwrap_or_else(|| {
                Self::get_versions_size(&uid)
                    .ok()
                    .flatten()
                    .filter(|&size| size >= 0)
                    .unwrap_or_else(|| Self::calculate_size(&uid).unwrap_or(0))
            });

            // Calculate available space for version history
            // Subtract size of files and current versions size from quota
            let available_space = if soft_quota {
                let files_view = FileView::new(format!("/{}/files", uid));
                let root_info = files_view.get_file_info("/")?;
                let free = quota - root_info.size; // Remaining free space for user
                
                if free > 0 {
                    ((free * Self::DEFAULT_MAX_SIZE as i64) / 100) - (versions_size + offset)
                } else {
                    free - versions_size - offset
                }
            } else {
                quota - offset
            };

            // With the probability of 0.1% we reduce the number of all versions not only for the current file
            let all_files = rand::thread_rng().gen_range(0..1000) == 0;

            let all_versions = Self::get_versions(&uid, &filename)?;
            let mut versions_by_file = HashMap::new();
            versions_by_file.insert(filename.to_string(), all_versions);

            let size_of_deleted_versions = Self::del_old_versions(
                &versions_by_file, 
                &mut versions_by_file[&filename.to_string()], 
                &versions_fileview
            )?;
            
            let mut available_space = available_space + size_of_deleted_versions;
            versions_size = versions_size - size_of_deleted_versions;

            // If still not enough free space we rearrange the versions from all files
            if available_space <= 0 || all_files {
                let result = Self::get_all_versions(&uid)?;
                let size_of_deleted_versions = Self::del_old_versions(
                    &result.by_file,
                    &mut result.all,
                    &versions_fileview
                )?;
                
                available_space = available_space + size_of_deleted_versions;
                versions_size = versions_size - size_of_deleted_versions;
            }

            // Check if enough space is available after versions are rearranged.
            // If not we delete the oldest versions until we meet the size limit for versions,
            // but always keep the two latest versions
            let mut all_versions: Vec<_> = all_versions.values().collect();
            all_versions.sort_by(|a, b| a.version.cmp(&b.version));
            
            let num_of_versions = all_versions.len().saturating_sub(2);
            let mut i = 0;
            
            while available_space < 0 && i < num_of_versions {
                let version = &all_versions[i];
                versions_fileview.unlink(&format!("{}.v{}", version.path, version.version))?;
                
                Hook::emit("\\OCP\\Versions", "delete", json!({
                    "path": format!("{}.v{}", version.path, version.version)
                }))?;
                
                versions_size -= version.size;
                available_space += version.size;
                i += 1;
            }

            return Ok(versions_size);
        }

        Ok(0)
    }

    /// Delete old version from a given list of versions
    fn del_old_versions(
        versions_by_file: &HashMap<String, HashMap<String, Version>>,
        all_versions: &mut HashMap<String, Version>,
        versions_fileview: &FileView
    ) -> Result<i64, String> {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() as i64;
        let mut size = 0;

        // Delete old versions for every given file
        for (_, versions) in versions_by_file {
            let mut versions: Vec<_> = versions.values().collect();
            versions.sort_by(|a, b| b.version.cmp(&a.version)); // Newest version first

            if versions.is_empty() {
                continue;
            }

            let mut interval = 1;
            let intervals = Self::max_versions_per_interval();
            let mut step = intervals[&interval].step;
            let mut next_interval = if intervals[&interval].interval_ends_after == -1 {
                -1
            } else {
                time - intervals[&interval].interval_ends_after
            };

            let first_version = versions[0];
            let prev_timestamp = first_version.version;
            let mut next_version = first_version.version - step;

            for version in &versions[1..] {
                let mut new_interval = true;
                while new_interval {
                    if next_interval == -1 || version.version >= next_interval {
                        if version.version > next_version {
                            // Distance between two version too small, delete version
                            versions_fileview.unlink(&format!("{}.v{}", version.path, version.version))?;
                            
                            Hook::emit("\\OCP\\Versions", "delete", json!({
                                "path": format!("{}.v{}", version.path, version.version)
                            }))?;
                            
                            size += version.size;
                            
                            // Update array with all versions - remove by key
                            let key_to_remove = format!("{}#{}", version.version, version.path);
                            all_versions.remove(&key_to_remove);
                        } else {
                            next_version = version.version - step;
                        }
                        new_interval = false; // Version checked so we can move to the next one
                    } else {
                        // Time to move on to the next interval
                        interval += 1;
                        step = intervals[&interval].step;
                        next_version = prev_timestamp - step;
                        
                        if intervals[&interval].interval_ends_after == -1 {
                            next_interval = -1;
                        } else {
                            next_interval = time - intervals[&interval].interval_ends_after;
                        }
                        
                        new_interval = true; // We changed the interval -> check same version with new interval
                    }
                }
                prev_timestamp = version.version;
            }
        }
        
        Ok(size)
    }

    /// Create recursively missing directories
    fn create_missing_directories(filename: &str, view: &FileView) -> Result<(), String> {
        let dirname = FileSystem::normalize_path(&PathBuf::from(filename).parent().unwrap_or_else(|| Path::new("")));
        let dir_parts: Vec<&str> = dirname.split('/').filter(|s| !s.is_empty()).collect();
        let mut dir = "/files_versions".to_string();
        
        for part in dir_parts {
            dir = format!("{}/{}", dir, part);
            if !view.file_exists(&dir)? {
                view.mkdir(&dir)?;
            }
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct VersionInterval {
    pub interval_ends_after: i64,
    pub step: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub cur: i64,
    pub version: i64,
    pub human_readable_timestamp: String,
    pub preview: String,
    pub path: String,
    pub size: i64,
}

#[derive(Debug)]
struct VersionItem {
    path: String,
    timestamp: i64,
}

#[derive(Debug, Clone)]
struct VersionEntry {
    version: i64,
    path: String,
    size: i64,
}

#[derive(Debug)]
struct VersionCollections {
    all: HashMap<String, VersionEntry>,
    by_file: HashMap<String, HashMap<String, VersionEntry>>,
}