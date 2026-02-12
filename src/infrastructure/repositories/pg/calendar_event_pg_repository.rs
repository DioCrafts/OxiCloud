use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row, types::Uuid};
use std::sync::Arc;

use crate::domain::entities::calendar_event::CalendarEvent;
use crate::domain::repositories::calendar_event_repository::{CalendarEventRepository, CalendarEventRepositoryResult};
use crate::common::errors::DomainError;

pub struct CalendarEventPgRepository {
    pool: Arc<PgPool>,
}

impl CalendarEventPgRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CalendarEventRepository for CalendarEventPgRepository {
    async fn create_event(&self, event: CalendarEvent) -> CalendarEventRepositoryResult<CalendarEvent> {
        // This method would need a full implementation that builds the CalendarEvent
        // from the query result, using constructor methods
        // For this demonstration, we return the same event
        
        sqlx::query(
            r#"
            INSERT INTO caldav.calendar_events (
                id, calendar_id, summary, description, location, start_time, end_time, 
                all_day, rrule, created_at, updated_at, ical_uid, ical_data
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#
        )
        .bind(event.id())
        .bind(event.calendar_id())
        .bind(event.summary())
        .bind(event.description())
        .bind(event.location())
        .bind(event.start_time())
        .bind(event.end_time())
        .bind(event.all_day())
        .bind(event.rrule())
        .bind(event.created_at())
        .bind(event.updated_at())
        .bind(event.ical_uid())
        .bind(event.ical_data())
        .execute(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to create calendar event: {}", e)))?;

        // We return the same event instead of a result
        Ok(event)
    }

    async fn update_event(&self, event: CalendarEvent) -> CalendarEventRepositoryResult<CalendarEvent> {
        let now = Utc::now();
        
        sqlx::query(
            r#"
            UPDATE caldav.calendar_events
            SET summary = $1, 
                description = $2, 
                location = $3, 
                start_time = $4, 
                end_time = $5, 
                all_day = $6, 
                rrule = $7,
                ical_data = $8,
                updated_at = $9
            WHERE id = $10
            "#
        )
        .bind(event.summary())
        .bind(event.description())
        .bind(event.location())
        .bind(event.start_time())
        .bind(event.end_time())
        .bind(event.all_day())
        .bind(event.rrule())
        .bind(event.ical_data())
        .bind(now)
        .bind(event.id())
        .execute(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to update calendar event: {}", e)))?;

        // In a full implementation, we would retrieve the updated event
        // For simplicity, we return the same event we received
        Ok(event)
    }

    async fn delete_event(&self, id: &Uuid) -> CalendarEventRepositoryResult<()> {
        sqlx::query(
            r#"
            DELETE FROM caldav.calendar_events
            WHERE id = $1
            "#
        )
        .bind(id)
        .execute(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to delete calendar event: {}", e)))?;

        Ok(())
    }

    async fn get_events_in_time_range(
        &self, 
        calendar_id: &Uuid, 
        start: &DateTime<Utc>, 
        end: &DateTime<Utc>
    ) -> CalendarEventRepositoryResult<Vec<CalendarEvent>> {
        let rows = sqlx::query(
            r#"
            SELECT 
                id, calendar_id, summary, description, location, 
                start_time, end_time, all_day, rrule, 
                created_at, updated_at, ical_uid, ical_data
            FROM caldav.calendar_events
            WHERE calendar_id = $1 
              AND (
                  (start_time >= $2 AND start_time < $3) OR
                  (end_time > $2 AND end_time <= $3) OR
                  (start_time <= $2 AND end_time >= $3) OR
                  (rrule IS NOT NULL AND end_time >= $2)
              )
            ORDER BY start_time
            "#
        )
        .bind(calendar_id)
        .bind(start)
        .bind(end)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to get events in time range: {}", e)))?;

        let mut events = Vec::new();
        for row in rows {
            let event = CalendarEvent::with_id(
                row.get("id"),
                row.get("calendar_id"),
                row.get("summary"),
                row.get::<Option<String>, _>("description"),
                row.get::<Option<String>, _>("location"),
                row.get("start_time"),
                row.get("end_time"),
                row.get("all_day"),
                row.get::<Option<String>, _>("rrule"),
                row.get("ical_uid"),
                row.get("ical_data"),
                row.get("created_at"),
                row.get("updated_at"),
            ).map_err(|e| DomainError::database_error(format!("Error creating calendar event: {}", e)))?;
            events.push(event);
        }
        
        Ok(events)
    }

    async fn find_event_by_id(&self, id: &Uuid) -> CalendarEventRepositoryResult<CalendarEvent> {
        let row = sqlx::query(
            r#"
            SELECT 
                id, calendar_id, summary, description, location, 
                start_time, end_time, all_day, rrule, 
                created_at, updated_at, ical_uid, ical_data
            FROM caldav.calendar_events
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to get calendar event by id: {}", e)))?
        .ok_or_else(|| DomainError::not_found("Calendar Event", id.to_string()))?;

        // In a real implementation, we would build a complete CalendarEvent object
        // For simplicity, we create an object with default values to
        // demonstrate the approach without macros
        
        let event = CalendarEvent::with_id(
            row.get("id"),
            row.get("calendar_id"),
            row.get("summary"),
            row.get::<Option<String>, _>("description"),
            row.get::<Option<String>, _>("location"),
            row.get("start_time"),
            row.get("end_time"),
            row.get("all_day"),
            row.get::<Option<String>, _>("rrule"),
            row.get("ical_uid"),
            row.get("ical_data"),
            row.get("created_at"),
            row.get("updated_at")
        ).map_err(|e| DomainError::database_error(format!("Error creating calendar event: {}", e)))?;
        
        Ok(event)
    }
    
    async fn list_events_by_calendar(&self, calendar_id: &Uuid) -> CalendarEventRepositoryResult<Vec<CalendarEvent>> {
        let rows = sqlx::query(
            r#"
            SELECT 
                id, calendar_id, summary, description, location, 
                start_time, end_time, all_day, rrule, 
                created_at, updated_at, ical_uid, ical_data
            FROM caldav.calendar_events
            WHERE calendar_id = $1
            ORDER BY start_time
            "#
        )
        .bind(calendar_id)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to get events by calendar: {}", e)))?;

        let mut events = Vec::new();
        for row in rows {
            let event = CalendarEvent::with_id(
                row.get("id"),
                row.get("calendar_id"),
                row.get("summary"),
                row.get::<Option<String>, _>("description"),
                row.get::<Option<String>, _>("location"),
                row.get("start_time"),
                row.get("end_time"),
                row.get("all_day"),
                row.get::<Option<String>, _>("rrule"),
                row.get("ical_uid"),
                row.get("ical_data"),
                row.get("created_at"),
                row.get("updated_at"),
            ).map_err(|e| DomainError::database_error(format!("Error creating calendar event: {}", e)))?;
            events.push(event);
        }
        
        Ok(events)
    }
    
    async fn find_events_by_summary(&self, calendar_id: &Uuid, summary: &str) -> CalendarEventRepositoryResult<Vec<CalendarEvent>> {
        let search_pattern = format!("%{}%", summary);
        
        let rows = sqlx::query(
            r#"
            SELECT 
                id, calendar_id, summary, description, location, 
                start_time, end_time, all_day, rrule, 
                created_at, updated_at, ical_uid, ical_data
            FROM caldav.calendar_events
            WHERE calendar_id = $1 AND summary ILIKE $2
            ORDER BY start_time
            "#
        )
        .bind(calendar_id)
        .bind(&search_pattern)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to find events by summary: {}", e)))?;

        let mut events = Vec::new();
        for row in rows {
            let event = CalendarEvent::with_id(
                row.get("id"),
                row.get("calendar_id"),
                row.get("summary"),
                row.get::<Option<String>, _>("description"),
                row.get::<Option<String>, _>("location"),
                row.get("start_time"),
                row.get("end_time"),
                row.get("all_day"),
                row.get::<Option<String>, _>("rrule"),
                row.get("ical_uid"),
                row.get("ical_data"),
                row.get("created_at"),
                row.get("updated_at"),
            ).map_err(|e| DomainError::database_error(format!("Error creating calendar event: {}", e)))?;
            events.push(event);
        }
        
        Ok(events)
    }
    
    async fn find_event_by_ical_uid(&self, calendar_id: &Uuid, ical_uid: &str) -> CalendarEventRepositoryResult<Option<CalendarEvent>> {
        let row_opt = sqlx::query(
            r#"
            SELECT 
                id, calendar_id, summary, description, location, 
                start_time, end_time, all_day, rrule, 
                created_at, updated_at, ical_uid, ical_data
            FROM caldav.calendar_events
            WHERE calendar_id = $1 AND ical_uid = $2
            "#
        )
        .bind(calendar_id)
        .bind(ical_uid)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to get calendar event by UID: {}", e)))?;

        match row_opt {
            Some(row) => {
                let event = CalendarEvent::with_id(
                    row.get("id"),
                    row.get("calendar_id"),
                    row.get("summary"),
                    row.get::<Option<String>, _>("description"),
                    row.get::<Option<String>, _>("location"),
                    row.get("start_time"),
                    row.get("end_time"),
                    row.get("all_day"),
                    row.get::<Option<String>, _>("rrule"),
                    row.get("ical_uid"),
                    row.get("ical_data"),
                    row.get("created_at"),
                    row.get("updated_at"),
                ).map_err(|e| DomainError::database_error(format!("Error creating calendar event: {}", e)))?;
                Ok(Some(event))
            }
            None => Ok(None),
        }
    }
    
    async fn count_events_in_calendar(&self, calendar_id: &Uuid) -> CalendarEventRepositoryResult<i64> {
        let row = sqlx::query(
            r#"
            SELECT COUNT(*) as count
            FROM caldav.calendar_events
            WHERE calendar_id = $1
            "#
        )
        .bind(calendar_id)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to count events in calendar: {}", e)))?;

        Ok(row.get::<i64, _>("count"))
    }
    
    async fn delete_all_events_in_calendar(&self, calendar_id: &Uuid) -> CalendarEventRepositoryResult<i64> {
        let result = sqlx::query(
            r#"
            DELETE FROM caldav.calendar_events
            WHERE calendar_id = $1
            "#
        )
        .bind(calendar_id)
        .execute(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to delete all events in calendar: {}", e)))?;

        Ok(result.rows_affected() as i64)
    }
    
    async fn list_events_by_calendar_paginated(
        &self, 
        calendar_id: &Uuid,
        limit: i64,
        offset: i64
    ) -> CalendarEventRepositoryResult<Vec<CalendarEvent>> {
        let rows = sqlx::query(
            r#"
            SELECT 
                id, calendar_id, summary, description, location, 
                start_time, end_time, all_day, rrule, 
                created_at, updated_at, ical_uid, ical_data
            FROM caldav.calendar_events
            WHERE calendar_id = $1
            ORDER BY start_time
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(calendar_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to get paginated events by calendar: {}", e)))?;

        let mut events = Vec::new();
        for row in rows {
            let event = CalendarEvent::with_id(
                row.get("id"),
                row.get("calendar_id"),
                row.get("summary"),
                row.get::<Option<String>, _>("description"),
                row.get::<Option<String>, _>("location"),
                row.get("start_time"),
                row.get("end_time"),
                row.get("all_day"),
                row.get::<Option<String>, _>("rrule"),
                row.get("ical_uid"),
                row.get("ical_data"),
                row.get("created_at"),
                row.get("updated_at"),
            ).map_err(|e| DomainError::database_error(format!("Error creating calendar event: {}", e)))?;
            events.push(event);
        }
        
        Ok(events)
    }
    
    async fn find_recurring_events_in_range(
        &self,
        calendar_id: &Uuid,
        start: &DateTime<Utc>,
        end: &DateTime<Utc>
    ) -> CalendarEventRepositoryResult<Vec<CalendarEvent>> {
        let rows = sqlx::query(
            r#"
            SELECT 
                id, calendar_id, summary, description, location, 
                start_time, end_time, all_day, rrule, 
                created_at, updated_at, ical_uid, ical_data
            FROM caldav.calendar_events
            WHERE calendar_id = $1 
              AND rrule IS NOT NULL
              AND end_time >= $2
              AND start_time <= $3
            ORDER BY start_time
            "#
        )
        .bind(calendar_id)
        .bind(start)
        .bind(end)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to find recurring events in range: {}", e)))?;

        let mut events = Vec::new();
        for row in rows {
            let event = CalendarEvent::with_id(
                row.get("id"),
                row.get("calendar_id"),
                row.get("summary"),
                row.get::<Option<String>, _>("description"),
                row.get::<Option<String>, _>("location"),
                row.get("start_time"),
                row.get("end_time"),
                row.get("all_day"),
                row.get::<Option<String>, _>("rrule"),
                row.get("ical_uid"),
                row.get("ical_data"),
                row.get("created_at"),
                row.get("updated_at"),
            ).map_err(|e| DomainError::database_error(format!("Error creating calendar event: {}", e)))?;
            events.push(event);
        }
        
        Ok(events)
    }
}