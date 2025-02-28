use std::io::{Read, Write};
use std::fs::{self, File, rename};
use std::path::{Path, PathInfo};
use std::time::SystemTime;
use std::convert::TryFrom;
use std::sync::Arc;
use async_std::io::prelude::*;
use rand::random;
use log::{error, info, warn};

use crate::connector::sabre::node::{Node, GetEtagPropertyForPath};
use crate::connector::sabre::exception::{
    DavException, ForbiddenException, ServiceUnavailableException, 
    EntityTooLargeException, UnsupportedMediaTypeException, 
    BadRequestException, NotImplementedException
};
use crate::files::{Filesystem, FileInfo, ChunkingHandler};
use crate::util::{EncryptionUtil, RequestUtil, UrlUtil};
use crate::log::LogUtil;

/// Implements a file in the DAV tree
///
/// This class represents a file in the DAV tree. It handles file operations
/// like reading, writing and deleting.
pub struct File {
    inner: Node,
    fileinfo_cache: Option<FileInfo>,
}

impl File {
    pub const GETETAG_PROPERTYNAME: &'static str = "getetag";

    /// Create a new File instance
    pub fn new(path: String) -> Self {
        Self {
            inner: Node::new(path),
            fileinfo_cache: None,
        }
    }

    /// Get the filesystem instance
    fn get_fs(&self) -> Arc<dyn Filesystem> {
        self.inner.get_fs()
    }

    /// Get the file path
    fn path(&self) -> &str {
        self.inner.path()
    }

    /// Updates the data
    ///
    /// The data argument is a readable stream resource.
    ///
    /// After a successful put operation, you may choose to return an ETag. The
    /// etag must always be surrounded by double-quotes. These quotes must
    /// appear in the actual string you're returning.
    ///
    /// Clients may use the ETag from a PUT request to later on make sure that
    /// when they update the file, the contents haven't changed in the mean
    /// time.
    ///
    /// If you don't plan to store the file byte-by-byte, and you return a
    /// different object on a subsequent GET you are strongly recommended to not
    /// return an ETag, and just return null.
    pub async fn put<R: Read>(&self, mut data: R) -> Result<Option<String>, DavException> {
        let fs = self.get_fs();

        if fs.file_exists(self.path()) && !fs.is_updatable(self.path()) {
            return Err(ForbiddenException::new().into());
        }

        // throw an exception if encryption was disabled but the files are still encrypted
        if EncryptionUtil::encrypted_files() {
            return Err(ServiceUnavailableException::new().into());
        }

        // chunked handling
        if RequestUtil::header_exists("HTTP_OC_CHUNKED") {
            return self.create_file_chunked(data).await;
        }

        // mark file as partial while uploading (ignored by the scanner)
        let mut part_path = format!("{}.part", self.path());

        // if file is located in /Shared we write the part file to the users
        // root folder because we can't create new files in /shared
        // we extend the name with a random number to avoid overwriting a existing file
        if Path::new(&part_path).parent().unwrap().to_str().unwrap() == "Shared" {
            let file_name = Path::new(&part_path)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap();
            part_path = format!("{}{}.part", file_name, random::<u32>());
        }

        let put_result = fs.file_put_contents(&part_path, &mut data);
        
        match put_result {
            Err(err) => {
                match err {
                    // a more general case - due to whatever reason the content could not be written
                    FilesystemError::NotPermitted(msg) => {
                        return Err(ForbiddenException::new_with_message(&msg).into());
                    },
                    // the file is too big to be stored
                    FilesystemError::EntityTooLarge(msg) => {
                        return Err(EntityTooLargeException::new(&msg).into());
                    },
                    // the file content is not permitted
                    FilesystemError::InvalidContent(msg) => {
                        return Err(UnsupportedMediaTypeException::new(&msg).into());
                    },
                    // the path for the file was not valid
                    FilesystemError::InvalidPath(msg) => {
                        return Err(ForbiddenException::new_with_message(&msg).into());
                    },
                    // any other error
                    _ => {
                        LogUtil::write("webdav", "Filesystem::file_put_contents() failed", LogLevel::Error);
                        let _ = fs.unlink(&part_path);
                        return Err(DavException::new().into());
                    }
                }
            },
            Ok(false) => {
                LogUtil::write("webdav", "Filesystem::file_put_contents() failed", LogLevel::Error);
                let _ = fs.unlink(&part_path);
                // because we have no clue about the cause we can only throw back a 500/Internal Server Error
                return Err(DavException::new().into());
            },
            Ok(true) => {
                // continue with the rename operation
            }
        }

        // rename to correct path
        let rename_okay = fs.rename(&part_path, self.path());
        let file_exists = fs.file_exists(self.path());
        
        if !rename_okay || !file_exists {
            LogUtil::write("webdav", "Filesystem::rename() failed", LogLevel::Error);
            let _ = fs.unlink(&part_path);
            return Err(DavException::new().into());
        }

        // allow sync clients to send the mtime along in a header
        if let Some(mtime) = RequestUtil::has_modification_time() {
            if fs.touch(self.path(), mtime) {
                RequestUtil::set_header("X-OC-MTime", "accepted");
            }
        }

        Ok(self.inner.get_etag_property_for_path(self.path()))
    }

    /// Returns the data
    pub fn get(&self) -> Result<Box<dyn Read>, DavException> {
        // throw exception if encryption is disabled but files are still encrypted
        if EncryptionUtil::encrypted_files() {
            return Err(ServiceUnavailableException::new().into());
        } else {
            match Filesystem::fopen(self.path(), "rb") {
                Ok(reader) => Ok(reader),
                Err(_) => Err(DavException::new().into())
            }
        }
    }

    /// Delete the current file
    pub fn delete(&self) -> Result<(), DavException> {
        if self.path() == "Shared" {
            return Err(ForbiddenException::new().into());
        }

        if !Filesystem::is_deletable(self.path()) {
            return Err(ForbiddenException::new().into());
        }

        match Filesystem::unlink(self.path()) {
            Ok(_) => {
                // remove properties
                self.inner.remove_properties();
                Ok(())
            },
            Err(_) => Err(DavException::new().into())
        }
    }

    /// Returns the size of the node, in bytes
    pub fn get_size(&self) -> Option<u64> {
        self.get_fileinfo_cache();
        
        if let Some(fileinfo) = &self.fileinfo_cache {
            if fileinfo.size > -1 {
                return Some(fileinfo.size as u64);
            }
        }
        
        None
    }

    /// Returns the ETag for a file
    ///
    /// An ETag is a unique identifier representing the current version of the
    /// file. If the file changes, the ETag MUST change.  The ETag is an
    /// arbitrary string, but MUST be surrounded by double-quotes.
    ///
    /// Return null if the ETag can not effectively be determined
    pub fn get_etag(&self) -> Option<String> {
        let properties = self.inner.get_properties(&[Self::GETETAG_PROPERTYNAME]);
        
        properties.get(Self::GETETAG_PROPERTYNAME).cloned()
    }

    /// Returns the mime-type for a file
    ///
    /// If null is returned, we'll assume application/octet-stream
    pub fn get_content_type(&self) -> Option<String> {
        if let Some(fileinfo) = &self.fileinfo_cache {
            if let Some(mimetype) = &fileinfo.mimetype {
                return Some(mimetype.clone());
            }
        }

        Filesystem::get_mime_type(self.path())
    }

    /// Get file info from cache
    fn get_fileinfo_cache(&self) {
        if self.fileinfo_cache.is_none() {
            self.fileinfo_cache = self.inner.get_fileinfo_cache();
        }
    }

    /// Handle chunked file uploads
    async fn create_file_chunked<R: Read>(&self, mut data: R) -> Result<Option<String>, DavException> {
        let (path, name) = UrlUtil::split_path(self.path());
        
        let info = ChunkingHandler::decode_name(name);
        if info.is_none() {
            return Err(NotImplementedException::new().into());
        }
        
        let info = info.unwrap();
        let mut chunk_handler = ChunkingHandler::new(info.clone());
        let bytes_written = chunk_handler.store(info.index, &mut data)?;

        // detect aborted upload
        if RequestUtil::method() == "PUT" {
            if let Some(expected) = RequestUtil::content_length() {
                if bytes_written != expected {
                    chunk_handler.remove(info.index)?;
                    return Err(BadRequestException::new(&format!(
                        "expected filesize {} got {}", expected, bytes_written
                    )).into());
                }
            }
        }

        if chunk_handler.is_complete() {
            // we first assembly the target file as a part file
            let part_file = format!("{}/{}.ocTransferId{}.part", path, info.name, info.transferid);
            chunk_handler.file_assemble(&part_file)?;

            // here is the final atomic rename
            let fs = self.get_fs();
            let target_path = format!("{}/{}", path, info.name);
            let rename_okay = fs.rename(&part_file, &target_path);
            let file_exists = fs.file_exists(&target_path);
            
            if !rename_okay || !file_exists {
                LogUtil::write("webdav", "Filesystem::rename() failed", LogLevel::Error);
                let _ = fs.unlink(&target_path);
                return Err(DavException::new().into());
            }

            // allow sync clients to send the mtime along in a header
            if let Some(mtime) = RequestUtil::has_modification_time() {
                if fs.touch(&target_path, mtime) {
                    RequestUtil::set_header("X-OC-MTime", "accepted");
                }
            }

            return Ok(Node::get_etag_property_for_path(&target_path));
        }

        Ok(None)
    }
}

impl DavFile for File {
    fn put<R: Read>(&self, data: R) -> Result<Option<String>, DavException> {
        async_std::task::block_on(self.put(data))
    }

    fn get(&self) -> Result<Box<dyn Read>, DavException> {
        self.get()
    }

    fn delete(&self) -> Result<(), DavException> {
        self.delete()
    }

    fn get_size(&self) -> Option<u64> {
        self.get_size()
    }

    fn get_etag(&self) -> Option<String> {
        self.get_etag()
    }

    fn get_content_type(&self) -> Option<String> {
        self.get_content_type()
    }
}

pub trait DavFile {
    fn put<R: Read>(&self, data: R) -> Result<Option<String>, DavException>;
    fn get(&self) -> Result<Box<dyn Read>, DavException>;
    fn delete(&self) -> Result<(), DavException>;
    fn get_size(&self) -> Option<u64>;
    fn get_etag(&self) -> Option<String>;
    fn get_content_type(&self) -> Option<String>;
}