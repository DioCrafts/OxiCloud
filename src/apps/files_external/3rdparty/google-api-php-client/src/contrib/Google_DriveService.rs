// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A trait for service resources
#[async_trait]
pub trait ServiceResource {
    async fn call(&self, method: &str, params: Vec<HashMap<String, serde_json::Value>>) -> Result<serde_json::Value, ServiceError>;
    fn use_objects(&self) -> bool;
}

#[derive(Debug)]
pub struct ServiceError {
    message: String,
}

impl ServiceError {
    pub fn new(message: &str) -> Self {
        ServiceError {
            message: message.to_string(),
        }
    }
}

/// The "about" collection of methods.
/// Typical usage is:
///
/// let drive_service = Google_DriveService::new(...);
/// let about = drive_service.about;
///
pub struct AboutServiceResource {
    service: Box<dyn ServiceResource>,
}

impl AboutServiceResource {
    pub fn new(service: Box<dyn ServiceResource>) -> Self {
        AboutServiceResource { service }
    }

    /// Gets the information about the current user along with Drive API settings (about.get)
    ///
    /// # Parameters
    ///
    /// * `opt_params` - Optional parameters.
    ///
    /// ## Optional Parameters
    ///
    /// * `include_subscribed` - When calculating the number of remaining change IDs, whether to include shared files and public files the user has opened. When set to false, this counts only change IDs for owned files and any shared or public files that the user has explictly added to a folder in Drive.
    /// * `max_change_id_count` - Maximum number of remaining change IDs to count
    /// * `start_change_id` - Change ID to start counting from when calculating number of remaining change IDs
    pub async fn get(&self, mut opt_params: HashMap<String, serde_json::Value>) -> Result<About, ServiceError> {
        let mut params = HashMap::new();
        params.extend(opt_params);
        
        let data = self.service.call("get", vec![params]).await?;
        
        if self.service.use_objects() {
            let about = serde_json::from_value(data).map_err(|e| ServiceError::new(&e.to_string()))?;
            Ok(about)
        } else {
            Ok(data.try_into().map_err(|_| ServiceError::new("Failed to convert data to About"))?)
        }
    }
}

/// The "apps" collection of methods.
/// Typical usage is:
///
/// let drive_service = Google_DriveService::new(...);
/// let apps = drive_service.apps;
///
pub struct AppsServiceResource {
    service: Box<dyn ServiceResource>,
}

impl AppsServiceResource {
    pub fn new(service: Box<dyn ServiceResource>) -> Self {
        AppsServiceResource { service }
    }

    /// Gets a specific app. (apps.get)
    ///
    /// # Parameters
    ///
    /// * `app_id` - The ID of the app.
    /// * `opt_params` - Optional parameters.
    pub async fn get(&self, app_id: &str, mut opt_params: HashMap<String, serde_json::Value>) -> Result<App, ServiceError> {
        let mut params = HashMap::new();
        params.insert("appId".to_string(), serde_json::Value::String(app_id.to_string()));
        params.extend(opt_params);
        
        let data = self.service.call("get", vec![params]).await?;
        
        if self.service.use_objects() {
            let app = serde_json::from_value(data).map_err(|e| ServiceError::new(&e.to_string()))?;
            Ok(app)
        } else {
            Ok(data.try_into().map_err(|_| ServiceError::new("Failed to convert data to App"))?)
        }
    }

    /// Lists a user's installed apps. (apps.list)
    ///
    /// # Parameters
    ///
    /// * `opt_params` - Optional parameters.
    pub async fn list_apps(&self, mut opt_params: HashMap<String, serde_json::Value>) -> Result<AppList, ServiceError> {
        let mut params = HashMap::new();
        params.extend(opt_params);
        
        let data = self.service.call("list", vec![params]).await?;
        
        if self.service.use_objects() {
            let app_list = serde_json::from_value(data).map_err(|e| ServiceError::new(&e.to_string()))?;
            Ok(app_list)
        } else {
            Ok(data.try_into().map_err(|_| ServiceError::new("Failed to convert data to AppList"))?)
        }
    }
}

/// The "changes" collection of methods.
/// Typical usage is:
///
/// let drive_service = Google_DriveService::new(...);
/// let changes = drive_service.changes;
///
pub struct ChangesServiceResource {
    service: Box<dyn ServiceResource>,
}

impl ChangesServiceResource {
    pub fn new(service: Box<dyn ServiceResource>) -> Self {
        ChangesServiceResource { service }
    }

    /// Gets a specific change. (changes.get)
    ///
    /// # Parameters
    ///
    /// * `change_id` - The ID of the change.
    /// * `opt_params` - Optional parameters.
    pub async fn get(&self, change_id: &str, mut opt_params: HashMap<String, serde_json::Value>) -> Result<Change, ServiceError> {
        let mut params = HashMap::new();
        params.insert("changeId".to_string(), serde_json::Value::String(change_id.to_string()));
        params.extend(opt_params);
        
        let data = self.service.call("get", vec![params]).await?;
        
        if self.service.use_objects() {
            let change = serde_json::from_value(data).map_err(|e| ServiceError::new(&e.to_string()))?;
            Ok(change)
        } else {
            Ok(data.try_into().map_err(|_| ServiceError::new("Failed to convert data to Change"))?)
        }
    }

    /// Lists the changes for a user. (changes.list)
    ///
    /// # Parameters
    ///
    /// * `opt_params` - Optional parameters.
    ///
    /// ## Optional Parameters
    ///
    /// * `include_deleted` - Whether to include deleted items.
    /// * `include_subscribed` - Whether to include shared files and public files the user has opened. When set to false, the list will include owned files plus any shared or public files the user has explictly added to a folder in Drive.
    /// * `max_results` - Maximum number of changes to return.
    /// * `page_token` - Page token for changes.
    /// * `start_change_id` - Change ID to start listing changes from.
    pub async fn list_changes(&self, mut opt_params: HashMap<String, serde_json::Value>) -> Result<ChangeList, ServiceError> {
        let mut params = HashMap::new();
        params.extend(opt_params);
        
        let data = self.service.call("list", vec![params]).await?;
        
        if self.service.use_objects() {
            let change_list = serde_json::from_value(data).map_err(|e| ServiceError::new(&e.to_string()))?;
            Ok(change_list)
        } else {
            Ok(data.try_into().map_err(|_| ServiceError::new("Failed to convert data to ChangeList"))?)
        }
    }
}

/// The "children" collection of methods.
/// Typical usage is:
///
/// let drive_service = Google_DriveService::new(...);
/// let children = drive_service.children;
///
pub struct ChildrenServiceResource {
    service: Box<dyn ServiceResource>,
}

impl ChildrenServiceResource {
    pub fn new(service: Box<dyn ServiceResource>) -> Self {
        ChildrenServiceResource { service }
    }

    /// Removes a child from a folder. (children.delete)
    ///
    /// # Parameters
    ///
    /// * `folder_id` - The ID of the folder.
    /// * `child_id` - The ID of the child.
    /// * `opt_params` - Optional parameters.
    pub async fn delete(&self, folder_id: &str, child_id: &str, mut opt_params: HashMap<String, serde_json::Value>) -> Result<serde_json::Value, ServiceError> {
        let mut params = HashMap::new();
        params.insert("folderId".to_string(), serde_json::Value::String(folder_id.to_string()));
        params.insert("childId".to_string(), serde_json::Value::String(child_id.to_string()));
        params.extend(opt_params);
        
        let data = self.service.call("delete", vec![params]).await?;
        Ok(data)
    }

    /// Gets a specific child reference. (children.get)
    ///
    /// # Parameters
    ///
    /// * `folder_id` - The ID of the folder.
    /// * `child_id` - The ID of the child.
    /// * `opt_params` - Optional parameters.
    pub async fn get(&self, folder_id: &str, child_id: &str, mut opt_params: HashMap<String, serde_json::Value>) -> Result<ChildReference, ServiceError> {
        let mut params = HashMap::new();
        params.insert("folderId".to_string(), serde_json::Value::String(folder_id.to_string()));
        params.insert("childId".to_string(), serde_json::Value::String(child_id.to_string()));
        params.extend(opt_params);
        
        let data = self.service.call("get", vec![params]).await?;
        
        if self.service.use_objects() {
            let child_ref = serde_json::from_value(data).map_err(|e| ServiceError::new(&e.to_string()))?;
            Ok(child_ref)
        } else {
            Ok(data.try_into().map_err(|_| ServiceError::new("Failed to convert data to ChildReference"))?)
        }
    }

    /// Inserts a file into a folder. (children.insert)
    ///
    /// # Parameters
    ///
    /// * `folder_id` - The ID of the folder.
    /// * `post_body` - The ChildReference to insert.
    /// * `opt_params` - Optional parameters.
    pub async fn insert(&self, folder_id: &str, post_body: ChildReference, mut opt_params: HashMap<String, serde_json::Value>) -> Result<ChildReference, ServiceError> {
        let mut params = HashMap::new();
        params.insert("folderId".to_string(), serde_json::Value::String(folder_id.to_string()));
        
        let body_value = serde_json::to_value(post_body).map_err(|e| ServiceError::new(&e.to_string()))?;
        params.insert("postBody".to_string(), body_value);
        params.extend(opt_params);
        
        let data = self.service.call("insert", vec![params]).await?;
        
        if self.service.use_objects() {
            let child_ref = serde_json::from_value(data).map_err(|e| ServiceError::new(&e.to_string()))?;
            Ok(child_ref)
        } else {
            Ok(data.try_into().map_err(|_| ServiceError::new("Failed to convert data to ChildReference"))?)
        }
    }

    /// Lists a folder's children. (children.list)
    ///
    /// # Parameters
    ///
    /// * `folder_id` - The ID of the folder.
    /// * `opt_params` - Optional parameters.
    ///
    /// ## Optional Parameters
    ///
    /// * `max_results` - Maximum number of children to return.
    /// * `page_token` - Page token for children.
    /// * `q` - Query string for searching children.
    pub async fn list_children(&self, folder_id: &str, mut opt_params: HashMap<String, serde_json::Value>) -> Result<ChildList, ServiceError> {
        let mut params = HashMap::new();
        params.insert("folderId".to_string(), serde_json::Value::String(folder_id.to_string()));
        params.extend(opt_params);
        
        let data = self.service.call("list", vec![params]).await?;
        
        if self.service.use_objects() {
            let child_list = serde_json::from_value(data).map_err(|e| ServiceError::new(&e.to_string()))?;
            Ok(child_list)
        } else {
            Ok(data.try_into().map_err(|_| ServiceError::new("Failed to convert data to ChildList"))?)
        }
    }
}

/// The "comments" collection of methods.
/// Typical usage is:
///
/// let drive_service = Google_DriveService::new(...);
/// let comments = drive_service.comments;
///
pub struct CommentsServiceResource {
    service: Box<dyn ServiceResource>,
}

impl CommentsServiceResource {
    pub fn new(service: Box<dyn ServiceResource>) -> Self {
        CommentsServiceResource { service }
    }

    /// Deletes a comment. (comments.delete)
    ///
    /// # Parameters
    ///
    /// * `file_id` - The ID of the file.
    /// * `comment_id` - The ID of the comment.
    /// * `opt_params` - Optional parameters.
    pub async fn delete(&self, file_id: &str, comment_id: &str, mut opt_params: HashMap<String, serde_json::Value>) -> Result<serde_json::Value, ServiceError> {
        let mut params = HashMap::new();
        params.insert("fileId".to_string(), serde_json::Value::String(file_id.to_string()));
        params.insert("commentId".to_string(), serde_json::Value::String(comment_id.to_string()));
        params.extend(opt_params);
        
        let data = self.service.call("delete", vec![params]).await?;
        Ok(data)
    }

    /// Gets a comment by ID. (comments.get)
    ///
    /// # Parameters
    ///
    /// * `file_id` - The ID of the file.
    /// * `comment_id` - The ID of the comment.
    /// * `opt_params` - Optional parameters.
    ///
    /// ## Optional Parameters
    ///
    /// * `include_deleted` - If set, this will succeed when retrieving a deleted comment, and will include any deleted replies.
    pub async fn get(&self, file_id: &str, comment_id: &str, mut opt_params: HashMap<String, serde_json::Value>) -> Result<Comment, ServiceError> {
        let mut params = HashMap::new();
        params.insert("fileId".to_string(), serde_json::Value::String(file_id.to_string()));
        params.insert("commentId".to_string(), serde_json::Value::String(comment_id.to_string()));
        params.extend(opt_params);
        
        let data = self.service.call("get", vec![params]).await?;
        
        if self.service.use_objects() {
            let comment = serde_json::from_value(data).map_err(|e| ServiceError::new(&e.to_string()))?;
            Ok(comment)
        } else {
            Ok(data.try_into().map_err(|_| ServiceError::new("Failed to convert data to Comment"))?)
        }
    }

    /// Creates a new comment on the given file. (comments.insert)
    ///
    /// # Parameters
    ///
    /// * `file_id` - The ID of the file.
    /// * `post_body` - The Comment to insert.
    /// * `opt_params` - Optional parameters.
    pub async fn insert(&self, file_id: &str, post_body: Comment, mut opt_params: HashMap<String, serde_json::Value>) -> Result<Comment, ServiceError> {
        let mut params = HashMap::new();
        params.insert("fileId".to_string(), serde_json::Value::String(file_id.to_string()));
        
        let body_value = serde_json::to_value(post_body).map_err(|e| ServiceError::new(&e.to_string()))?;
        params.insert("postBody".to_string(), body_value);
        params.extend(opt_params);
        
        let data = self.service.call("insert", vec![params]).await?;
        
        if self.service.use_objects() {
            let comment = serde_json::from_value(data).map_err(|e| ServiceError::new(&e.to_string()))?;
            Ok(comment)
        } else {
            Ok(data.try_into().map_err(|_| ServiceError::new("Failed to convert data to Comment"))?)
        }
    }

    /// Lists a file's comments. (comments.list)
    ///
    /// # Parameters
    ///
    /// * `file_id` - The ID of the file.
    /// * `opt_params` - Optional parameters.
    ///
    /// ## Optional Parameters
    ///
    /// * `include_deleted` - If set, all comments and replies, including deleted comments and replies (with content stripped) will be returned.
    /// * `max_results` - The maximum number of discussions to include in the response, used for paging.
    /// * `page_token` - The continuation token, used to page through large result sets. To get the next page of results, set this parameter to the value of "nextPageToken" from the previous response.
    /// * `updated_min` - Only discussions that were updated after this timestamp will be returned. Formatted as an RFC 3339 timestamp.
    pub async fn list_comments(&self, file_id: &str, mut opt_params: HashMap<String, serde_json::Value>) -> Result<CommentList, ServiceError> {
        let mut params = HashMap::new();
        params.insert("fileId".to_string(), serde_json::Value::String(file_id.to_string()));
        params.extend(opt_params);
        
        let data = self.service.call("list", vec![params]).await?;
        
        if self.service.use_objects() {
            let comment_list = serde_json::from_value(data).map_err(|e| ServiceError::new(&e.to_string()))?;
            Ok(comment_list)
        } else {
            Ok(data.try_into().map_err(|_| ServiceError::new("Failed to convert data to CommentList"))?)
        }
    }

    /// Updates an existing comment. This method supports patch semantics. (comments.patch)
    ///
    /// # Parameters
    ///
    /// * `file_id` - The ID of the file.
    /// * `comment_id` - The ID of the comment.
    /// * `post_body` - The Comment with updates.
    /// * `opt_params` - Optional parameters.
    pub async fn patch(&self, file_id: &str, comment_id: &str, post_body: Comment, mut opt_params: HashMap<String, serde_json::Value>) -> Result<Comment, ServiceError> {
        let mut params = HashMap::new();
        params.insert("fileId".to_string(), serde_json::Value::String(file_id.to_string()));
        params.insert("commentId".to_string(), serde_json::Value::String(comment_id.to_string()));
        
        let body_value = serde_json::to_value(post_body).map_err(|e| ServiceError::new(&e.to_string()))?;
        params.insert("postBody".to_string(), body_value);
        params.extend(opt_params);
        
        let data = self.service.call("patch", vec![params]).await?;
        
        if self.service.use_objects() {
            let comment = serde_json::from_value(data).map_err(|e| ServiceError::new(&e.to_string()))?;
            Ok(comment)
        } else {
            Ok(data.try_into().map_err(|_| ServiceError::new("Failed to convert data to Comment"))?)
        }
    }

    /// Updates an existing comment. (comments.update)
    ///
    /// # Parameters
    ///
    /// * `file_id` - The ID of the file.
    /// * `comment_id` - The ID of the comment.
    /// * `post_body` - The Comment with updates.
    /// * `opt_params` - Optional parameters.
    pub async fn update(&self, file_id: &str, comment_id: &str, post_body: Comment, mut opt_params: HashMap<String, serde_json::Value>) -> Result<Comment, ServiceError> {
        let mut params = HashMap::new();
        params.insert("fileId".to_string(), serde_json::Value::String(file_id.to_string()));
        params.insert("commentId".to_string(), serde_json::Value::String(comment_id.to_string()));
        
        let body_value = serde_json::to_value(post_body).map_err(|e| ServiceError::new(&e.to_string()))?;
        params.insert("postBody".to_string(), body_value);
        params.extend(opt_params);
        
        let data = self.service.call("update", vec![params]).await?;
        
        if self.service.use_objects() {
            let comment = serde_json::from_value(data).map_err(|e| ServiceError::new(&e.to_string()))?;
            Ok(comment)
        } else {
            Ok(data.try_into().map_err(|_| ServiceError::new("Failed to convert data to Comment"))?)
        }
    }
}

/// The "files" collection of methods.
/// Typical usage is:
///
/// let drive_service = Google_DriveService::new(...);
/// let files = drive_service.files;
///
pub struct FilesServiceResource {
    service: Box<dyn ServiceResource>,
}

impl FilesServiceResource {
    pub fn new(service: Box<dyn ServiceResource>) -> Self {
        FilesServiceResource { service }
    }

    /// Creates a copy of the specified file. (files.copy)
    ///
    /// # Parameters
    ///
    /// * `file_id` - The ID of the file to copy.
    /// * `post_body` - The DriveFile to copy.
    /// * `opt_params` - Optional parameters.
    ///
    /// ## Optional Parameters
    ///
    /// * `convert` - Whether to convert this file to the corresponding Google Docs format.
    /// * `ocr` - Whether to attempt OCR on .jpg, .png, .gif, or .pdf uploads.
    /// * `ocr_language` - If ocr is true, hints at the language to use. Valid values are ISO 639-1 codes.
    /// * `pinned` - Whether to pin the head revision of the new copy.
    /// * `timed_text_language` - The language of the timed text.
    /// * `timed_text_track_name` - The timed text track name.
    pub async fn copy(&self, file_id: &str, post_body: DriveFile, mut opt_params: HashMap<String, serde_json::Value>) -> Result<DriveFile, ServiceError> {
        let mut params = HashMap::new();
        params.insert("fileId".to_string(), serde_json::Value::String(file_id.to_string()));
        
        let body_value = serde_json::to_value(post_body).map_err(|e| ServiceError::new(&e.to_string()))?;
        params.insert("postBody".to_string(), body_value);
        params.extend(opt_params);
        
        let data = self.service.call("copy", vec![params]).await?;
        
        if self.service.use_objects() {
            let file = serde_json::from_value(data).map_err(|e| ServiceError::new(&e.to_string()))?;
            Ok(file)
        } else {
            Ok(data.try_into().map_err(|_| ServiceError::new("Failed to convert data to DriveFile"))?)
        }
    }

    /// Permanently deletes a file by ID. Skips the trash. (files.delete)
    ///
    /// # Parameters
    ///
    /// * `file_id` - The ID of the file to delete.
    /// * `opt_params` - Optional parameters.
    pub async fn delete(&self, file_id: &str, mut opt_params: HashMap<String, serde_json::Value>) -> Result<serde_json::Value, ServiceError> {
        let mut params = HashMap::new();
        params.insert("fileId".to_string(), serde_json::Value::String(file_id.to_string()));
        params.extend(opt_params);
        
        let data = self.service.call("delete", vec![params]).await?;
        Ok(data)
    }

    /// Gets a file's metadata by ID. (files.get)
    ///
    /// # Parameters
    ///
    /// * `file_id` - The ID for the file in question.
    /// * `opt_params` - Optional parameters.
    ///
    /// ## Optional Parameters
    ///
    /// * `projection` - This parameter is deprecated and has no function.
    /// * `update_viewed_date` - Whether to update the view date after successfully retrieving the file.
    pub async fn get(&self, file_id: &str, mut opt_params: HashMap<String, serde_json::Value>) -> Result<DriveFile, ServiceError> {
        let mut params = HashMap::new();
        params.insert("fileId".to_string(), serde_json::Value::String(file_id.to_string()));
        params.extend(opt_params);
        
        let data = self.service.call("get", vec![params]).await?;
        
        if self.service.use_objects() {
            let file = serde_json::from_value(data).map_err(|e| ServiceError::new(&e.to_string()))?;
            Ok(file)
        } else {
            Ok(data.try_into().map_err(|_| ServiceError::new("Failed to convert data to DriveFile"))?)
        }
    }

    /// Insert a new file. (files.insert)
    ///
    /// # Parameters
    ///
    /// * `post_body` - The DriveFile to insert.
    /// * `opt_params` - Optional parameters.
    ///
    /// ## Optional Parameters
    ///
    /// * `convert` - Whether to convert this file to the corresponding Google Docs format.
    /// * `ocr` - Whether to attempt OCR on .jpg, .png, .gif, or .pdf uploads.
    /// * `ocr_language` - If ocr is true, hints at the language to use. Valid values are ISO 639-1 codes.
    /// * `pinned` - Whether to pin the head revision of the uploaded file.
    /// * `timed_text_language` - The language of the timed text.
    /// * `timed_text_track_name` - The timed text track name.
    /// * `use_content_as_indexable_text` - Whether to use the content as indexable text.
    pub async fn insert(&self, post_body: DriveFile, mut opt_params: HashMap<String, serde_json::Value>) -> Result<DriveFile, ServiceError> {
        let mut params = HashMap::new();
        
        let body_value = serde_json::to_value(post_body).map_err(|e| ServiceError::new(&e.to_string()))?;
        params.insert("postBody".to_string(), body_value);
        params.extend(opt_params);
        
        let data = self.service.call("insert", vec![params]).await?;
        
        if self.service.use_objects() {
            let file = serde_json::from_value(data).map_err(|e| ServiceError::new(&e.to_string()))?;
            Ok(file)
        } else {
            Ok(data.try_into().map_err(|_| ServiceError::new("Failed to convert data to DriveFile"))?)
        }
    }

    /// Lists the user's files. (files.list)
    ///
    /// # Parameters
    ///
    /// * `opt_params` - Optional parameters.
    ///
    /// ## Optional Parameters
    ///
    /// * `max_results` - Maximum number of files to return.
    /// * `page_token` - Page token for files.
    /// * `projection` - This parameter is deprecated and has no function.
    /// * `q` - Query string for searching files.
    pub async fn list_files(&self, mut opt_params: HashMap<String, serde_json::Value>) -> Result<FileList, ServiceError> {
        let mut params = HashMap::new();
        params.extend(opt_params);
        
        let data = self.service.call("list", vec![params]).await?;
        
        if self.service.use_objects() {
            let file_list = serde_json::from_value(data).map_err(|e| ServiceError::new(&e.to_string()))?;
            Ok(file_list)
        } else {
            Ok(data.try_into().map_err(|_| ServiceError::new("Failed to convert data to FileList"))?)
        }
    }

    /// Updates file metadata and/or content. This method supports patch semantics. (files.patch)
    ///
    /// # Parameters
    ///
    /// * `file_id` - The ID of the file to update.
    /// * `post_body` - The DriveFile with updates.
    /// * `opt_params` - Optional parameters.
    ///
    /// ## Optional Parameters
    ///
    /// * `convert` - Whether to convert this file to the corresponding Google Docs format.
    /// * `new_revision` - Whether a blob upload should create a new revision. If not set or false, the blob data in the current head revision is replaced. If true, a new blob is created as head revision, and previous revisions are preserved (causing increased use of the user's data storage quota).
    /// * `ocr` - Whether to attempt OCR on .jpg, .png, .gif, or .pdf uploads.
    /// * `ocr_language` - If ocr is true, hints at the language to use. Valid values are ISO 639-1 codes.
    /// * `pinned` - Whether to pin the new revision.
    /// * `set_modified_date` - Whether to set the modified date with the supplied modified date.
    /// * `timed_text_language`