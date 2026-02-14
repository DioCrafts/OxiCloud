use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::Arc;

use crate::application::dtos::calendar_dto::{
    CalendarDto, CalendarEventDto, CreateCalendarDto, CreateEventDto, CreateEventICalDto,
    UpdateCalendarDto, UpdateEventDto,
};
use crate::application::ports::calendar_ports::{CalendarStoragePort, CalendarUseCase};
use crate::common::errors::{DomainError, ErrorKind};

pub struct CalendarService {
    calendar_storage: Arc<dyn CalendarStoragePort>,
}

impl CalendarService {
    pub fn new(calendar_storage: Arc<dyn CalendarStoragePort>) -> Self {
        Self { calendar_storage }
    }
}

#[async_trait]
impl CalendarUseCase for CalendarService {
    async fn create_calendar(
        &self,
        calendar: CreateCalendarDto,
        user_id: &str,
    ) -> Result<CalendarDto, DomainError> {
        self.calendar_storage
            .create_calendar(calendar, user_id)
            .await
    }

    async fn update_calendar(
        &self,
        calendar_id: &str,
        update: UpdateCalendarDto,
        user_id: &str,
    ) -> Result<CalendarDto, DomainError> {
        let has_access = self
            .calendar_storage
            .check_calendar_access(calendar_id, user_id)
            .await?;
        if !has_access {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Calendar",
                "You don't have permission to update this calendar",
            ));
        }
        self.calendar_storage
            .update_calendar(calendar_id, update)
            .await
    }

    async fn delete_calendar(&self, calendar_id: &str, user_id: &str) -> Result<(), DomainError> {
        let has_access = self
            .calendar_storage
            .check_calendar_access(calendar_id, user_id)
            .await?;
        if !has_access {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Calendar",
                "You don't have permission to delete this calendar",
            ));
        }
        self.calendar_storage.delete_calendar(calendar_id).await
    }

    async fn get_calendar(
        &self,
        calendar_id: &str,
        user_id: &str,
    ) -> Result<CalendarDto, DomainError> {
        let calendar = self.calendar_storage.get_calendar(calendar_id).await?;
        let has_access = self
            .calendar_storage
            .check_calendar_access(calendar_id, user_id)
            .await?;
        if !has_access && !calendar.is_public {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Calendar",
                "You don't have permission to view this calendar",
            ));
        }
        Ok(calendar)
    }

    async fn list_my_calendars(&self, user_id: &str) -> Result<Vec<CalendarDto>, DomainError> {
        self.calendar_storage.list_calendars_by_owner(user_id).await
    }

    async fn list_shared_calendars(&self, user_id: &str) -> Result<Vec<CalendarDto>, DomainError> {
        self.calendar_storage
            .list_calendars_shared_with_user(user_id)
            .await
    }

    async fn list_public_calendars(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<CalendarDto>, DomainError> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);
        self.calendar_storage
            .list_public_calendars(limit, offset)
            .await
    }

    async fn share_calendar(
        &self,
        calendar_id: &str,
        target_user_id: &str,
        access_level: &str,
        caller_user_id: &str,
    ) -> Result<(), DomainError> {
        let calendar = self.calendar_storage.get_calendar(calendar_id).await?;
        if calendar.owner_id != caller_user_id {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Calendar",
                "Only the calendar owner can change sharing settings",
            ));
        }
        match access_level {
            "read" | "write" | "owner" => {}
            _ => {
                return Err(DomainError::new(
                    ErrorKind::InvalidInput,
                    "Calendar",
                    format!(
                        "Invalid access level: {}. Valid values are: read, write, owner",
                        access_level
                    ),
                ));
            }
        }
        self.calendar_storage
            .share_calendar(calendar_id, target_user_id, access_level)
            .await
    }

    async fn remove_calendar_sharing(
        &self,
        calendar_id: &str,
        target_user_id: &str,
        caller_user_id: &str,
    ) -> Result<(), DomainError> {
        let calendar = self.calendar_storage.get_calendar(calendar_id).await?;
        if calendar.owner_id != caller_user_id {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Calendar",
                "Only the calendar owner can change sharing settings",
            ));
        }
        self.calendar_storage
            .remove_calendar_sharing(calendar_id, target_user_id)
            .await
    }

    async fn get_calendar_shares(
        &self,
        calendar_id: &str,
        user_id: &str,
    ) -> Result<Vec<(String, String)>, DomainError> {
        let calendar = self.calendar_storage.get_calendar(calendar_id).await?;
        if calendar.owner_id != user_id {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Calendar",
                "Only the calendar owner can view sharing settings",
            ));
        }
        self.calendar_storage.get_calendar_shares(calendar_id).await
    }

    async fn create_event(
        &self,
        event: CreateEventDto,
        user_id: &str,
    ) -> Result<CalendarEventDto, DomainError> {
        let has_access = self
            .calendar_storage
            .check_calendar_access(&event.calendar_id, user_id)
            .await?;
        if !has_access {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Calendar",
                "You don't have permission to add events to this calendar",
            ));
        }
        self.calendar_storage.create_event(event).await
    }

    async fn create_event_from_ical(
        &self,
        event: CreateEventICalDto,
        user_id: &str,
    ) -> Result<CalendarEventDto, DomainError> {
        let has_access = self
            .calendar_storage
            .check_calendar_access(&event.calendar_id, user_id)
            .await?;
        if !has_access {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Calendar",
                "You don't have permission to add events to this calendar",
            ));
        }
        self.calendar_storage.create_event_from_ical(event).await
    }

    async fn update_event(
        &self,
        event_id: &str,
        update: UpdateEventDto,
        user_id: &str,
    ) -> Result<CalendarEventDto, DomainError> {
        let event = self.calendar_storage.get_event(event_id).await?;
        let has_access = self
            .calendar_storage
            .check_calendar_access(&event.calendar_id, user_id)
            .await?;
        if !has_access {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Calendar",
                "You don't have permission to update events in this calendar",
            ));
        }
        self.calendar_storage.update_event(event_id, update).await
    }

    async fn delete_event(&self, event_id: &str, user_id: &str) -> Result<(), DomainError> {
        let event = self.calendar_storage.get_event(event_id).await?;
        let has_access = self
            .calendar_storage
            .check_calendar_access(&event.calendar_id, user_id)
            .await?;
        if !has_access {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Calendar",
                "You don't have permission to delete events in this calendar",
            ));
        }
        self.calendar_storage.delete_event(event_id).await
    }

    async fn get_event(
        &self,
        event_id: &str,
        user_id: &str,
    ) -> Result<CalendarEventDto, DomainError> {
        let event = self.calendar_storage.get_event(event_id).await?;
        let has_access = self
            .calendar_storage
            .check_calendar_access(&event.calendar_id, user_id)
            .await?;
        let calendar = self
            .calendar_storage
            .get_calendar(&event.calendar_id)
            .await?;
        if !has_access && !calendar.is_public {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Calendar",
                "You don't have permission to view events in this calendar",
            ));
        }
        Ok(event)
    }

    async fn list_events(
        &self,
        calendar_id: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        user_id: &str,
    ) -> Result<Vec<CalendarEventDto>, DomainError> {
        let has_access = self
            .calendar_storage
            .check_calendar_access(calendar_id, user_id)
            .await?;
        let calendar = self.calendar_storage.get_calendar(calendar_id).await?;
        if !has_access && !calendar.is_public {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Calendar",
                "You don't have permission to view events in this calendar",
            ));
        }
        if limit.is_some() || offset.is_some() {
            let limit = limit.unwrap_or(100);
            let offset = offset.unwrap_or(0);
            self.calendar_storage
                .list_events_by_calendar_paginated(calendar_id, limit, offset)
                .await
        } else {
            self.calendar_storage
                .list_events_by_calendar(calendar_id)
                .await
        }
    }

    async fn get_events_in_range(
        &self,
        calendar_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        user_id: &str,
    ) -> Result<Vec<CalendarEventDto>, DomainError> {
        let has_access = self
            .calendar_storage
            .check_calendar_access(calendar_id, user_id)
            .await?;
        let calendar = self.calendar_storage.get_calendar(calendar_id).await?;
        if !has_access && !calendar.is_public {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Calendar",
                "You don't have permission to view events in this calendar",
            ));
        }
        self.calendar_storage
            .get_events_in_time_range(calendar_id, &start, &end)
            .await
    }
}
