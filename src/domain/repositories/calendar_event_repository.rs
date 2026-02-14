use crate::common::errors::DomainError;
use crate::domain::entities::calendar_event::CalendarEvent;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub type CalendarEventRepositoryResult<T> = Result<T, DomainError>;

/// Repository interface for CalendarEvent entity operations
#[async_trait]
pub trait CalendarEventRepository: Send + Sync + 'static {
    /// Creates a new calendar event
    async fn create_event(
        &self,
        event: CalendarEvent,
    ) -> CalendarEventRepositoryResult<CalendarEvent>;

    /// Updates an existing calendar event
    async fn update_event(
        &self,
        event: CalendarEvent,
    ) -> CalendarEventRepositoryResult<CalendarEvent>;

    /// Deletes a calendar event by ID
    async fn delete_event(&self, id: &Uuid) -> CalendarEventRepositoryResult<()>;

    /// Finds a calendar event by its ID
    async fn find_event_by_id(&self, id: &Uuid) -> CalendarEventRepositoryResult<CalendarEvent>;

    /// Lists all events in a specific calendar
    async fn list_events_by_calendar(
        &self,
        calendar_id: &Uuid,
    ) -> CalendarEventRepositoryResult<Vec<CalendarEvent>>;

    /// Finds events in a calendar by their summary/title (partial match)
    async fn find_events_by_summary(
        &self,
        calendar_id: &Uuid,
        summary: &str,
    ) -> CalendarEventRepositoryResult<Vec<CalendarEvent>>;

    /// Gets events in a specific time range for a calendar
    async fn get_events_in_time_range(
        &self,
        calendar_id: &Uuid,
        start: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> CalendarEventRepositoryResult<Vec<CalendarEvent>>;

    /// Finds an event by its iCalendar UID in a specific calendar
    async fn find_event_by_ical_uid(
        &self,
        calendar_id: &Uuid,
        ical_uid: &str,
    ) -> CalendarEventRepositoryResult<Option<CalendarEvent>>;

    /// Counts events in a calendar
    async fn count_events_in_calendar(
        &self,
        calendar_id: &Uuid,
    ) -> CalendarEventRepositoryResult<i64>;

    /// Deletes all events in a calendar
    async fn delete_all_events_in_calendar(
        &self,
        calendar_id: &Uuid,
    ) -> CalendarEventRepositoryResult<i64>;

    /// Lists events by calendar with pagination
    async fn list_events_by_calendar_paginated(
        &self,
        calendar_id: &Uuid,
        limit: i64,
        offset: i64,
    ) -> CalendarEventRepositoryResult<Vec<CalendarEvent>>;

    /// Finds events with recurrence rules that might occur in a time range
    async fn find_recurring_events_in_range(
        &self,
        calendar_id: &Uuid,
        start: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> CalendarEventRepositoryResult<Vec<CalendarEvent>>;
}
