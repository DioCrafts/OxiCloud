// ownCloud
//
// @author Sam Tuke
// @copyright 2012 Sam Tuke samtuke@owncloud.org
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

use std::path::Path;

use crate::crypt::Crypt;
use crate::helper::Helper;
use crate::keymanager::Keymanager;
use crate::session::Session;
use crate::util::Util;

/// Class for hook specific logic
pub struct Hooks;

impl Hooks {
    /// Startup encryption backend upon user login
    /// This method should never be called for users using client side encryption
    pub fn login(params: &mut LoginParams) -> bool {
        if !app_is_enabled("files_encryption") {
            return true;
        }

        let l = l10n("files_encryption");
        let view = filesystem_view("/");

        // ensure filesystem is loaded
        if !filesystem_is_loaded() {
            setup_fs(&params.uid);
        }

        let private_key = Keymanager::get_private_key(&view, &params.uid);

        // if no private key exists, check server configuration
        if private_key.is_none() {
            // check if all requirements are met
            if !Helper::check_requirements() || !Helper::check_configuration() {
                let error_msg = l.t("Missing requirements.");
                let hint = l.t("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.");
                disable_app("files_encryption");
                write_log("Encryption library", &format!("{} {}", error_msg, hint), "ERROR");
                print_error_page(error_msg, hint);
            }
        }

        let util = Util::new(view.clone(), params.uid.clone());

        // setup user, if user not ready force relogin
        if !Helper::setup_user(&util, &params.password) {
            return false;
        }

        let session = util.init_encryption(params);

        // Check if first-run file migration has already been performed
        let mut ready = false;
        if util.get_migration_status() == Util::MIGRATION_OPEN {
            ready = util.begin_migration();
        }

        // If migration not yet done
        if ready {
            let user_view = filesystem_view(&format!("/{}", params.uid));

            // Set legacy encryption key if it exists, to support
            // depreciated encryption system
            if user_view.file_exists("encryption.key") {
                if let Some(enc_legacy_key) = user_view.file_get_contents("encryption.key") {
                    let plain_legacy_key = Crypt::legacy_decrypt(&enc_legacy_key, &params.password);
                    session.set_legacy_key(plain_legacy_key);
                }
            }

            // Encrypt existing user files:
            if util.encrypt_all(
                &format!("/{}/files", params.uid),
                session.get_legacy_key(),
                &params.password,
            ) {
                write_log(
                    "Encryption library",
                    &format!("Encryption of existing files belonging to \"{}\" completed", params.uid),
                    "INFO",
                );
            }

            // Register successful migration in DB
            util.finish_migration();
        }

        true
    }

    /// Setup encryption backend upon user created
    /// This method should never be called for users using client side encryption
    pub fn post_create_user(params: &CreateUserParams) {
        if app_is_enabled("files_encryption") {
            let view = filesystem_view("/");
            let util = Util::new(view, params.uid.clone());
            Helper::setup_user(&util, &params.password);
        }
    }

    /// Cleanup encryption backend upon user deleted
    /// This method should never be called for users using client side encryption
    pub fn post_delete_user(params: &DeleteUserParams) {
        if app_is_enabled("files_encryption") {
            let view = filesystem_view("/");

            // cleanup public key
            let public_key = format!("/public-keys/{}.public.key", params.uid);

            // Disable encryption proxy to prevent recursive calls
            let proxy_status = file_proxy_enabled();
            set_file_proxy_enabled(false);

            view.unlink(&public_key);

            set_file_proxy_enabled(proxy_status);
        }
    }

    /// If the password can't be changed within ownCloud, than update the key password in advance.
    pub fn pre_set_passphrase(params: &SetPassphraseParams) {
        if app_is_enabled("files_encryption") {
            if !user_can_change_password(&params.uid) {
                Self::set_passphrase(params);
            }
        }
    }

    /// Change a user's encryption passphrase
    pub fn set_passphrase(params: &SetPassphraseParams) {
        if !app_is_enabled("files_encryption") {
            return;
        }

        // Only attempt to change passphrase if server-side encryption
        // is in use (client-side encryption does not have access to
        // the necessary keys)
        if Crypt::mode() == "server" {
            if params.uid == get_user() {
                let view = filesystem_view("/");
                let session = Session::new(view.clone());

                // Get existing decrypted private key
                let private_key = session.get_private_key();

                // Encrypt private key with new user pwd as passphrase
                let encrypted_private_key = Crypt::symmetric_encrypt_file_content(&private_key, &params.password);

                // Save private key
                Keymanager::set_private_key(&encrypted_private_key);

                // NOTE: Session does not need to be updated as the
                // private key has not changed, only the passphrase
                // used to decrypt it has changed
            } else {
                // admin changed the password for a different user, create new keys and reencrypt file keys
                let user = &params.uid;
                let recovery_password = &params.recovery_password;
                let new_user_password = &params.password;

                let view = filesystem_view("/");

                // make sure that the users home is mounted
                init_mount_points(user);

                let keypair = Crypt::create_keypair();

                // Disable encryption proxy to prevent recursive calls
                let proxy_status = file_proxy_enabled();
                set_file_proxy_enabled(false);

                // Save public key
                view.file_put_contents(
                    &format!("/public-keys/{}.public.key", user),
                    &keypair.public_key,
                );

                // Encrypt private key empty passphrase
                let encrypted_private_key = Crypt::symmetric_encrypt_file_content(
                    &keypair.private_key,
                    new_user_password,
                );

                // Save private key
                view.file_put_contents(
                    &format!("/{}/files_encryption/{}.private.key", user, user),
                    &encrypted_private_key,
                );

                if !recovery_password.is_empty() {
                    // if recovery key is set we can re-encrypt the key files
                    let util = Util::new(view, user.clone());
                    util.recover_users_files(recovery_password);
                }

                set_file_proxy_enabled(proxy_status);
            }
        }
    }

    /// Check if files can be encrypted to every user.
    pub fn pre_shared(params: &mut PreSharedParams) {
        if !app_is_enabled("files_encryption") {
            return;
        }

        let l = l10n("files_encryption");
        let mut users = Vec::new();
        let view = files_view("/public-keys/");

        match params.share_type {
            ShareType::User => {
                users.push(params.share_with.clone());
            }
            ShareType::Group => {
                users = group_users_in_group(&params.share_with);
            }
            _ => {}
        }

        let mut not_configured = Vec::new();
        for user in &users {
            if !view.file_exists(&format!("{}.public.key", user)) {
                not_configured.push(user.clone());
            }
        }

        if !not_configured.is_empty() {
            params.run = false;
            params.error = format!(
                "{} {}",
                l.t("Following users are not set up for encryption:"),
                not_configured.join(", ")
            );
        }
    }

    /// Handle post share actions
    pub fn post_shared(params: &PostSharedParams) {
        if !app_is_enabled("files_encryption") {
            return;
        }

        if params.item_type == "file" || params.item_type == "folder" {
            let view = filesystem_view("/");
            let session = Session::new(view.clone());
            let user_id = get_user();
            let util = Util::new(view.clone(), user_id);
            let mut path = util.file_id_to_path(params.item_source);

            let share = util.get_parent_from_share(params.id);
            // if parent is set, then this is a re-share action
            if share.parent.is_some() {
                // get the parent from current share
                let parent = util.get_share_parent(params.parent);

                // if parent is file the it is an 1:1 share
                if parent.item_type == "file" {
                    // prefix path with Shared
                    path = format!("/Shared{}", parent.file_target);
                } else {
                    // NOTE: parent is folder but shared was a file!
                    // we try to rebuild the missing path
                    // some examples we face here
                    // user1 share folder1 with user2 folder1 has
                    // the following structure
                    // /folder1/subfolder1/subsubfolder1/somefile.txt
                    // user2 re-share subfolder2 with user3
                    // user3 re-share somefile.txt user4
                    // so our path should be
                    // /Shared/subfolder1/subsubfolder1/somefile.txt
                    // while user3 is sharing

                    if params.item_type == "file" {
                        // get target path
                        let target_path = util.file_id_to_path(params.file_source);
                        let target_path_split = target_path.split('/').rev().collect::<Vec<_>>();

                        // init values
                        let mut new_path = String::new();
                        let shared_part = parent.file_target.trim_start_matches('/');

                        // rebuild path
                        for path_part in target_path_split {
                            if path_part != shared_part {
                                new_path = format!("/{}{}", path_part, new_path);
                            } else {
                                break;
                            }
                        }
                        // prefix path with Shared
                        path = format!("/Shared{}{}", parent.file_target, new_path);
                    } else {
                        // prefix path with Shared
                        path = format!("/Shared{}{}", parent.file_target, params.file_target);
                    }
                }
            }

            let sharing_enabled = share_is_enabled();

            // get the path including mount point only if not a shared folder
            if !path.starts_with("/Shared") {
                // get path including the the storage mount point
                path = util.get_path_with_mount_point(params.item_source);
            }

            // if a folder was shared, get a list of all (sub-)folders
            let all_files = if params.item_type == "folder" {
                util.get_all_files(&path)
            } else {
                vec![path.clone()]
            };

            for path in all_files {
                let users_sharing = util.get_sharing_users_array(sharing_enabled, &path);
                util.set_shared_file_keyfiles(&session, &users_sharing, &path);
            }
        }
    }

    /// Handle post unshare actions
    pub fn post_unshare(params: &PostUnshareParams) {
        if !app_is_enabled("files_encryption") {
            return;
        }

        if params.item_type == "file" || params.item_type == "folder" {
            let view = filesystem_view("/");
            let user_id = get_user();
            let util = Util::new(view.clone(), user_id);
            let mut path = util.file_id_to_path(params.item_source);

            // check if this is a re-share
            if params.item_parent.is_some() {
                // get the parent from current share
                let parent = util.get_share_parent(params.item_parent);

                // get target path
                let target_path = util.file_id_to_path(params.item_source);
                let target_path_split = target_path.split('/').rev().collect::<Vec<_>>();

                // init values
                let mut new_path = String::new();
                let shared_part = parent.file_target.trim_start_matches('/');

                // rebuild path
                for path_part in target_path_split {
                    if path_part != shared_part {
                        new_path = format!("/{}{}", path_part, new_path);
                    } else {
                        break;
                    }
                }

                // prefix path with Shared
                path = format!("/Shared{}{}", parent.file_target, new_path);
            }

            // for group shares get a list of the group members
            let user_ids = match params.share_type {
                ShareType::Group => group_users_in_group(&params.share_with),
                ShareType::Link => vec![util.get_public_share_key_id()],
                _ => vec![params.share_with.clone()],
            };

            // get the path including mount point only if not a shared folder
            if !path.starts_with("/Shared") {
                // get path including the the storage mount point
                path = util.get_path_with_mount_point(params.item_source);
            }

            // if we unshare a folder we need a list of all (sub-)files
            let all_files = if params.item_type == "folder" {
                util.get_all_files(&path)
            } else {
                vec![path.clone()]
            };

            for path in all_files {
                // check if the user still has access to the file, otherwise delete share key
                let sharing_users = util.get_sharing_users_array(true, &path);

                // Unshare every user who no longer has access to the file
                let del_users: Vec<_> = user_ids
                    .iter()
                    .filter(|uid| !sharing_users.contains(uid))
                    .cloned()
                    .collect();

                // delete share key
                Keymanager::del_share_key(&view, &del_users, &path);
            }
        }
    }

    /// After a file is renamed, rename its keyfile and share-keys also fix the file size and fix also the sharing
    ///
    /// This function is connected to the rename signal of OC_Filesystem and adjust the name and location
    /// of the stored versions along the actual file
    pub fn post_rename(params: &RenameParams) {
        if !app_is_enabled("files_encryption") {
            return;
        }

        // Disable encryption proxy to prevent recursive calls
        let proxy_status = file_proxy_enabled();
        set_file_proxy_enabled(false);

        let view = filesystem_view("/");
        let session = Session::new(view.clone());
        let user_id = get_user();
        let util = Util::new(view.clone(), user_id.clone());

        // Format paths to be relative to user files dir
        let (base_dir, old_keyfile_path) = if util.is_system_wide_mount_point(&params.oldpath) {
            let base = "files_encryption/".to_string();
            (base.clone(), format!("{}keyfiles/{}", base, params.oldpath))
        } else {
            let base = format!("{}/files_encryption/", user_id);
            (base.clone(), format!("{}keyfiles/{}", base, params.oldpath))
        };

        let new_keyfile_path = if util.is_system_wide_mount_point(&params.newpath) {
            format!("{}keyfiles/{}", base_dir, params.newpath)
        } else {
            format!("{}keyfiles/{}", base_dir, params.newpath)
        };

        // add key ext if this is not an folder
        if !view.is_dir(&old_keyfile_path) {
            let old_keyfile_path = format!("{}.key", old_keyfile_path);
            let new_keyfile_path = format!("{}.key", new_keyfile_path);

            // handle share-keys
            let local_key_path = view.get_local_file(&format!("{}share-keys/{}", base_dir, params.oldpath));
            let escaped_path = Helper::escape_glob_pattern(&local_key_path);
            let matches = glob(&format!("{}*.shareKey", escaped_path));
            
            for src in matches {
                let dst = normalize_path(&src.replace(&params.oldpath, &params.newpath));

                // create destination folder if not exists
                if !Path::new(&dst).parent().map_or(true, |p| p.exists()) {
                    std::fs::create_dir_all(Path::new(&dst).parent().unwrap()).unwrap();
                }

                std::fs::rename(&src, &dst).unwrap();
            }
        } else {
            // handle share-keys folders
            let old_share_keyfile_path = format!("{}share-keys/{}", base_dir, params.oldpath);
            let new_share_keyfile_path = format!("{}share-keys/{}", base_dir, params.newpath);

            // create destination folder if not exists
            if !view.file_exists(&Path::new(&new_share_keyfile_path).parent().unwrap().to_str().unwrap()) {
                view.mkdir(&Path::new(&new_share_keyfile_path).parent().unwrap().to_str().unwrap(), 0o750, true);
            }

            view.rename(&old_share_keyfile_path, &new_share_keyfile_path);
        }

        // Rename keyfile so it isn't orphaned
        if view.file_exists(&old_keyfile_path) {
            // create destination folder if not exists
            if !view.file_exists(&Path::new(&new_keyfile_path).parent().unwrap().to_str().unwrap()) {
                view.mkdir(&Path::new(&new_keyfile_path).parent().unwrap().to_str().unwrap(), 0o750, true);
            }

            view.rename(&old_keyfile_path, &new_keyfile_path);
        }

        // build the path to the file
        let new_path = format!("/{}/files{}", user_id, params.newpath);
        let new_path_relative = params.newpath.clone();

        if util.fix_file_size(&new_path) {
            // get sharing app state
            let sharing_enabled = share_is_enabled();

            // get users
            let users_sharing = util.get_sharing_users_array(sharing_enabled, &new_path_relative);

            // update sharing-keys
            util.set_shared_file_keyfiles(&session, &users_sharing, &new_path_relative);
        }

        set_file_proxy_enabled(proxy_status);
    }

    /// Set migration status and the init status back to '0' so that all new files get encrypted
    /// if the app gets enabled again
    pub fn pre_disable(params: &AppParams) {
        if params.app == "files_encryption" {
            // Set migration status to 0
            db_prepare_and_execute("UPDATE `*PREFIX*encryption` SET `migration_status`=0");

            let session = Session::new(filesystem_view("/"));
            session.set_initialized(Session::NOT_INITIALIZED);
        }
    }

    /// Set the init status to 'NOT_INITIALIZED' (0) if the app gets enabled
    pub fn post_enable(params: &AppParams) {
        if params.app == "files_encryption" {
            let session = Session::new(filesystem_view("/"));
            session.set_initialized(Session::NOT_INITIALIZED);
        }
    }
}

// Structs for parameters

pub struct LoginParams {
    pub uid: String,
    pub password: String,
}

pub struct CreateUserParams {
    pub uid: String,
    pub password: String,
}

pub struct DeleteUserParams {
    pub uid: String,
}

pub struct SetPassphraseParams {
    pub uid: String,
    pub password: String,
    pub recovery_password: String,
}

pub struct PreSharedParams {
    pub share_type: ShareType,
    pub share_with: String,
    pub run: bool,
    pub error: String,
}

pub struct PostSharedParams {
    pub item_type: String,
    pub item_source: i64,
    pub parent: i64,
    pub id: i64,
    pub file_source: i64,
    pub file_target: String,
}

pub struct PostUnshareParams {
    pub item_type: String,
    pub item_source: i64,
    pub share_type: ShareType,
    pub share_with: String,
    pub item_parent: Option<i64>,
}

pub struct RenameParams {
    pub oldpath: String,
    pub newpath: String,
}

pub struct AppParams {
    pub app: String,
}

pub enum ShareType {
    User,
    Group,
    Link,
}

// Helper functions (these would be implemented in the actual application)
fn app_is_enabled(app: &str) -> bool {
    // Implementation depends on the framework
    unimplemented!()
}

fn l10n(app: &str) -> L10n {
    // Implementation depends on the framework
    unimplemented!()
}

fn filesystem_view(path: &str) -> FilesystemView {
    // Implementation depends on the framework
    unimplemented!()
}

fn filesystem_is_loaded() -> bool {
    // Implementation depends on the framework
    unimplemented!()
}

fn setup_fs(uid: &str) {
    // Implementation depends on the framework
    unimplemented!()
}

fn disable_app(app: &str) {
    // Implementation depends on the framework
    unimplemented!()
}

fn write_log(app: &str, message: &str, level: &str) {
    // Implementation depends on the framework
    unimplemented!()
}

fn print_error_page(error: &str, hint: &str) {
    // Implementation depends on the framework
    unimplemented!()
}

fn file_proxy_enabled() -> bool {
    // Implementation depends on the framework
    unimplemented!()
}

fn set_file_proxy_enabled(enabled: bool) {
    // Implementation depends on the framework
    unimplemented!()
}

fn user_can_change_password(uid: &str) -> bool {
    // Implementation depends on the framework
    unimplemented!()
}

fn get_user() -> String {
    // Implementation depends on the framework
    unimplemented!()
}

fn init_mount_points(user: &str) {
    // Implementation depends on the framework
    unimplemented!()
}

fn group_users_in_group(group: &str) -> Vec<String> {
    // Implementation depends on the framework
    unimplemented!()
}

fn files_view(path: &str) -> FilesystemView {
    // Implementation depends on the framework
    unimplemented!()
}

fn share_is_enabled() -> bool {
    // Implementation depends on the framework
    unimplemented!()
}

fn glob(pattern: &str) -> Vec<String> {
    // Implementation depends on the framework
    unimplemented!()
}

fn normalize_path(path: &str) -> String {
    // Implementation depends on the framework
    unimplemented!()
}

fn db_prepare_and_execute(query: &str) {
    // Implementation depends on the framework
    unimplemented!()
}

// Mock structures that would be defined in the actual application
pub struct FilesystemView;
pub struct L10n;

impl FilesystemView {
    pub fn file_exists(&self, _path: &str) -> bool {
        unimplemented!()
    }

    pub fn file_get_contents(&self, _path: &str) -> Option<String> {
        unimplemented!()
    }

    pub fn unlink(&self, _path: &str) {
        unimplemented!()
    }

    pub fn is_dir(&self, _path: &str) -> bool {
        unimplemented!()
    }

    pub fn get_local_file(&self, _path: &str) -> String {
        unimplemented!()
    }

    pub fn mkdir(&self, _path: &str, _mode: u32, _recursive: bool) {
        unimplemented!()
    }

    pub fn rename(&self, _oldpath: &str, _newpath: &str) {
        unimplemented!()
    }

    pub fn file_put_contents(&self, _path: &str, _content: &str) {
        unimplemented!()
    }
}

impl L10n {
    pub fn t(&self, _string: &str) -> String {
        unimplemented!()
    }
}