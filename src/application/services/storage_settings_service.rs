use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::application::dtos::settings_dto::{
    SaveStorageSettingsDto, StorageSettingsDto, StorageTestResultDto, TestStorageConnectionDto,
};
use crate::application::ports::blob_storage_ports::BlobStorageBackend;
use crate::common::config::{S3StorageConfig, StorageConfig};
use crate::common::errors::{DomainError, ErrorKind};
use crate::domain::repositories::settings_repository::SettingsRepository;
use crate::infrastructure::repositories::pg::SettingsPgRepository;
use crate::infrastructure::services::dedup_service::DedupService;
use crate::infrastructure::services::s3_blob_backend::S3BlobBackend;

/// Storage settings service — manages storage backend configuration via the admin panel.
///
/// Configuration priority: **env vars > DB settings > defaults**.
pub struct StorageSettingsService {
    settings_repo: Arc<SettingsPgRepository>,
    env_storage_config: StorageConfig,
    dedup_service: Arc<DedupService>,
}

impl StorageSettingsService {
    pub fn new(
        settings_repo: Arc<SettingsPgRepository>,
        env_storage_config: StorageConfig,
        dedup_service: Arc<DedupService>,
    ) -> Self {
        Self {
            settings_repo,
            env_storage_config,
            dedup_service,
        }
    }

    /// Detect which storage fields are overridden by environment variables.
    fn get_env_overrides(&self) -> Vec<String> {
        let mut out = Vec::new();
        let vars = [
            ("OXICLOUD_STORAGE_BACKEND", "backend"),
            ("OXICLOUD_S3_ENDPOINT_URL", "s3_endpoint_url"),
            ("OXICLOUD_S3_BUCKET", "s3_bucket"),
            ("OXICLOUD_S3_REGION", "s3_region"),
            ("OXICLOUD_S3_ACCESS_KEY", "s3_access_key"),
            ("OXICLOUD_S3_SECRET_KEY", "s3_secret_key"),
            ("OXICLOUD_S3_FORCE_PATH_STYLE", "s3_force_path_style"),
        ];
        for (env_key, field_name) in &vars {
            if std::env::var(env_key).is_ok() {
                out.push(field_name.to_string());
            }
        }
        out
    }

    /// Apply environment variable overrides on top of a config.
    fn apply_env_overrides(&self, config: &mut StorageConfig) {
        let e = &self.env_storage_config;
        if std::env::var("OXICLOUD_STORAGE_BACKEND").is_ok() {
            config.backend = e.backend.clone();
        }
        // S3 env overrides — only apply if S3 config exists in env
        if let Some(env_s3) = &e.s3 {
            let s3 = config.s3.get_or_insert_with(|| S3StorageConfig {
                endpoint_url: None,
                bucket: String::new(),
                region: "us-east-1".to_string(),
                access_key: String::new(),
                secret_key: String::new(),
                force_path_style: false,
            });
            if std::env::var("OXICLOUD_S3_ENDPOINT_URL").is_ok() {
                s3.endpoint_url = env_s3.endpoint_url.clone();
            }
            if std::env::var("OXICLOUD_S3_BUCKET").is_ok() {
                s3.bucket = env_s3.bucket.clone();
            }
            if std::env::var("OXICLOUD_S3_REGION").is_ok() {
                s3.region = env_s3.region.clone();
            }
            if std::env::var("OXICLOUD_S3_ACCESS_KEY").is_ok() {
                s3.access_key = env_s3.access_key.clone();
            }
            if std::env::var("OXICLOUD_S3_SECRET_KEY").is_ok() {
                s3.secret_key = env_s3.secret_key.clone();
            }
            if std::env::var("OXICLOUD_S3_FORCE_PATH_STYLE").is_ok() {
                s3.force_path_style = env_s3.force_path_style;
            }
        }
    }

    /// Load effective storage config: DB settings + env var overrides + defaults.
    pub async fn load_effective_storage_config(&self) -> Result<StorageConfig, DomainError> {
        let db: HashMap<String, String> = self.settings_repo.get_by_category("storage").await?;
        let d = StorageConfig::default();

        let backend = db
            .get("storage.backend")
            .map(|v| match v.as_str() {
                "s3" => crate::common::config::StorageBackendType::S3,
                "azure" => crate::common::config::StorageBackendType::Azure,
                _ => crate::common::config::StorageBackendType::Local,
            })
            .unwrap_or(d.backend);

        let s3 = {
            let bucket = db.get("storage.s3.bucket").cloned().unwrap_or_default();
            if bucket.is_empty() {
                None
            } else {
                Some(S3StorageConfig {
                    endpoint_url: db
                        .get("storage.s3.endpoint_url")
                        .cloned()
                        .filter(|s| !s.is_empty()),
                    bucket,
                    region: db
                        .get("storage.s3.region")
                        .cloned()
                        .unwrap_or_else(|| "us-east-1".to_string()),
                    access_key: db.get("storage.s3.access_key").cloned().unwrap_or_default(),
                    secret_key: db.get("storage.s3.secret_key").cloned().unwrap_or_default(),
                    force_path_style: db
                        .get("storage.s3.force_path_style")
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(false),
                })
            }
        };

        let mut config = StorageConfig {
            backend,
            s3,
            ..self.env_storage_config.clone()
        };

        self.apply_env_overrides(&mut config);
        Ok(config)
    }

    /// Get storage settings for display in admin UI (secrets masked).
    pub async fn get_storage_settings(&self) -> Result<StorageSettingsDto, DomainError> {
        let db: HashMap<String, String> = self.settings_repo.get_by_category("storage").await?;

        let has_access_key = db
            .get("storage.s3.access_key")
            .map(|s| !s.is_empty())
            .unwrap_or(false)
            || std::env::var("OXICLOUD_S3_ACCESS_KEY")
                .map(|s| !s.is_empty())
                .unwrap_or(false);

        let has_secret_key = db
            .get("storage.s3.secret_key")
            .map(|s| !s.is_empty())
            .unwrap_or(false)
            || std::env::var("OXICLOUD_S3_SECRET_KEY")
                .map(|s| !s.is_empty())
                .unwrap_or(false);

        let effective = self.load_effective_storage_config().await?;
        let stats = self.dedup_service.get_stats().await;
        let current_backend = self.dedup_service.backend().backend_type().to_string();

        let backend_str = match effective.backend {
            crate::common::config::StorageBackendType::Local => "local",
            crate::common::config::StorageBackendType::S3 => "s3",
            crate::common::config::StorageBackendType::Azure => "azure",
        };

        Ok(StorageSettingsDto {
            backend: backend_str.to_string(),
            s3_endpoint_url: effective.s3.as_ref().and_then(|s| s.endpoint_url.clone()),
            s3_bucket: effective.s3.as_ref().map(|s| s.bucket.clone()),
            s3_region: effective.s3.as_ref().map(|s| s.region.clone()),
            s3_access_key_set: has_access_key,
            s3_secret_key_set: has_secret_key,
            s3_force_path_style: effective.s3.as_ref().is_some_and(|s| s.force_path_style),
            env_overrides: self.get_env_overrides(),
            current_backend,
            total_blobs: stats.total_blobs,
            total_bytes_stored: stats.total_bytes_stored,
            dedup_ratio: stats.dedup_ratio,
        })
    }

    /// Save storage settings to DB.
    pub async fn save_storage_settings(
        &self,
        dto: SaveStorageSettingsDto,
        updated_by: Uuid,
    ) -> Result<(), DomainError> {
        let cat = "storage";
        let by = Some(updated_by);

        self.settings_repo
            .set("storage.backend", &dto.backend, cat, false, by)
            .await?;

        if let Some(ref v) = dto.s3_endpoint_url {
            self.settings_repo
                .set("storage.s3.endpoint_url", v, cat, false, by)
                .await?;
        }
        if let Some(ref v) = dto.s3_bucket {
            self.settings_repo
                .set("storage.s3.bucket", v, cat, false, by)
                .await?;
        }
        if let Some(ref v) = dto.s3_region {
            self.settings_repo
                .set("storage.s3.region", v, cat, false, by)
                .await?;
        }
        if let Some(ref v) = dto.s3_access_key
            && !v.is_empty()
        {
            self.settings_repo
                .set("storage.s3.access_key", v, cat, true, by)
                .await?;
        }
        if let Some(ref v) = dto.s3_secret_key
            && !v.is_empty()
        {
            self.settings_repo
                .set("storage.s3.secret_key", v, cat, true, by)
                .await?;
        }
        if let Some(v) = dto.s3_force_path_style {
            self.settings_repo
                .set(
                    "storage.s3.force_path_style",
                    &v.to_string(),
                    cat,
                    false,
                    by,
                )
                .await?;
        }

        tracing::info!("Storage settings saved by admin (backend={})", dto.backend);
        Ok(())
    }

    /// Test a storage connection by building a temporary backend and calling health_check().
    pub async fn test_storage_connection(
        &self,
        dto: TestStorageConnectionDto,
    ) -> Result<StorageTestResultDto, DomainError> {
        match dto.backend.as_str() {
            "local" => {
                // Test local backend health via the current dedup service backend
                let status = self.dedup_service.backend().health_check().await?;
                Ok(StorageTestResultDto {
                    connected: status.connected,
                    message: status.message,
                    backend_type: "local".to_string(),
                    available_bytes: status.available_bytes,
                })
            }
            "s3" => {
                let bucket = dto.s3_bucket.as_deref().unwrap_or_default();
                if bucket.is_empty() {
                    return Ok(StorageTestResultDto {
                        connected: false,
                        message: "S3 bucket name is required".to_string(),
                        backend_type: "s3".to_string(),
                        available_bytes: None,
                    });
                }

                // Build a temporary S3 backend from the DTO values,
                // falling back to existing DB/env config for missing fields.
                let effective = self.load_effective_storage_config().await.ok();
                let existing_s3 = effective.as_ref().and_then(|c| c.s3.as_ref());

                let config = S3StorageConfig {
                    endpoint_url: dto
                        .s3_endpoint_url
                        .clone()
                        .or_else(|| existing_s3.and_then(|s| s.endpoint_url.clone())),
                    bucket: bucket.to_string(),
                    region: dto.s3_region.clone().unwrap_or_else(|| {
                        existing_s3
                            .map(|s| s.region.clone())
                            .unwrap_or_else(|| "us-east-1".to_string())
                    }),
                    access_key: dto
                        .s3_access_key
                        .clone()
                        .filter(|s| !s.is_empty())
                        .unwrap_or_else(|| {
                            existing_s3
                                .map(|s| s.access_key.clone())
                                .unwrap_or_default()
                        }),
                    secret_key: dto
                        .s3_secret_key
                        .clone()
                        .filter(|s| !s.is_empty())
                        .unwrap_or_else(|| {
                            existing_s3
                                .map(|s| s.secret_key.clone())
                                .unwrap_or_default()
                        }),
                    force_path_style: dto
                        .s3_force_path_style
                        .unwrap_or_else(|| existing_s3.is_some_and(|s| s.force_path_style)),
                };

                let backend = S3BlobBackend::new(&config);
                match backend.health_check().await {
                    Ok(status) => Ok(StorageTestResultDto {
                        connected: status.connected,
                        message: status.message,
                        backend_type: "s3".to_string(),
                        available_bytes: status.available_bytes,
                    }),
                    Err(e) => Ok(StorageTestResultDto {
                        connected: false,
                        message: format!("Connection failed: {}", e),
                        backend_type: "s3".to_string(),
                        available_bytes: None,
                    }),
                }
            }
            other => Err(DomainError::new(
                ErrorKind::InvalidInput,
                "Storage",
                format!("Unknown backend type: {}", other),
            )),
        }
    }
}
