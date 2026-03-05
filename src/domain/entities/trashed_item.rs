use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum TrashedItemType {
    File,
    Folder,
}

#[derive(Debug, Clone)]
pub struct TrashedItem {
    id: Uuid,
    original_id: Uuid,
    user_id: Uuid,
    item_type: TrashedItemType,
    name: String,
    original_path: String,
    trashed_at: DateTime<Utc>,
    deletion_date: DateTime<Utc>,
}

impl TrashedItem {
    pub fn new(
        original_id: Uuid,
        user_id: Uuid,
        item_type: TrashedItemType,
        name: String,
        original_path: String,
        retention_days: u32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            original_id,
            user_id,
            item_type,
            name,
            original_path,
            trashed_at: now,
            deletion_date: now + chrono::Duration::days(retention_days as i64),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_raw(
        id: Uuid,
        original_id: Uuid,
        user_id: Uuid,
        item_type: TrashedItemType,
        name: String,
        original_path: String,
        trashed_at: DateTime<Utc>,
        deletion_date: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            original_id,
            user_id,
            item_type,
            name,
            original_path,
            trashed_at,
            deletion_date,
        }
    }

    // ── Getters ──

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn original_id(&self) -> Uuid {
        self.original_id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn item_type(&self) -> &TrashedItemType {
        &self.item_type
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn original_path(&self) -> &str {
        &self.original_path
    }

    pub fn trashed_at(&self) -> DateTime<Utc> {
        self.trashed_at
    }

    pub fn deletion_date(&self) -> DateTime<Utc> {
        self.deletion_date
    }

    pub fn days_until_deletion(&self) -> i64 {
        let now = Utc::now();
        (self.deletion_date - now).num_days().max(0)
    }
}
