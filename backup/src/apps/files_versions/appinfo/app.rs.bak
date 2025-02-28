use owncloud_core::{
    class_path::ClassPath,
    filesystem::Filesystem,
    hooks::HookManager,
    user::User,
    util::Util,
};

pub fn register() -> Result<(), Box<dyn std::error::Error>> {
    // Register class paths
    ClassPath::register::<files_versions::Storage>("files_versions/lib/versions.php")?;
    ClassPath::register::<files_versions::Hooks>("files_versions/lib/hooks.php")?;
    ClassPath::register::<files_versions::Capabilities>("files_versions/lib/capabilities.php")?;

    // Add scripts and styles
    Util::add_script("files_versions", "versions")?;
    Util::add_style("files_versions", "versions")?;

    // Listen to write signals
    HookManager::connect(
        Filesystem::HOOK_NAMESPACE,
        "write",
        "files_versions::Hooks",
        "write_hook",
    )?;
    
    // Listen to delete and rename signals
    HookManager::connect(
        Filesystem::HOOK_NAMESPACE,
        "post_delete",
        "files_versions::Hooks",
        "remove_hook",
    )?;
    
    HookManager::connect(
        Filesystem::HOOK_NAMESPACE,
        "rename",
        "files_versions::Hooks",
        "rename_hook",
    )?;
    
    // Listen to delete user signal
    HookManager::connect(
        User::HOOK_NAMESPACE,
        "pre_deleteUser",
        "files_versions::Hooks",
        "deleteUser_hook",
    )?;

    Ok(())
}