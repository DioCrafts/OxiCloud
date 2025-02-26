//! # Filesystem Abstraction Layer
//! 
//! This file provides an abstraction layer for filesystem operations.
//! This is a deprecated implementation that forwards calls to the new implementation.

/// Abstraction of filesystem functions
/// 
/// This class won't call any filesystem functions for itself but will pass them 
/// to the correct Storage object. This class should also handle all the file 
/// permission related stuff.
///
/// Hooks provided:
///   read(path)
///   write(path, &run)
///   post_write(path)
///   create(path, &run) (when a file is created, both create and write will be emitted in that order)
///   post_create(path)
///   delete(path, &run)
///   post_delete(path)
///   rename(oldpath,newpath, &run)
///   post_rename(oldpath,newpath)
///   copy(oldpath,newpath, &run) (if the newpath doesn't exists yes, copy, create and write will be emitted in that order)
///   post_rename(oldpath,newpath)
///
///   the &run parameter can be set to false to prevent the operation from occurring
///
/// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
#[allow(dead_code)]
#[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
pub struct OcFilesystem;

#[allow(deprecated)]
impl OcFilesystem {
    /// Get the mountpoint of the storage object for a path
    /// (note: because a storage is not always mounted inside the fakeroot, the
    /// returned mountpoint is relative to the absolute root of the filesystem
    /// and doesn't take the chroot into account)
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn get_mount_point(path: &str) -> String {
        oc::files::filesystem::get_mount_point(path)
    }

    /// Resolve a path to a storage and internal path
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn resolve_path(path: &str) -> (Box<dyn oc::files::storage::Storage>, String) {
        oc::files::filesystem::resolve_path(path)
    }

    /// Initialize the filesystem
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn init(user: &str, root: &str) -> bool {
        oc::files::filesystem::init(user, root)
    }

    /// Get the default filesystem view
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn get_view() -> oc::files::View {
        oc::files::filesystem::get_view()
    }

    /// Tear down the filesystem, removing all storage providers
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn tear_down() {
        oc::files::filesystem::tear_down()
    }

    /// Get the relative path of the root data directory for the current user
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    /// Returns path like /admin/files
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn get_root() -> String {
        oc::files::filesystem::get_root()
    }

    /// Clear all mounts and storage backends
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn clear_mounts() {
        oc::files::filesystem::clear_mounts()
    }

    /// Mount a Storage in our virtual filesystem
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn mount(class: Box<dyn oc::files::storage::Storage>, arguments: Vec<String>, mountpoint: &str) {
        oc::files::filesystem::mount(class, arguments, mountpoint)
    }

    /// Return the path to a local version of the file
    /// we need this because we can't know if a file is stored local or not from
    /// outside the filestorage and for some purposes a local file is needed
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn get_local_file(path: &str) -> String {
        oc::files::filesystem::get_local_file(path)
    }

    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn get_local_folder(path: &str) -> String {
        oc::files::filesystem::get_local_folder(path)
    }

    /// Return path to file which reflects one visible in browser
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn get_local_path(path: &str) -> String {
        oc::files::filesystem::get_local_path(path)
    }

    /// Check if the requested path is valid
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn is_valid_path(path: &str) -> bool {
        oc::files::filesystem::is_valid_path(path)
    }

    /// Checks if a file is blacklisted for storage in the filesystem
    /// Listens to write and rename hooks
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn is_blacklisted(data: &[String]) {
        oc::files::filesystem::is_blacklisted(data)
    }

    /// Create a directory
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn mkdir(path: &str) -> bool {
        oc::files::filesystem::mkdir(path)
    }

    /// Remove a directory
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn rmdir(path: &str) -> bool {
        oc::files::filesystem::rmdir(path)
    }

    /// Open a directory
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn opendir(path: &str) -> Option<oc::files::dir::Dir> {
        oc::files::filesystem::opendir(path)
    }

    /// Read a directory
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn readdir(path: &str) -> Option<Vec<String>> {
        oc::files::filesystem::readdir(path)
    }

    /// Check if a path is a directory
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn is_dir(path: &str) -> bool {
        oc::files::filesystem::is_dir(path)
    }

    /// Check if a path is a file
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn is_file(path: &str) -> bool {
        oc::files::filesystem::is_file(path)
    }

    /// Get file statistics
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn stat(path: &str) -> Option<oc::files::stat::Stat> {
        oc::files::filesystem::stat(path)
    }

    /// Get file type
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn filetype(path: &str) -> Option<String> {
        oc::files::filesystem::filetype(path)
    }

    /// Get file size
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn filesize(path: &str) -> Option<u64> {
        oc::files::filesystem::filesize(path)
    }

    /// Read file contents
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn readfile(path: &str) -> Option<Vec<u8>> {
        oc::files::filesystem::readfile(path)
    }

    /// Check if a file is readable
    ///
    /// @deprecated Replaced by is_readable() as part of CRUDS
    #[deprecated(note = "Replaced by is_readable() as part of CRUDS")]
    pub fn is_readable(path: &str) -> bool {
        oc::files::filesystem::is_readable(path)
    }

    /// Check if a file is creatable
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn is_creatable(path: &str) -> bool {
        oc::files::filesystem::is_creatable(path)
    }

    /// Check if a file is readable
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn is_readable(path: &str) -> bool {
        oc::files::filesystem::is_readable(path)
    }

    /// Check if a file is updatable
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn is_updatable(path: &str) -> bool {
        oc::files::filesystem::is_updatable(path)
    }

    /// Check if a file is deletable
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn is_deletable(path: &str) -> bool {
        oc::files::filesystem::is_deletable(path)
    }

    /// Check if a file is sharable
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn is_sharable(path: &str) -> bool {
        oc::files::filesystem::is_sharable(path)
    }

    /// Check if a file exists
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn file_exists(path: &str) -> bool {
        oc::files::filesystem::file_exists(path)
    }

    /// Get file modification time
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn filemtime(path: &str) -> Option<i64> {
        oc::files::filesystem::filemtime(path)
    }

    /// Touch a file (update modification time)
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn touch(path: &str, mtime: Option<i64>) -> bool {
        oc::files::filesystem::touch(path, mtime)
    }

    /// Get file contents
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn file_get_contents(path: &str) -> Option<Vec<u8>> {
        oc::files::filesystem::file_get_contents(path)
    }

    /// Put file contents
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn file_put_contents(path: &str, data: &[u8]) -> Option<usize> {
        oc::files::filesystem::file_put_contents(path, data)
    }

    /// Delete a file
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn unlink(path: &str) -> bool {
        oc::files::filesystem::unlink(path)
    }

    /// Rename a file
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn rename(path1: &str, path2: &str) -> bool {
        oc::files::filesystem::rename(path1, path2)
    }

    /// Copy a file
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn copy(path1: &str, path2: &str) -> bool {
        oc::files::filesystem::copy(path1, path2)
    }

    /// Open a file
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn fopen(path: &str, mode: &str) -> Option<Box<dyn std::io::Read + std::io::Write>> {
        oc::files::filesystem::fopen(path, mode)
    }

    /// Move a file to a temporary location
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn to_tmp_file(path: &str) -> Option<String> {
        oc::files::filesystem::to_tmp_file(path)
    }

    /// Move a file from a temporary location
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn from_tmp_file(tmp_file: &str, path: &str) -> bool {
        oc::files::filesystem::from_tmp_file(tmp_file, path)
    }

    /// Get MIME type of a file
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn get_mime_type(path: &str) -> Option<String> {
        oc::files::filesystem::get_mime_type(path)
    }

    /// Get hash of a file
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn hash(hash_type: &str, path: &str, raw: bool) -> Option<String> {
        oc::files::filesystem::hash(hash_type, path, raw)
    }

    /// Get free space
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn free_space(path: &str) -> Option<u64> {
        oc::files::filesystem::free_space(path)
    }

    /// Search for files
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn search(query: &str) -> Vec<String> {
        oc::files::filesystem::search(query)
    }

    /// Check if a file or folder has been updated since a specific time
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn has_updated(path: &str, time: i64) -> bool {
        oc::files::filesystem::has_updated(path, time)
    }

    /// Normalize a path
    ///
    /// @deprecated OC_Filesystem is replaced by \OC\Files\Filesystem
    #[deprecated(note = "OC_Filesystem is replaced by oc::files::filesystem")]
    pub fn normalize_path(path: &str, strip_trailing_slash: bool) -> String {
        oc::files::filesystem::normalize_path(path, strip_trailing_slash)
    }
}