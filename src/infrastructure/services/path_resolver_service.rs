//! Single-query WebDAV path resolver.
//!
//! Replaces the double-query pattern (`get_folder_by_path` + `get_file_by_path`)
//! with a single `UNION ALL` query that returns the first match.  PostgreSQL's
//! `Append` node short-circuits on `LIMIT 1`, so if the folder branch matches
//! the file branch is never executed.

use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::application::dtos::display_helpers::{
    category_for, format_file_size, icon_class_for, icon_special_class_for,
};
use crate::application::dtos::file_dto::FileDto;
use crate::application::dtos::folder_dto::FolderDto;
use crate::common::errors::DomainError;

/// Result of resolving a WebDAV path — either a folder or a file.
#[derive(Debug, Clone)]
pub enum ResolvedResource {
    Folder(FolderDto),
    File(FileDto),
}

/// Resolves a WebDAV path to a folder or file in a single SQL round-trip.
pub struct PathResolverService {
    pool: Arc<PgPool>,
}

impl PathResolverService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Resolve `path` to a folder or file **owned by `user_id`**.
    ///
    /// Adds `AND fo.user_id = $4` / `AND fi.user_id = $4` so that one
    /// user can never resolve another user's resources.
    pub async fn resolve_path_for_user(
        &self,
        path: &str,
        user_id: Uuid,
    ) -> Result<ResolvedResource, DomainError> {
        let path = path.trim_start_matches('/').trim_end_matches('/');
        if path.is_empty() {
            return Err(DomainError::not_found("Resource", "empty path"));
        }

        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let filename = segments[segments.len() - 1];
        let folder_path = if segments.len() > 1 {
            segments[..segments.len() - 1].join("/")
        } else {
            String::new()
        };

        let row = sqlx::query_as::<
            _,
            (
                String,         // resource_type
                String,         // id
                String,         // name
                String,         // path
                Option<String>, // parent_id
                Option<String>, // user_id
                i64,            // created_at
                i64,            // modified_at
                Option<i64>,    // size
                Option<String>, // mime_type
                Option<String>, // folder_id
            ),
        >(
            r#"
            SELECT resource_type, id, name, path, parent_id, user_id,
                   created_at, modified_at, size, mime_type, folder_id
              FROM (
                SELECT 'folder'::text       AS resource_type,
                       fo.id::text,
                       fo.name,
                       fo.path,
                       fo.parent_id::text,
                       fo.user_id::text,
                       EXTRACT(EPOCH FROM fo.created_at)::bigint AS created_at,
                       EXTRACT(EPOCH FROM fo.updated_at)::bigint AS modified_at,
                       NULL::bigint         AS size,
                       NULL::text           AS mime_type,
                       NULL::text           AS folder_id
                  FROM storage.folders fo
                 WHERE fo.path = $1 AND NOT fo.is_trashed
                   AND fo.user_id = $4

                UNION ALL

                SELECT 'file'::text         AS resource_type,
                       fi.id::text,
                       fi.name,
                       CASE
                         WHEN fo.path IS NOT NULL AND fo.path != ''
                         THEN fo.path || '/' || fi.name
                         ELSE fi.name
                       END                  AS path,
                       NULL::text           AS parent_id,
                       fi.user_id::text,
                       EXTRACT(EPOCH FROM fi.created_at)::bigint AS created_at,
                       EXTRACT(EPOCH FROM fi.updated_at)::bigint AS modified_at,
                       fi.size,
                       fi.mime_type,
                       fi.folder_id::text
                  FROM storage.files fi
                  LEFT JOIN storage.folders fo ON fo.id = fi.folder_id
                 WHERE fi.name = $2
                   AND (
                         ($3 = '' AND fi.folder_id IS NULL)
                         OR fo.path = $3
                       )
                   AND NOT fi.is_trashed
                   AND fi.user_id = $4
              ) sub
             LIMIT 1
            "#,
        )
        .bind(path) // $1
        .bind(filename) // $2
        .bind(&folder_path) // $3
        .bind(user_id) // $4
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::internal_error("PathResolver", format!("resolve_for_user: {e}")))?
        .ok_or_else(|| DomainError::not_found("Resource", path))?;

        let (
            resource_type,
            id,
            name,
            res_path,
            parent_id,
            uid,
            created_at,
            modified_at,
            size,
            mime_type,
            folder_id,
        ) = row;

        match resource_type.as_str() {
            "folder" => Ok(ResolvedResource::Folder(FolderDto {
                id,
                name: name.clone(),
                path: res_path,
                parent_id,
                owner_id: uid,
                created_at: created_at as u64,
                modified_at: modified_at as u64,
                is_root: false,
                icon_class: Arc::from("fas fa-folder"),
                icon_special_class: Arc::from("folder-icon"),
                category: Arc::from("Folder"),
            })),
            _ => {
                let mime = mime_type.unwrap_or_else(|| "application/octet-stream".to_string());
                let sz = size.unwrap_or(0) as u64;
                Ok(ResolvedResource::File(FileDto {
                    id,
                    name: name.clone(),
                    path: res_path,
                    size: sz,
                    mime_type: Arc::from(&*mime),
                    folder_id,
                    created_at: created_at as u64,
                    modified_at: modified_at as u64,
                    icon_class: Arc::from(icon_class_for(&name, &mime)),
                    icon_special_class: Arc::from(icon_special_class_for(&name, &mime)),
                    category: Arc::from(category_for(&name, &mime)),
                    size_formatted: format_file_size(sz),
                    owner_id: uid,
                    sort_date: None,
                    etag: String::new(),
                }))
            }
        }
    }

    /// Returns `true` if the resource at `path` belongs to `user_id`.
    pub async fn exists_for_user(&self, path: &str, user_id: Uuid) -> Result<bool, DomainError> {
        let path = path.trim_start_matches('/').trim_end_matches('/');
        if path.is_empty() {
            return Ok(false);
        }

        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let filename = segments[segments.len() - 1];
        let folder_path = if segments.len() > 1 {
            segments[..segments.len() - 1].join("/")
        } else {
            String::new()
        };

        let exists = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
              SELECT 1 FROM storage.folders
               WHERE path = $1 AND NOT is_trashed AND user_id = $4
            ) OR EXISTS(
              SELECT 1
                FROM storage.files fi
                LEFT JOIN storage.folders fo ON fo.id = fi.folder_id
               WHERE fi.name = $2
                 AND (($3 = '' AND fi.folder_id IS NULL) OR fo.path = $3)
                 AND NOT fi.is_trashed
                 AND fi.user_id = $4
            )
            "#,
        )
        .bind(path)
        .bind(filename)
        .bind(&folder_path)
        .bind(user_id)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| {
            DomainError::internal_error("PathResolver", format!("exists_for_user: {e}"))
        })?;

        Ok(exists)
    }
}
