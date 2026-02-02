use async_trait::async_trait;
use chrono::Utc;
use sqlx::{PgPool, Row, types::Uuid};
use std::sync::Arc;

use crate::domain::entities::calendar::Calendar;
use crate::domain::repositories::calendar_repository::{CalendarRepository, CalendarRepositoryResult};
use crate::common::errors::DomainError;

pub struct CalendarPgRepository {
    pool: Arc<PgPool>,
}

impl CalendarPgRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CalendarRepository for CalendarPgRepository {
    async fn create_calendar(&self, calendar: Calendar) -> CalendarRepositoryResult<Calendar> {
        let row = sqlx::query(
            r#"
            INSERT INTO caldav.calendars (id, name, owner_id, description, color, is_public, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, name, owner_id, description, color, is_public, created_at, updated_at
            "#
        )
        .bind(calendar.id())
        .bind(calendar.name())
        .bind(calendar.owner_id())
        .bind(calendar.description())
        .bind(calendar.color())
        .bind(false) // is_public no existe como campo
        .bind(calendar.created_at())
        .bind(calendar.updated_at())
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to create calendar: {}", e)))?;
        
        // Construir el objeto Calendar utilizando su constructor with_id
        let result = Calendar::with_id(
            row.get("id"),
            row.get("name"),
            row.get("owner_id"),
            row.get("description"),
            row.get("color"),
            row.get("created_at"),
            row.get("updated_at"),
        ).map_err(|e| DomainError::database_error(format!("Failed to create calendar object: {}", e)))?;

        Ok(result)
    }

    async fn update_calendar(&self, calendar: Calendar) -> CalendarRepositoryResult<Calendar> {
        let now = Utc::now();
        let row = sqlx::query(
            r#"
            UPDATE caldav.calendars
            SET name = $1, description = $2, color = $3, is_public = $4, updated_at = $5
            WHERE id = $6
            RETURNING id, name, owner_id, description, color, is_public, created_at, updated_at
            "#
        )
        .bind(calendar.name())
        .bind(calendar.description())
        .bind(calendar.color())
        .bind(false) // is_public no existe como campo
        .bind(now)
        .bind(calendar.id())
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to update calendar: {}", e)))?;
        
        // Construir el objeto Calendar utilizando su constructor with_id
        let result = Calendar::with_id(
            row.get("id"),
            row.get("name"),
            row.get("owner_id"),
            row.get("description"),
            row.get("color"),
            row.get("created_at"),
            row.get("updated_at"),
        ).map_err(|e| DomainError::database_error(format!("Failed to create calendar object: {}", e)))?;

        Ok(result)
    }

    async fn delete_calendar(&self, id: &Uuid) -> CalendarRepositoryResult<()> {
        sqlx::query(
            r#"
            DELETE FROM caldav.calendars
            WHERE id = $1
            "#
        )
        .bind(id)
        .execute(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to delete calendar: {}", e)))?;

        Ok(())
    }

    async fn find_calendar_by_id(&self, id: &Uuid) -> CalendarRepositoryResult<Calendar> {
        let row = sqlx::query(
            r#"
            SELECT id, name, owner_id, description, color, is_public, created_at, updated_at
            FROM caldav.calendars
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to get calendar by id: {}", e)))?
        .ok_or_else(|| DomainError::not_found("Calendar", id.to_string()))?;

        let calendar = Calendar::with_id(
            row.get("id"),
            row.get("name"),
            row.get("owner_id"),
            row.get("description"),
            row.get("color"),
            row.get("created_at"),
            row.get("updated_at"),
        ).map_err(|e| DomainError::database_error(format!("Failed to create calendar object: {}", e)))?;

        Ok(calendar)
    }

    async fn list_calendars_by_owner(&self, owner_id: &str) -> CalendarRepositoryResult<Vec<Calendar>> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, owner_id, description, color, is_public, created_at, updated_at
            FROM caldav.calendars
            WHERE owner_id = $1
            ORDER BY name
            "#
        )
        .bind(owner_id)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to get calendars by owner: {}", e)))?;

        let mut calendars = Vec::new();
        for row in rows {
            let calendar = Calendar::with_id(
                row.get("id"),
                row.get("name"),
                row.get("owner_id"),
                row.get("description"),
                row.get("color"),
                row.get("created_at"),
                row.get("updated_at"),
            ).map_err(|e| DomainError::database_error(format!("Failed to create calendar object: {}", e)))?;
            calendars.push(calendar);
        }

        Ok(calendars)
    }

    async fn find_calendar_by_name_and_owner(&self, name: &str, owner_id: &str) -> CalendarRepositoryResult<Calendar> {
        let row = sqlx::query(
            r#"
            SELECT id, name, owner_id, description, color, is_public, created_at, updated_at
            FROM caldav.calendars
            WHERE name = $1 AND owner_id = $2
            "#
        )
        .bind(name)
        .bind(owner_id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to find calendar by name and owner: {}", e)))?
        .ok_or_else(|| DomainError::not_found("Calendar", format!("{} (owned by {})", name, owner_id)))?;

        let calendar = Calendar::with_id(
            row.get("id"),
            row.get("name"),
            row.get("owner_id"),
            row.get("description"),
            row.get("color"),
            row.get("created_at"),
            row.get("updated_at"),
        ).map_err(|e| DomainError::database_error(format!("Failed to create calendar object: {}", e)))?;

        Ok(calendar)
    }

    async fn list_calendars_shared_with_user(&self, user_id: &str) -> CalendarRepositoryResult<Vec<Calendar>> {
        let rows = sqlx::query(
            r#"
            SELECT c.id, c.name, c.owner_id, c.description, c.color, c.is_public, c.created_at, c.updated_at
            FROM caldav.calendars c
            INNER JOIN caldav.calendar_shares s ON c.id = s.calendar_id
            WHERE s.user_id = $1
            ORDER BY c.name
            "#
        )
        .bind(user_id)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to get shared calendars: {}", e)))?;

        let mut calendars = Vec::new();
        for row in rows {
            let calendar = Calendar::with_id(
                row.get("id"),
                row.get("name"),
                row.get("owner_id"),
                row.get("description"),
                row.get("color"),
                row.get("created_at"),
                row.get("updated_at"),
            ).map_err(|e| DomainError::database_error(format!("Failed to create calendar object: {}", e)))?;
            calendars.push(calendar);
        }

        Ok(calendars)
    }

    async fn list_public_calendars(&self, limit: i64, offset: i64) -> CalendarRepositoryResult<Vec<Calendar>> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, owner_id, description, color, is_public, created_at, updated_at
            FROM caldav.calendars
            WHERE is_public = true
            ORDER BY name
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to get public calendars: {}", e)))?;

        let mut calendars = Vec::new();
        for row in rows {
            let calendar = Calendar::with_id(
                row.get("id"),
                row.get("name"),
                row.get("owner_id"),
                row.get("description"),
                row.get("color"), 
                row.get("created_at"),
                row.get("updated_at"),
            ).map_err(|e| DomainError::database_error(format!("Failed to create calendar object: {}", e)))?;
            calendars.push(calendar);
        }

        Ok(calendars)
    }

    async fn user_has_calendar_access(&self, calendar_id: &Uuid, user_id: &str) -> CalendarRepositoryResult<bool> {
        // Check if the user is the owner of the calendar or has a share
        let row = sqlx::query(
            r#"
            SELECT EXISTS (
                SELECT 1 FROM caldav.calendars c
                WHERE c.id = $1 AND (c.owner_id = $2 OR c.is_public = true)
                UNION
                SELECT 1 FROM caldav.calendar_shares s
                WHERE s.calendar_id = $1 AND s.user_id = $2
            ) as has_access
            "#
        )
        .bind(calendar_id)
        .bind(user_id)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to check calendar access: {}", e)))?;

        Ok(row.get::<bool, _>("has_access"))
    }

    async fn share_calendar(&self, calendar_id: &Uuid, user_id: &str, access_level: &str) -> CalendarRepositoryResult<()> {
        // Validate access level
        if !["read", "write", "owner"].contains(&access_level) {
            return Err(DomainError::validation_error(
                format!("Invalid access level: '{}'. Must be 'read', 'write', or 'owner'", access_level)
            ));
        }
        
        sqlx::query(
            r#"
            INSERT INTO caldav.calendar_shares (calendar_id, user_id, access_level)
            VALUES ($1, $2, $3)
            ON CONFLICT (calendar_id, user_id) DO UPDATE SET access_level = $3
            "#
        )
        .bind(calendar_id)
        .bind(user_id)
        .bind(access_level)
        .execute(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to share calendar: {}", e)))?;

        Ok(())
    }

    async fn remove_calendar_sharing(&self, calendar_id: &Uuid, user_id: &str) -> CalendarRepositoryResult<()> {
        sqlx::query(
            r#"
            DELETE FROM caldav.calendar_shares
            WHERE calendar_id = $1 AND user_id = $2
            "#
        )
        .bind(calendar_id)
        .bind(user_id)
        .execute(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to unshare calendar: {}", e)))?;

        Ok(())
    }

    async fn get_calendar_shares(&self, calendar_id: &Uuid) -> CalendarRepositoryResult<Vec<(String, String)>> {
        let rows = sqlx::query(
            r#"
            SELECT user_id, access_level
            FROM caldav.calendar_shares
            WHERE calendar_id = $1
            ORDER BY user_id
            "#
        )
        .bind(calendar_id)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to get calendar shares: {}", e)))?;

        let mut shares = Vec::new();
        for row in rows {
            shares.push((row.get("user_id"), row.get("access_level")));
        }

        Ok(shares)
    }
    
    async fn get_calendar_property(&self, calendar_id: &Uuid, property_name: &str) -> CalendarRepositoryResult<Option<String>> {
        let row = sqlx::query(
            r#"
            SELECT value
            FROM caldav.calendar_properties
            WHERE calendar_id = $1 AND name = $2
            "#
        )
        .bind(calendar_id)
        .bind(property_name)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to get calendar property: {}", e)))?;
        
        Ok(row.map(|r| r.get("value")))
    }
    
    async fn set_calendar_property(&self, calendar_id: &Uuid, property_name: &str, property_value: &str) -> CalendarRepositoryResult<()> {
        sqlx::query(
            r#"
            INSERT INTO caldav.calendar_properties (calendar_id, name, value)
            VALUES ($1, $2, $3)
            ON CONFLICT (calendar_id, name) DO UPDATE SET value = $3
            "#
        )
        .bind(calendar_id)
        .bind(property_name)
        .bind(property_value)
        .execute(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to set calendar property: {}", e)))?;
        
        Ok(())
    }
    
    async fn remove_calendar_property(&self, calendar_id: &Uuid, property_name: &str) -> CalendarRepositoryResult<()> {
        sqlx::query(
            r#"
            DELETE FROM caldav.calendar_properties
            WHERE calendar_id = $1 AND name = $2
            "#
        )
        .bind(calendar_id)
        .bind(property_name)
        .execute(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to remove calendar property: {}", e)))?;
        
        Ok(())
    }
    
    async fn get_calendar_properties(&self, calendar_id: &Uuid) -> CalendarRepositoryResult<std::collections::HashMap<String, String>> {
        let rows = sqlx::query(
            r#"
            SELECT name, value
            FROM caldav.calendar_properties
            WHERE calendar_id = $1
            "#
        )
        .bind(calendar_id)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to get calendar properties: {}", e)))?;
        
        let mut properties = std::collections::HashMap::new();
        for row in rows {
            properties.insert(row.get("name"), row.get("value"));
        }
        
        Ok(properties)
    }
}