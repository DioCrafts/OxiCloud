// Registry of classes and hook connections for files sharing app

use std::sync::Arc;

use nextcloud_app_api::{
    app::AppManager,
    class_path::ClassPathRegistry,
    hooks::{HookManager, HookTarget},
    share::{ShareBackendManager, ShareType},
    util::ScriptManager,
};

pub fn register(app_manager: &AppManager) -> anyhow::Result<()> {
    // Register class paths
    let class_registry = app_manager.get_class_path_registry();
    
    class_registry.register("OC_Share_Backend_File", "files_sharing/lib/share/file.php")?;
    class_registry.register("OC_Share_Backend_Folder", "files_sharing/lib/share/folder.php")?;
    class_registry.register("OC\\Files\\Storage\\Shared", "files_sharing/lib/sharedstorage.php")?;
    class_registry.register("OC\\Files\\Cache\\Shared_Cache", "files_sharing/lib/cache.php")?;
    class_registry.register("OC\\Files\\Cache\\Shared_Permissions", "files_sharing/lib/permissions.php")?;
    class_registry.register("OC\\Files\\Cache\\Shared_Updater", "files_sharing/lib/updater.php")?;
    class_registry.register("OC\\Files\\Cache\\Shared_Watcher", "files_sharing/lib/watcher.php")?;
    class_registry.register("OCA\\Files\\Share\\Api", "files_sharing/lib/api.php")?;
    class_registry.register("OCA\\Files\\Share\\Maintainer", "files_sharing/lib/maintainer.php")?;

    // Connect hooks
    let hook_manager = app_manager.get_hook_manager();
    
    hook_manager.connect(
        HookTarget::OcFilesystem,
        "setup",
        "\\OC\\Files\\Storage\\Shared",
        "setup",
    )?;

    // Register share backends
    let share_manager = app_manager.get_share_manager();
    
    share_manager.register_backend(
        ShareType::File,
        "OC_Share_Backend_File",
        None,
    )?;
    
    share_manager.register_backend(
        ShareType::Folder,
        "OC_Share_Backend_Folder",
        Some(ShareType::File),
    )?;

    // Add scripts
    let script_manager = app_manager.get_script_manager();
    script_manager.add_script("files_sharing", "share")?;

    // Connect file system and share hooks
    hook_manager.connect(
        HookTarget::OcFilesystem,
        "post_write",
        "\\OC\\Files\\Cache\\Shared_Updater",
        "writeHook",
    )?;
    
    hook_manager.connect(
        HookTarget::OcFilesystem,
        "delete",
        "\\OC\\Files\\Cache\\Shared_Updater",
        "deleteHook",
    )?;
    
    hook_manager.connect(
        HookTarget::OcFilesystem,
        "post_rename",
        "\\OC\\Files\\Cache\\Shared_Updater",
        "renameHook",
    )?;
    
    hook_manager.connect(
        HookTarget::OcpShare,
        "post_shared",
        "\\OC\\Files\\Cache\\Shared_Updater",
        "shareHook",
    )?;
    
    hook_manager.connect(
        HookTarget::OcpShare,
        "pre_unshare",
        "\\OC\\Files\\Cache\\Shared_Updater",
        "shareHook",
    )?;
    
    hook_manager.connect(
        HookTarget::OcAppconfig,
        "post_set_value",
        "\\OCA\\Files\\Share\\Maintainer",
        "configChangeHook",
    )?;

    Ok(())
}