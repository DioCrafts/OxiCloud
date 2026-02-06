/**
 * Calendar Event Entity
 * 
 * This module defines the CalendarEvent entity, which represents an event or
 * appointment in a calendar, following the iCalendar (RFC 5545) specification.
 * 
 * Calendar events have properties like summary, description, location, start/end times,
 * and can include recurrence rules for repeating events. Each event belongs to a
 * specific calendar and stores its complete iCalendar representation.
 */

use uuid::Uuid;
use chrono::{DateTime, Utc, Duration, TimeZone};

use crate::common::errors::{Result, DomainError, ErrorKind};

// Re-exportar errores de entidad desde el módulo centralizado
pub use super::entity_errors::CalendarEventError;

/**
 * CalendarEvent entity.
 * 
 * Represents a calendar event or appointment that can be synced via CalDAV.
 * Follows the iCalendar format (RFC 5545) for compatibility with CalDAV clients.
 */
#[derive(Debug, Clone)]
pub struct CalendarEvent {
    /// Unique identifier for the event
    id: Uuid,
    
    /// ID of the calendar this event belongs to
    calendar_id: Uuid,
    
    /// Short summary/title of the event
    summary: String,
    
    /// Detailed description of the event (optional)
    description: Option<String>,
    
    /// Location of the event (optional)
    location: Option<String>,
    
    /// Start time of the event
    start_time: DateTime<Utc>,
    
    /// End time of the event
    end_time: DateTime<Utc>,
    
    /// Whether this is an all-day event
    all_day: bool,
    
    /// Recurrence rule in iCalendar RRULE format (optional)
    rrule: Option<String>,
    
    /// Unique identifier in iCalendar format (used for CalDAV sync)
    ical_uid: String,
    
    /// Complete iCalendar data (VEVENT component)
    ical_data: String,
    
    /// Time when the event was created
    created_at: DateTime<Utc>,
    
    /// Time when the event was last modified
    updated_at: DateTime<Utc>,
}

impl CalendarEvent {
    /**
     * Creates a new calendar event with the given properties.
     * 
     * @param calendar_id ID of the calendar this event belongs to
     * @param summary Short summary/title of the event
     * @param description Detailed description of the event (optional)
     * @param location Location of the event (optional)
     * @param start_time Start time of the event
     * @param end_time End time of the event
     * @param all_day Whether this is an all-day event
     * @param rrule Recurrence rule in iCalendar RRULE format (optional)
     * @param ical_data Complete iCalendar data (VEVENT component)
     * @return Result containing the new CalendarEvent or a domain error
     */
    pub fn new(
        calendar_id: Uuid,
        summary: String,
        description: Option<String>,
        location: Option<String>,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        all_day: bool,
        rrule: Option<String>,
        ical_data: String,
    ) -> Result<Self> {
        // Validate inputs
        if summary.is_empty() {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "CalendarEvent",
                "Event summary cannot be empty",
            ));
        }
        
        if end_time < start_time {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "CalendarEvent",
                "End time cannot be before start time",
            ));
        }
        
        // Validate RRULE if provided (basic validation)
        if let Some(ref rule) = rrule {
            if !rule.starts_with("FREQ=") {
                return Err(DomainError::new(
                    ErrorKind::InvalidInput,
                    "CalendarEvent",
                    "Recurrence rule must start with FREQ=",
                ));
            }
        }
        
        // Validate iCalendar data (basic validation)
        if !ical_data.contains("BEGIN:VEVENT") || !ical_data.contains("END:VEVENT") {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "CalendarEvent",
                "iCalendar data must contain a VEVENT component",
            ));
        }
        
        let now = Utc::now();
        
        Ok(Self {
            id: Uuid::new_v4(),
            calendar_id,
            summary,
            description,
            location,
            start_time,
            end_time,
            all_day,
            rrule,
            ical_uid: Uuid::new_v4().to_string(),
            ical_data,
            created_at: now,
            updated_at: now,
        })
    }
    
    /**
     * Creates a calendar event with specific ID and timestamps.
     * Typically used when reconstructing from storage.
     * 
     * @param id Unique identifier for the event
     * @param calendar_id ID of the calendar this event belongs to
     * @param summary Short summary/title of the event
     * @param description Detailed description of the event (optional)
     * @param location Location of the event (optional)
     * @param start_time Start time of the event
     * @param end_time End time of the event
     * @param all_day Whether this is an all-day event
     * @param rrule Recurrence rule in iCalendar RRULE format (optional)
     * @param ical_uid Unique identifier in iCalendar format
     * @param ical_data Complete iCalendar data (VEVENT component)
     * @param created_at Time when the event was created
     * @param updated_at Time when the event was last modified
     * @return Result containing the new CalendarEvent or a domain error
     */
    pub fn with_id(
        id: Uuid,
        calendar_id: Uuid,
        summary: String,
        description: Option<String>,
        location: Option<String>,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        all_day: bool,
        rrule: Option<String>,
        ical_uid: String,
        ical_data: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self> {
        // Basic validation
        if summary.is_empty() {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "CalendarEvent",
                "Event summary cannot be empty",
            ));
        }
        
        if end_time < start_time {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "CalendarEvent",
                "End time cannot be before start time",
            ));
        }
        
        Ok(Self {
            id,
            calendar_id,
            summary,
            description,
            location,
            start_time,
            end_time,
            all_day,
            rrule,
            ical_uid,
            ical_data,
            created_at,
            updated_at,
        })
    }
    
    /**
     * Creates a calendar event from an iCalendar VEVENT component.
     * Parses the iCalendar data to extract event properties.
     * 
     * @param calendar_id ID of the calendar this event belongs to
     * @param ical_data Complete iCalendar data (VEVENT component)
     * @return Result containing the new CalendarEvent or a domain error
     */
    pub fn from_ical(calendar_id: Uuid, ical_data: String) -> Result<Self> {
        // This implementation would require a proper iCalendar parser
        // For brevity, we're using a simplified version here
        
        // Extract required fields from iCalendar data
        let summary = Self::extract_ical_property(&ical_data, "SUMMARY")
            .ok_or_else(|| DomainError::new(
                ErrorKind::InvalidInput,
                "CalendarEvent",
                "Missing SUMMARY in iCalendar data",
            ))?;
        
        let dtstart = Self::extract_ical_property(&ical_data, "DTSTART")
            .ok_or_else(|| DomainError::new(
                ErrorKind::InvalidInput,
                "CalendarEvent",
                "Missing DTSTART in iCalendar data",
            ))?;
        
        let dtend = Self::extract_ical_property(&ical_data, "DTEND")
            .ok_or_else(|| DomainError::new(
                ErrorKind::InvalidInput,
                "CalendarEvent",
                "Missing DTEND in iCalendar data",
            ))?;
        
        // Parse dates (simplified)
        let start_time = Self::parse_ical_datetime(&dtstart)
            .map_err(|e| DomainError::new(
                ErrorKind::InvalidInput,
                "CalendarEvent",
                format!("Invalid DTSTART: {}", e),
            ))?;
        
        let end_time = Self::parse_ical_datetime(&dtend)
            .map_err(|e| DomainError::new(
                ErrorKind::InvalidInput,
                "CalendarEvent",
                format!("Invalid DTEND: {}", e),
            ))?;
        
        // Determine if all-day event (simplified check)
        let all_day = dtstart.contains("VALUE=DATE") && !dtstart.contains("T");
        
        // Extract optional fields
        let description = Self::extract_ical_property(&ical_data, "DESCRIPTION");
        let location = Self::extract_ical_property(&ical_data, "LOCATION");
        let rrule = Self::extract_ical_property(&ical_data, "RRULE");
        
        // Extract UID or generate a new one
        let ical_uid = Self::extract_ical_property(&ical_data, "UID")
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        
        let now = Utc::now();
        
        Ok(Self {
            id: Uuid::new_v4(),
            calendar_id,
            summary,
            description,
            location,
            start_time,
            end_time,
            all_day,
            rrule,
            ical_uid,
            ical_data,
            created_at: now,
            updated_at: now,
        })
    }
    
    // Getters
    
    /// Returns the event's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    
    /// Returns the ID of the calendar this event belongs to
    pub fn calendar_id(&self) -> &Uuid {
        &self.calendar_id
    }
    
    /// Returns the event's summary/title
    pub fn summary(&self) -> &str {
        &self.summary
    }
    
    /// Returns the event's description, if any
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    
    /// Returns the event's location, if any
    pub fn location(&self) -> Option<&str> {
        self.location.as_deref()
    }
    
    /// Returns the event's start time
    pub fn start_time(&self) -> &DateTime<Utc> {
        &self.start_time
    }
    
    /// Returns the event's end time
    pub fn end_time(&self) -> &DateTime<Utc> {
        &self.end_time
    }
    
    /// Returns whether this is an all-day event
    pub fn all_day(&self) -> bool {
        self.all_day
    }
    
    /// Returns the event's recurrence rule, if any
    pub fn rrule(&self) -> Option<&str> {
        self.rrule.as_deref()
    }
    
    /// Returns the event's iCalendar UID
    pub fn ical_uid(&self) -> &str {
        &self.ical_uid
    }
    
    /// Returns the complete iCalendar data for the event
    pub fn ical_data(&self) -> &str {
        &self.ical_data
    }
    
    /// Returns the time when the event was created
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    
    /// Returns the time when the event was last modified
    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
    
    /// Returns the duration of the event
    pub fn duration(&self) -> Duration {
        self.end_time - self.start_time
    }
    
    // Setters and Mutators
    
    /**
     * Updates the event's summary/title.
     * 
     * @param summary New summary/title for the event
     * @return Result indicating success or containing a domain error
     */
    pub fn update_summary(&mut self, summary: String) -> Result<()> {
        if summary.is_empty() {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "CalendarEvent",
                "Event summary cannot be empty",
            ));
        }
        
        // Clone the summary before updating the struct
        let summary_clone = summary.clone();
        self.summary = summary;
        self.updated_at = Utc::now();
        
        // Update iCalendar data using the cloned value
        self.update_ical_property("SUMMARY", &summary_clone);
        
        Ok(())
    }
    
    /**
     * Updates the event's description.
     * 
     * @param description New description for the event
     */
    pub fn update_description(&mut self, description: Option<String>) {
        self.description = description.clone();
        self.updated_at = Utc::now();
        
        // Update iCalendar data
        match description {
            Some(desc) => self.update_ical_property("DESCRIPTION", &desc),
            None => self.remove_ical_property("DESCRIPTION"),
        }
    }
    
    /**
     * Updates the event's location.
     * 
     * @param location New location for the event
     */
    pub fn update_location(&mut self, location: Option<String>) {
        self.location = location.clone();
        self.updated_at = Utc::now();
        
        // Update iCalendar data
        match location {
            Some(loc) => self.update_ical_property("LOCATION", &loc),
            None => self.remove_ical_property("LOCATION"),
        }
    }
    
    /**
     * Updates the event's start and end times.
     * 
     * @param start_time New start time for the event
     * @param end_time New end time for the event
     * @return Result indicating success or containing a domain error
     */
    pub fn update_time_range(&mut self, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Result<()> {
        if end_time < start_time {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "CalendarEvent",
                "End time cannot be before start time",
            ));
        }
        
        self.start_time = start_time;
        self.end_time = end_time;
        self.updated_at = Utc::now();
        
        // Update iCalendar data
        let start_str = if self.all_day {
            format!("{}T000000Z", start_time.format("%Y%m%d"))
        } else {
            format!("{}", start_time.format("%Y%m%dT%H%M%SZ"))
        };
        
        let end_str = if self.all_day {
            format!("{}T000000Z", end_time.format("%Y%m%d"))
        } else {
            format!("{}", end_time.format("%Y%m%dT%H%M%SZ"))
        };
        
        self.update_ical_property("DTSTART", &start_str);
        self.update_ical_property("DTEND", &end_str);
        
        Ok(())
    }
    
    /**
     * Updates whether this is an all-day event.
     * 
     * @param all_day Whether this is an all-day event
     */
    pub fn update_all_day(&mut self, all_day: bool) {
        self.all_day = all_day;
        self.updated_at = Utc::now();
        
        // Update iCalendar data
        let start_str = if all_day {
            format!("VALUE=DATE:{}", self.start_time.format("%Y%m%d"))
        } else {
            format!("{}", self.start_time.format("%Y%m%dT%H%M%SZ"))
        };
        
        let end_str = if all_day {
            format!("VALUE=DATE:{}", self.end_time.format("%Y%m%d"))
        } else {
            format!("{}", self.end_time.format("%Y%m%dT%H%M%SZ"))
        };
        
        self.update_ical_property("DTSTART", &start_str);
        self.update_ical_property("DTEND", &end_str);
    }
    
    /**
     * Updates the event's recurrence rule.
     * 
     * @param rrule New recurrence rule for the event
     * @return Result indicating success or containing a domain error
     */
    pub fn update_rrule(&mut self, rrule: Option<String>) -> Result<()> {
        // Validate RRULE if provided (basic validation)
        if let Some(ref rule) = rrule {
            if !rule.starts_with("FREQ=") {
                return Err(DomainError::new(
                    ErrorKind::InvalidInput,
                    "CalendarEvent",
                    "Recurrence rule must start with FREQ=",
                ));
            }
        }
        
        self.rrule = rrule.clone();
        self.updated_at = Utc::now();
        
        // Update iCalendar data
        match rrule {
            Some(rule) => self.update_ical_property("RRULE", &rule),
            None => self.remove_ical_property("RRULE"),
        }
        
        Ok(())
    }
    
    /**
     * Updates the complete iCalendar data for the event.
     * Also updates the event properties based on the new iCalendar data.
     * 
     * @param ical_data New iCalendar data for the event
     * @return Result indicating success or containing a domain error
     */
    pub fn update_ical_data(&mut self, ical_data: String) -> Result<()> {
        // Validate iCalendar data (basic validation)
        if !ical_data.contains("BEGIN:VEVENT") || !ical_data.contains("END:VEVENT") {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "CalendarEvent",
                "iCalendar data must contain a VEVENT component",
            ));
        }
        
        // Extract and update properties from iCalendar data
        if let Some(summary) = Self::extract_ical_property(&ical_data, "SUMMARY") {
            self.summary = summary;
        }
        
        self.description = Self::extract_ical_property(&ical_data, "DESCRIPTION");
        self.location = Self::extract_ical_property(&ical_data, "LOCATION");
        
        if let Some(dtstart) = Self::extract_ical_property(&ical_data, "DTSTART") {
            if let Ok(start_time) = Self::parse_ical_datetime(&dtstart) {
                self.start_time = start_time;
            }
        }
        
        if let Some(dtend) = Self::extract_ical_property(&ical_data, "DTEND") {
            if let Ok(end_time) = Self::parse_ical_datetime(&dtend) {
                self.end_time = end_time;
            }
        }
        
        // Update all-day status based on DTSTART
        if let Some(dtstart) = Self::extract_ical_property(&ical_data, "DTSTART") {
            self.all_day = dtstart.contains("VALUE=DATE") && !dtstart.contains("T");
        }
        
        self.rrule = Self::extract_ical_property(&ical_data, "RRULE");
        
        if let Some(uid) = Self::extract_ical_property(&ical_data, "UID") {
            self.ical_uid = uid;
        }
        
        self.ical_data = ical_data;
        self.updated_at = Utc::now();
        
        Ok(())
    }
    
    /**
     * Checks if this event belongs to the specified calendar.
     * 
     * @param calendar_id ID of the calendar to check against
     * @return true if the event belongs to the calendar, false otherwise
     */
    pub fn belongs_to_calendar(&self, calendar_id: &Uuid) -> bool {
        self.calendar_id == *calendar_id
    }
    
    /**
     * Checks if this event occurs within the specified time range.
     * 
     * @param start Start of the time range to check
     * @param end End of the time range to check
     * @return true if the event occurs within the range, false otherwise
     */
    pub fn occurs_in_range(&self, start: &DateTime<Utc>, end: &DateTime<Utc>) -> bool {
        // Basic case: event directly overlaps with range
        if self.start_time <= *end && self.end_time >= *start {
            return true;
        }
        
        // If event has recurrence, check if any recurrence occurs in range
        // Note: A full implementation would need a proper recurrence rule parser
        if let Some(rrule) = &self.rrule {
            // Simplified check for demonstration
            // A real implementation would need to generate recurrence instances
            // and check if any fall within the range
            
            // For now, we'll just check if the recurrence hasn't ended
            // or if it ended after the start of our range
            if let Some(until_pos) = rrule.find("UNTIL=") {
                let until_start = until_pos + 6; // "UNTIL=" is 6 chars
                if let Some(until_end) = rrule[until_start..].find(';') {
                    let until_str = &rrule[until_start..until_start+until_end];
                    if let Ok(until_date) = Self::parse_ical_datetime(&until_str) {
                        return until_date >= *start;
                    }
                } else {
                    // UNTIL is the last part of the rule
                    let until_str = &rrule[until_start..];
                    if let Ok(until_date) = Self::parse_ical_datetime(&until_str) {
                        return until_date >= *start;
                    }
                }
            } else {
                // No UNTIL specified, so recurrence continues indefinitely
                return true;
            }
        }
        
        false
    }
    
    // Helper methods for iCalendar operations
    
    /**
     * Extracts a property value from iCalendar data.
     * 
     * @param ical_data The iCalendar data to search in
     * @param property_name The name of the property to extract
     * @return Option containing the property value if found
     */
    fn extract_ical_property(ical_data: &str, property_name: &str) -> Option<String> {
        // Find the property in the iCalendar data
        let search_str = format!("\n{}:", property_name);
        let search_str_alt = format!("\r\n{}:", property_name);
        
        let pos = ical_data.find(&search_str)
            .or_else(|| ical_data.find(&search_str_alt));
        
        if let Some(pos) = pos {
            // Find the start of the value
            let value_start = pos + search_str.len();
            
            // Find the end of the value (next line or end of string)
            let value_end = ical_data[value_start..]
                .find('\n')
                .map(|p| value_start + p)
                .unwrap_or_else(|| ical_data.len());
            
            // Extract and return the value
            let value = ical_data[value_start..value_end].trim();
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
        
        None
    }
    
    /**
     * Parses an iCalendar datetime string into a DateTime object.
     * 
     * @param datetime The iCalendar datetime string to parse
     * @return Result containing the parsed DateTime or an error
     */
    fn parse_ical_datetime(datetime: &str) -> std::result::Result<DateTime<Utc>, String> {
        // Handle VALUE=DATE format
        if datetime.contains("VALUE=DATE") {
            let date_str = datetime.split(':').last().unwrap_or("");
            if date_str.len() != 8 {
                return Err("Invalid date format".to_string());
            }
            
            let year = date_str[0..4].parse::<i32>()
                .map_err(|_| "Invalid year".to_string())?;
            let month = date_str[4..6].parse::<u32>()
                .map_err(|_| "Invalid month".to_string())?;
            let day = date_str[6..8].parse::<u32>()
                .map_err(|_| "Invalid day".to_string())?;
            
            return match chrono::NaiveDate::from_ymd_opt(year, month, day) {
                Some(date) => Ok(Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).unwrap())),
                None => Err("Invalid date components".to_string()),
            };
        }
        
        // Handle standard UTC format (20230101T120000Z)
        let datetime_str = datetime.split(':').last().unwrap_or(datetime);
        if datetime_str.len() < 15 || !datetime_str.ends_with('Z') {
            return Err("Invalid datetime format".to_string());
        }
        
        let year = datetime_str[0..4].parse::<i32>()
            .map_err(|_| "Invalid year".to_string())?;
        let month = datetime_str[4..6].parse::<u32>()
            .map_err(|_| "Invalid month".to_string())?;
        let day = datetime_str[6..8].parse::<u32>()
            .map_err(|_| "Invalid day".to_string())?;
        
        let hour = datetime_str[9..11].parse::<u32>()
            .map_err(|_| "Invalid hour".to_string())?;
        let minute = datetime_str[11..13].parse::<u32>()
            .map_err(|_| "Invalid minute".to_string())?;
        let second = datetime_str[13..15].parse::<u32>()
            .map_err(|_| "Invalid second".to_string())?;
        
        match chrono::NaiveDate::from_ymd_opt(year, month, day) {
            Some(date) => match date.and_hms_opt(hour, minute, second) {
                Some(datetime) => Ok(Utc.from_utc_datetime(&datetime)),
                None => Err("Invalid time components".to_string()),
            },
            None => Err("Invalid date components".to_string()),
        }
    }
    
    /**
     * Updates an iCalendar property in the event's iCalendar data.
     * 
     * @param property_name The name of the property to update
     * @param value The new value for the property
     */
    fn update_ical_property(&mut self, property_name: &str, value: &str) {
        let search_str = format!("\n{}:", property_name);
        let search_str_alt = format!("\r\n{}:", property_name);
        
        // Check if property exists
        let pos = self.ical_data.find(&search_str)
            .or_else(|| self.ical_data.find(&search_str_alt));
        
        if let Some(pos) = pos {
            // Find the start of the value
            let value_start = pos + search_str.len();
            
            // Find the end of the value (next line or end of string)
            let value_end = self.ical_data[value_start..]
                .find('\n')
                .map(|p| value_start + p)
                .unwrap_or_else(|| self.ical_data.len());
            
            // Replace the value
            let before = &self.ical_data[..value_start];
            let after = &self.ical_data[value_end..];
            self.ical_data = format!("{}{}{}", before, value, after);
        } else {
            // Property doesn't exist, add it before END:VEVENT
            let end_pos = self.ical_data.find("END:VEVENT")
                .unwrap_or(self.ical_data.len());
            
            let before = &self.ical_data[..end_pos];
            let after = &self.ical_data[end_pos..];
            self.ical_data = format!("{}{}:{}\n{}", before, property_name, value, after);
        }
    }
    
    /**
     * Removes an iCalendar property from the event's iCalendar data.
     * 
     * @param property_name The name of the property to remove
     */
    fn remove_ical_property(&mut self, property_name: &str) {
        let search_str = format!("\n{}:", property_name);
        let search_str_alt = format!("\r\n{}:", property_name);
        
        // Check if property exists
        let pos = self.ical_data.find(&search_str)
            .or_else(|| self.ical_data.find(&search_str_alt));
        
        if let Some(pos) = pos {
            // Find the end of the value (next line or end of string)
            let value_end = self.ical_data[pos + 1..]
                .find('\n')
                .map(|p| pos + 1 + p)
                .unwrap_or_else(|| self.ical_data.len());
            
            // Remove the property
            let before = &self.ical_data[..pos];
            let after = &self.ical_data[value_end..];
            self.ical_data = format!("{}{}", before, after);
        }
    }
}