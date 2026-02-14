use crate::application::dtos::calendar_dto::{
    CalendarDto, CalendarEventDto, CreateCalendarDto, CreateEventDto, CreateEventICalDto,
    UpdateCalendarDto, UpdateEventDto,
};
use crate::common::errors::DomainError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

/// Port for external calendar storage mechanisms
#[async_trait]
pub trait CalendarStoragePort: Send + Sync + 'static {
    // Calendar operations
    async fn create_calendar(
        &self,
        calendar: CreateCalendarDto,
        owner_id: &str,
    ) -> Result<CalendarDto, DomainError>;
    async fn update_calendar(
        &self,
        calendar_id: &str,
        update: UpdateCalendarDto,
    ) -> Result<CalendarDto, DomainError>;
    async fn delete_calendar(&self, calendar_id: &str) -> Result<(), DomainError>;
    async fn get_calendar(&self, calendar_id: &str) -> Result<CalendarDto, DomainError>;
    async fn list_calendars_by_owner(
        &self,
        owner_id: &str,
    ) -> Result<Vec<CalendarDto>, DomainError>;
    async fn list_calendars_shared_with_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<CalendarDto>, DomainError>;
    async fn list_public_calendars(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<CalendarDto>, DomainError>;
    async fn check_calendar_access(
        &self,
        calendar_id: &str,
        user_id: &str,
    ) -> Result<bool, DomainError>;

    // Calendar sharing
    async fn share_calendar(
        &self,
        calendar_id: &str,
        user_id: &str,
        access_level: &str,
    ) -> Result<(), DomainError>;
    async fn remove_calendar_sharing(
        &self,
        calendar_id: &str,
        user_id: &str,
    ) -> Result<(), DomainError>;
    async fn get_calendar_shares(
        &self,
        calendar_id: &str,
    ) -> Result<Vec<(String, String)>, DomainError>;

    // Calendar properties
    async fn set_calendar_property(
        &self,
        calendar_id: &str,
        property_name: &str,
        property_value: &str,
    ) -> Result<(), DomainError>;
    async fn get_calendar_property(
        &self,
        calendar_id: &str,
        property_name: &str,
    ) -> Result<Option<String>, DomainError>;
    async fn get_calendar_properties(
        &self,
        calendar_id: &str,
    ) -> Result<std::collections::HashMap<String, String>, DomainError>;

    // Event operations
    async fn create_event(&self, event: CreateEventDto) -> Result<CalendarEventDto, DomainError>;
    async fn create_event_from_ical(
        &self,
        event: CreateEventICalDto,
    ) -> Result<CalendarEventDto, DomainError>;
    async fn update_event(
        &self,
        event_id: &str,
        update: UpdateEventDto,
    ) -> Result<CalendarEventDto, DomainError>;
    async fn delete_event(&self, event_id: &str) -> Result<(), DomainError>;
    async fn get_event(&self, event_id: &str) -> Result<CalendarEventDto, DomainError>;
    async fn list_events_by_calendar(
        &self,
        calendar_id: &str,
    ) -> Result<Vec<CalendarEventDto>, DomainError>;
    async fn list_events_by_calendar_paginated(
        &self,
        calendar_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<CalendarEventDto>, DomainError>;
    async fn get_events_in_time_range(
        &self,
        calendar_id: &str,
        start: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<Vec<CalendarEventDto>, DomainError>;
}

/// Port for calendar use cases
#[async_trait]
pub trait CalendarUseCase: Send + Sync + 'static {
    // Calendar operations
    async fn create_calendar(
        &self,
        calendar: CreateCalendarDto,
    ) -> Result<CalendarDto, DomainError>;
    async fn update_calendar(
        &self,
        calendar_id: &str,
        update: UpdateCalendarDto,
    ) -> Result<CalendarDto, DomainError>;
    async fn delete_calendar(&self, calendar_id: &str) -> Result<(), DomainError>;
    async fn get_calendar(&self, calendar_id: &str) -> Result<CalendarDto, DomainError>;
    async fn list_my_calendars(&self) -> Result<Vec<CalendarDto>, DomainError>;
    async fn list_shared_calendars(&self) -> Result<Vec<CalendarDto>, DomainError>;
    async fn list_public_calendars(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<CalendarDto>, DomainError>;

    // Calendar sharing
    async fn share_calendar(
        &self,
        calendar_id: &str,
        user_id: &str,
        access_level: &str,
    ) -> Result<(), DomainError>;
    async fn remove_calendar_sharing(
        &self,
        calendar_id: &str,
        user_id: &str,
    ) -> Result<(), DomainError>;
    async fn get_calendar_shares(
        &self,
        calendar_id: &str,
    ) -> Result<Vec<(String, String)>, DomainError>;

    // Event operations
    async fn create_event(&self, event: CreateEventDto) -> Result<CalendarEventDto, DomainError>;
    async fn create_event_from_ical(
        &self,
        event: CreateEventICalDto,
    ) -> Result<CalendarEventDto, DomainError>;
    async fn update_event(
        &self,
        event_id: &str,
        update: UpdateEventDto,
    ) -> Result<CalendarEventDto, DomainError>;
    async fn delete_event(&self, event_id: &str) -> Result<(), DomainError>;
    async fn get_event(&self, event_id: &str) -> Result<CalendarEventDto, DomainError>;
    async fn list_events(
        &self,
        calendar_id: &str,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<CalendarEventDto>, DomainError>;
    async fn get_events_in_range(
        &self,
        calendar_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<CalendarEventDto>, DomainError>;

    // ─── User-contextualized variants (for CalDAV protocol handler) ──
    async fn create_calendar_for_user(
        &self,
        calendar: CreateCalendarDto,
        user_id: &str,
    ) -> Result<CalendarDto, DomainError>;
    async fn update_calendar_for_user(
        &self,
        calendar_id: &str,
        update: UpdateCalendarDto,
        user_id: &str,
    ) -> Result<CalendarDto, DomainError>;
    async fn delete_calendar_for_user(
        &self,
        calendar_id: &str,
        user_id: &str,
    ) -> Result<(), DomainError>;
    async fn get_calendar_for_user(
        &self,
        calendar_id: &str,
        user_id: &str,
    ) -> Result<CalendarDto, DomainError>;
    async fn list_my_calendars_for_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<CalendarDto>, DomainError>;
    async fn list_events_for_user(
        &self,
        calendar_id: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        user_id: &str,
    ) -> Result<Vec<CalendarEventDto>, DomainError>;
    async fn get_events_in_range_for_user(
        &self,
        calendar_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        user_id: &str,
    ) -> Result<Vec<CalendarEventDto>, DomainError>;
    async fn create_event_from_ical_for_user(
        &self,
        event: CreateEventICalDto,
        user_id: &str,
    ) -> Result<CalendarEventDto, DomainError>;
    async fn delete_event_for_user(&self, event_id: &str, user_id: &str)
    -> Result<(), DomainError>;
}
