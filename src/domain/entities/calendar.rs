use chrono::{DateTime, Utc};
/**
 * Calendar Entity
 *
 * This module defines the Calendar entity, which represents a calendar in the CalDAV
 * implementation. Calendars contain calendar events and are owned by users.
 *
 * Calendars have properties such as name, color, and description, and they serve as
 * containers for calendar events. Each calendar belongs to a specific user and can
 * have custom properties.
 */
use uuid::Uuid;

use crate::common::errors::{DomainError, ErrorKind, Result};

// Re-export entity errors from the centralized module
pub use super::entity_errors::CalendarError;

/**
 * Calendar entity.
 *
 * Represents a calendar container that can hold multiple calendar events.
 * Each calendar is owned by a user and has properties like name, color, and description.
 */
#[derive(Debug, Clone)]
pub struct Calendar {
    /// Unique identifier for the calendar
    id: Uuid,

    /// Display name of the calendar
    name: String,

    /// ID of the user who owns this calendar
    owner_id: String,

    /// Optional description of the calendar
    description: Option<String>,

    /// Optional color code for UI display (hex format #RRGGBB)
    color: Option<String>,

    /// Time when the calendar was created
    created_at: DateTime<Utc>,

    /// Time when the calendar was last modified
    updated_at: DateTime<Utc>,

    /// Optional list of custom properties (for extended CalDAV support)
    custom_properties: std::collections::HashMap<String, String>,
}

impl Calendar {
    /**
     * Creates a new calendar with the given properties.
     *
     * @param name Display name of the calendar
     * @param owner_id ID of the user who owns this calendar
     * @param description Optional description of the calendar
     * @param color Optional color code for UI display (#RRGGBB format)
     * @return Result containing the new Calendar or a domain error
     */
    pub fn new(
        name: String,
        owner_id: String,
        description: Option<String>,
        color: Option<String>,
    ) -> Result<Self> {
        // Validate inputs
        if name.is_empty() {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "Calendar",
                "Calendar name cannot be empty",
            ));
        }

        if owner_id.is_empty() {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "Calendar",
                "Owner ID cannot be empty",
            ));
        }

        // Validate color format if provided (#RRGGBB)
        if let Some(ref color_str) = color {
            if !color_str.starts_with('#') || color_str.len() != 7 {
                return Err(DomainError::new(
                    ErrorKind::InvalidInput,
                    "Calendar",
                    "Color must be in #RRGGBB format",
                ));
            }

            // Check if remaining characters are valid hex
            if color_str[1..].chars().any(|c| !c.is_ascii_hexdigit()) {
                return Err(DomainError::new(
                    ErrorKind::InvalidInput,
                    "Calendar",
                    "Color must be in #RRGGBB format with valid hex digits",
                ));
            }
        }

        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            name,
            owner_id,
            description,
            color,
            created_at: now,
            updated_at: now,
            custom_properties: std::collections::HashMap::new(),
        })
    }

    /**
     * Creates a calendar with specific ID and timestamps.
     * Typically used when reconstructing from storage.
     *
     * @param id Unique identifier for the calendar
     * @param name Display name of the calendar
     * @param owner_id ID of the user who owns this calendar
     * @param description Optional description of the calendar
     * @param color Optional color code for UI display
     * @param created_at Time when the calendar was created
     * @param updated_at Time when the calendar was last modified
     * @return Result containing the new Calendar or a domain error
     */
    pub fn with_id(
        id: Uuid,
        name: String,
        owner_id: String,
        description: Option<String>,
        color: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self> {
        // Basic validation
        if name.is_empty() {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "Calendar",
                "Calendar name cannot be empty",
            ));
        }

        if owner_id.is_empty() {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "Calendar",
                "Owner ID cannot be empty",
            ));
        }

        Ok(Self {
            id,
            name,
            owner_id,
            description,
            color,
            created_at,
            updated_at,
            custom_properties: std::collections::HashMap::new(),
        })
    }

    // Getters

    /// Returns the calendar's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Returns the calendar's display name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the ID of the user who owns this calendar
    pub fn owner_id(&self) -> &str {
        &self.owner_id
    }

    /// Returns the calendar's description, if any
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the calendar's color code, if any
    pub fn color(&self) -> Option<&str> {
        self.color.as_deref()
    }

    /// Returns the time when the calendar was created
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    /// Returns the time when the calendar was last modified
    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    /// Returns a custom property value by name, if it exists
    pub fn custom_property(&self, name: &str) -> Option<&str> {
        self.custom_properties.get(name).map(|s| s.as_str())
    }

    /// Returns all custom properties
    pub fn custom_properties(&self) -> &std::collections::HashMap<String, String> {
        &self.custom_properties
    }

    // Setters and Mutators

    /**
     * Updates the calendar's name.
     *
     * @param name New display name for the calendar
     * @return Result indicating success or containing a domain error
     */
    pub fn update_name(&mut self, name: String) -> Result<()> {
        if name.is_empty() {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "Calendar",
                "Calendar name cannot be empty",
            ));
        }

        self.name = name;
        self.updated_at = Utc::now();
        Ok(())
    }

    /**
     * Updates the calendar's description.
     *
     * @param description New description for the calendar
     */
    pub fn update_description(&mut self, description: Option<String>) {
        self.description = description;
        self.updated_at = Utc::now();
    }

    /**
     * Updates the calendar's color.
     *
     * @param color New color code for the calendar
     * @return Result indicating success or containing a domain error
     */
    pub fn update_color(&mut self, color: Option<String>) -> Result<()> {
        // Validate color format if provided (#RRGGBB)
        if let Some(ref color_str) = color {
            if !color_str.starts_with('#') || color_str.len() != 7 {
                return Err(DomainError::new(
                    ErrorKind::InvalidInput,
                    "Calendar",
                    "Color must be in #RRGGBB format",
                ));
            }

            // Check if remaining characters are valid hex
            if color_str[1..].chars().any(|c| !c.is_ascii_hexdigit()) {
                return Err(DomainError::new(
                    ErrorKind::InvalidInput,
                    "Calendar",
                    "Color must be in #RRGGBB format with valid hex digits",
                ));
            }
        }

        self.color = color;
        self.updated_at = Utc::now();
        Ok(())
    }

    /**
     * Sets a custom property for extended CalDAV support.
     *
     * @param name Name of the property
     * @param value Value of the property
     */
    pub fn set_custom_property(&mut self, name: String, value: String) {
        self.custom_properties.insert(name, value);
        self.updated_at = Utc::now();
    }

    /**
     * Removes a custom property.
     *
     * @param name Name of the property to remove
     * @return true if the property was removed, false if it didn't exist
     */
    pub fn remove_custom_property(&mut self, name: &str) -> bool {
        let result = self.custom_properties.remove(name).is_some();
        if result {
            self.updated_at = Utc::now();
        }
        result
    }

    /**
     * Checks if this calendar belongs to the specified user.
     *
     * @param user_id ID of the user to check ownership against
     * @return true if the calendar belongs to the user, false otherwise
     */
    pub fn belongs_to(&self, user_id: &str) -> bool {
        self.owner_id == user_id
    }

    /**
     * Updates the last modification time of the calendar to now.
     * Called when calendar events are added, modified, or removed.
     */
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}
