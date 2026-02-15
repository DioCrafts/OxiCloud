use chrono::{DateTime, Utc};
use quick_xml::{
    Reader, Writer,
    events::{BytesEnd, BytesStart, BytesText, Event},
};
/**
 * CalDAV Adapter Module
 *
 * This module provides conversion between CalDAV protocol XML structures and OxiCloud domain objects.
 * It handles parsing CalDAV request XML and generating CalDAV response XML according to RFC 4791.
 */
use std::io::{BufReader, Read, Write};
use uuid::Uuid;

use crate::application::adapters::webdav_adapter::{
    PropFindRequest, PropFindType, QualifiedName, Result, WebDavAdapter, WebDavError,
};
use crate::application::dtos::calendar_dto::{CalendarDto, CalendarEventDto};

/// CalDAV report type
#[derive(Debug, PartialEq)]
pub enum CalDavReportType {
    /// Calendar-query report
    CalendarQuery {
        time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
        props: Vec<QualifiedName>,
    },
    /// Calendar-multiget report
    CalendarMultiget {
        hrefs: Vec<String>,
        props: Vec<QualifiedName>,
    },
    /// Sync-collection report
    SyncCollection {
        sync_token: String,
        props: Vec<QualifiedName>,
    },
}

/// CalDAV adapter for converting between XML and domain objects
pub struct CalDavAdapter;

impl CalDavAdapter {
    /// Parse a REPORT XML request for CalDAV
    pub fn parse_report<R: Read>(reader: R) -> Result<CalDavReportType> {
        let mut xml_reader = Reader::from_reader(BufReader::new(reader));
        xml_reader.config_mut().trim_text(true);

        let mut buffer = Vec::new();
        let mut in_calendar_query = false;
        let mut in_calendar_multiget = false;
        let mut in_sync_collection = false;
        let mut in_prop = false;
        let mut in_filter = false;
        let mut start_time: Option<DateTime<Utc>> = None;
        let mut end_time: Option<DateTime<Utc>> = None;
        let mut props = Vec::new();
        let mut hrefs = Vec::new();
        let mut sync_token = String::new();

        loop {
            match xml_reader.read_event_into(&mut buffer) {
                Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    let name_str = std::str::from_utf8(name.as_ref()).unwrap_or("");

                    match name_str {
                        s if s == "calendar-query" || s.ends_with(":calendar-query") => {
                            in_calendar_query = true
                        }
                        s if s == "calendar-multiget" || s.ends_with(":calendar-multiget") => {
                            in_calendar_multiget = true
                        }
                        s if s == "sync-collection" || s.ends_with(":sync-collection") => {
                            in_sync_collection = true
                        }
                        s if s == "prop" || s.ends_with(":prop") => in_prop = true,
                        s if s == "filter" || s.ends_with(":filter") => in_filter = true,
                        s if s == "time-range" || s.ends_with(":time-range") => {
                            // Parse time-range attributes
                            for attr in e.attributes().flatten() {
                                let attr_name =
                                    std::str::from_utf8(attr.key.as_ref()).unwrap_or("");
                                let attr_value = attr.unescape_value().unwrap_or_default();

                                if attr_name == "start" {
                                    // Parse ISO date format with Z for UTC
                                    start_time = DateTime::parse_from_rfc3339(&attr_value)
                                        .ok()
                                        .map(|dt| dt.with_timezone(&Utc));
                                } else if attr_name == "end" {
                                    end_time = DateTime::parse_from_rfc3339(&attr_value)
                                        .ok()
                                        .map(|dt| dt.with_timezone(&Utc));
                                }
                            }
                        }
                        s if s == "sync-token" || s.ends_with(":sync-token") => {
                            // We'll capture the text in the Text event
                        }
                        s if s == "href" || s.ends_with(":href") => {
                            // We'll capture the text in the Text event
                        }
                        _ if in_prop => {
                            // Add property to request
                            let namespace = WebDavAdapter::extract_namespace(name_str);
                            let prop_name = WebDavAdapter::extract_local_name(name_str);

                            props.push(QualifiedName::new(namespace, prop_name));
                        }
                        _ => { /* Ignore other elements */ }
                    }
                }
                Ok(Event::Text(e)) => {
                    let text = e.decode().unwrap_or_default();

                    // Check if we're in sync-token element
                    if in_sync_collection && !in_prop && !in_filter {
                        sync_token = text.to_string();
                    }

                    // Check if we're in href element
                    if (in_calendar_multiget || in_sync_collection) && !in_prop && !in_filter {
                        hrefs.push(text.to_string());
                    }
                }
                Ok(Event::End(ref e)) => {
                    let name = e.name();
                    let name_str = std::str::from_utf8(name.as_ref()).unwrap_or("");

                    match name_str {
                        // Don't reset report-type flags — they're needed at EOF for decision logic
                        s if s == "prop" || s.ends_with(":prop") => in_prop = false,
                        s if s == "filter" || s.ends_with(":filter") => in_filter = false,
                        s if s == "time-range" || s.ends_with(":time-range") => { /* time-range end, attributes already parsed */
                        }
                        _ => (),
                    }
                }
                Ok(Event::Empty(ref e)) => {
                    let name = e.name();
                    let name_str = std::str::from_utf8(name.as_ref()).unwrap_or("");

                    if in_prop {
                        // Add empty property element to request
                        let namespace = WebDavAdapter::extract_namespace(name_str);
                        let prop_name = WebDavAdapter::extract_local_name(name_str);

                        props.push(QualifiedName::new(namespace, prop_name));
                    } else if name_str == "time-range" || name_str.ends_with(":time-range") {
                        // Parse time-range attributes
                        for attr in e.attributes().flatten() {
                            let attr_name =
                                std::str::from_utf8(attr.key.as_ref()).unwrap_or("");
                            let attr_value = attr.unescape_value().unwrap_or_default();

                            if attr_name == "start" {
                                // Parse ISO date format with Z for UTC
                                start_time = DateTime::parse_from_rfc3339(&attr_value)
                                    .ok()
                                    .map(|dt| dt.with_timezone(&Utc));
                            } else if attr_name == "end" {
                                end_time = DateTime::parse_from_rfc3339(&attr_value)
                                    .ok()
                                    .map(|dt| dt.with_timezone(&Utc));
                            }
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(WebDavError::XmlError(e)),
                _ => (),
            }

            buffer.clear();
        }

        // Create the appropriate report type based on what we parsed
        let report_type = if in_calendar_query {
            // If both start and end time are present, create a time range
            let time_range = if let (Some(start), Some(end)) = (start_time, end_time) {
                Some((start, end))
            } else {
                None
            };

            CalDavReportType::CalendarQuery { time_range, props }
        } else if in_calendar_multiget {
            CalDavReportType::CalendarMultiget { hrefs, props }
        } else if in_sync_collection {
            CalDavReportType::SyncCollection { sync_token, props }
        } else {
            // Default to empty calendar query
            CalDavReportType::CalendarQuery {
                time_range: None,
                props,
            }
        };

        Ok(report_type)
    }

    /// Generate a PROPFIND response for calendars
    pub fn generate_calendars_propfind_response<W: Write>(
        writer: W,
        calendars: &[CalendarDto],
        request: &PropFindRequest,
        base_href: &str,
    ) -> Result<()> {
        let mut xml_writer = Writer::new(writer);

        // Start multistatus response
        xml_writer.write_event(Event::Start(
            BytesStart::new("D:multistatus").with_attributes([
                ("xmlns:D", "DAV:"),
                ("xmlns:C", "urn:ietf:params:xml:ns:caldav"),
                ("xmlns:CS", "http://calendarserver.org/ns/"),
            ]),
        ))?;

        // Add responses for calendars
        for calendar in calendars {
            Self::write_calendar_response(
                &mut xml_writer,
                calendar,
                request,
                &format!("{}{}/", base_href, calendar.id),
            )?;
        }

        // End multistatus
        xml_writer.write_event(Event::End(BytesEnd::new("D:multistatus")))?;

        Ok(())
    }

    /// Write calendar properties as a response
    fn write_calendar_response<W: Write>(
        xml_writer: &mut Writer<W>,
        calendar: &CalendarDto,
        request: &PropFindRequest,
        href: &str,
    ) -> Result<()> {
        // Start response element
        xml_writer.write_event(Event::Start(BytesStart::new("D:response")))?;

        // Write href
        xml_writer.write_event(Event::Start(BytesStart::new("D:href")))?;
        xml_writer.write_event(Event::Text(BytesText::new(href)))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:href")))?;

        // Write propstat
        xml_writer.write_event(Event::Start(BytesStart::new("D:propstat")))?;

        // Start prop
        xml_writer.write_event(Event::Start(BytesStart::new("D:prop")))?;

        // Write properties based on request type
        match &request.prop_find_type {
            PropFindType::AllProp => {
                // Write all standard properties for a calendar
                Self::write_calendar_standard_props(xml_writer, calendar)?;
            }
            PropFindType::PropName => {
                // Write only property names (empty elements)
                Self::write_calendar_prop_names(xml_writer)?;
            }
            PropFindType::Prop(props) => {
                // Write requested properties
                Self::write_calendar_requested_props(xml_writer, calendar, props)?;
            }
        }

        // End prop
        xml_writer.write_event(Event::End(BytesEnd::new("D:prop")))?;

        // Write status
        xml_writer.write_event(Event::Start(BytesStart::new("D:status")))?;
        xml_writer.write_event(Event::Text(BytesText::new("HTTP/1.1 200 OK")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:status")))?;

        // End propstat
        xml_writer.write_event(Event::End(BytesEnd::new("D:propstat")))?;

        // End response
        xml_writer.write_event(Event::End(BytesEnd::new("D:response")))?;

        Ok(())
    }

    /// Write standard calendar properties
    fn write_calendar_standard_props<W: Write>(
        xml_writer: &mut Writer<W>,
        calendar: &CalendarDto,
    ) -> Result<()> {
        // Common WebDAV properties

        // Resource type (collection + calendar)
        xml_writer.write_event(Event::Start(BytesStart::new("D:resourcetype")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:collection")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("C:calendar")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:resourcetype")))?;

        // Display name
        xml_writer.write_event(Event::Start(BytesStart::new("D:displayname")))?;
        xml_writer.write_event(Event::Text(BytesText::new(&calendar.name)))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:displayname")))?;

        // Last modified
        xml_writer.write_event(Event::Start(BytesStart::new("D:getlastmodified")))?;
        xml_writer.write_event(Event::Text(BytesText::new(
            &calendar.updated_at.to_rfc2822(),
        )))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:getlastmodified")))?;

        // ETag
        xml_writer.write_event(Event::Start(BytesStart::new("D:getetag")))?;
        xml_writer.write_event(Event::Text(BytesText::new(&format!("\"{}\"", calendar.id))))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:getetag")))?;

        // Content type for calendar collection
        xml_writer.write_event(Event::Start(BytesStart::new("D:getcontenttype")))?;
        xml_writer.write_event(Event::Text(BytesText::new(
            "text/calendar; component=VCALENDAR",
        )))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:getcontenttype")))?;

        // CalDAV specific properties

        // Supported calendar component set
        xml_writer.write_event(Event::Start(BytesStart::new(
            "C:supported-calendar-component-set",
        )))?;
        xml_writer.write_event(Event::Empty(
            BytesStart::new("C:comp").with_attributes([("name", "VEVENT")]),
        ))?;
        xml_writer.write_event(Event::End(BytesEnd::new(
            "C:supported-calendar-component-set",
        )))?;

        // Calendar timezone (empty for UTC)
        xml_writer.write_event(Event::Empty(BytesStart::new("C:calendar-timezone")))?;

        // Calendar color
        if let Some(color) = &calendar.color {
            xml_writer.write_event(Event::Start(BytesStart::new("CS:calendar-color")))?;
            xml_writer.write_event(Event::Text(BytesText::new(color)))?;
            xml_writer.write_event(Event::End(BytesEnd::new("CS:calendar-color")))?;
        }

        // Support calendar-access (RFC4791)
        xml_writer.write_event(Event::Empty(BytesStart::new("C:calendar-access")))?;

        // Current user privilege set
        xml_writer.write_event(Event::Start(BytesStart::new(
            "D:current-user-privilege-set",
        )))?;
        xml_writer.write_event(Event::Start(BytesStart::new("D:privilege")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:read")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:privilege")))?;

        // Only add write privilege if user owns the calendar or has write access
        if calendar.owner_id == "current_user_id" {
            // This should be replaced with actual user check
            xml_writer.write_event(Event::Start(BytesStart::new("D:privilege")))?;
            xml_writer.write_event(Event::Empty(BytesStart::new("D:write")))?;
            xml_writer.write_event(Event::End(BytesEnd::new("D:privilege")))?;
        }

        xml_writer.write_event(Event::End(BytesEnd::new("D:current-user-privilege-set")))?;

        // Calendar description if present
        if let Some(desc) = &calendar.description {
            xml_writer.write_event(Event::Start(BytesStart::new("C:calendar-description")))?;
            xml_writer.write_event(Event::Text(BytesText::new(desc)))?;
            xml_writer.write_event(Event::End(BytesEnd::new("C:calendar-description")))?;
        }

        // Custom properties
        for (name, value) in &calendar.custom_properties {
            // Skip properties that start with _ - they're internal
            if !name.starts_with('_') {
                xml_writer.write_event(Event::Start(BytesStart::new(format!("CS:{}", name))))?;
                xml_writer.write_event(Event::Text(BytesText::new(value)))?;
                xml_writer.write_event(Event::End(BytesEnd::new(format!("CS:{}", name))))?;
            }
        }

        Ok(())
    }

    /// Write calendar property names
    fn write_calendar_prop_names<W: Write>(xml_writer: &mut Writer<W>) -> Result<()> {
        // Common WebDAV property names
        xml_writer.write_event(Event::Empty(BytesStart::new("D:resourcetype")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:displayname")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:getlastmodified")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:getetag")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:getcontenttype")))?;

        // CalDAV specific property names
        xml_writer.write_event(Event::Empty(BytesStart::new(
            "C:supported-calendar-component-set",
        )))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("C:calendar-timezone")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("CS:calendar-color")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("C:calendar-access")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new(
            "D:current-user-privilege-set",
        )))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("C:calendar-description")))?;

        Ok(())
    }

    /// Write requested calendar properties
    fn write_calendar_requested_props<W: Write>(
        xml_writer: &mut Writer<W>,
        calendar: &CalendarDto,
        props: &[QualifiedName],
    ) -> Result<()> {
        for prop in props {
            match (prop.namespace.as_str(), prop.name.as_str()) {
                // DAV namespace properties
                ("DAV:", "resourcetype") => {
                    xml_writer.write_event(Event::Start(BytesStart::new("D:resourcetype")))?;
                    xml_writer.write_event(Event::Empty(BytesStart::new("D:collection")))?;
                    xml_writer.write_event(Event::Empty(BytesStart::new("C:calendar")))?;
                    xml_writer.write_event(Event::End(BytesEnd::new("D:resourcetype")))?;
                }
                ("DAV:", "displayname") => {
                    xml_writer.write_event(Event::Start(BytesStart::new("D:displayname")))?;
                    xml_writer.write_event(Event::Text(BytesText::new(&calendar.name)))?;
                    xml_writer.write_event(Event::End(BytesEnd::new("D:displayname")))?;
                }
                ("DAV:", "getlastmodified") => {
                    xml_writer.write_event(Event::Start(BytesStart::new("D:getlastmodified")))?;
                    xml_writer.write_event(Event::Text(BytesText::new(
                        &calendar.updated_at.to_rfc2822(),
                    )))?;
                    xml_writer.write_event(Event::End(BytesEnd::new("D:getlastmodified")))?;
                }
                ("DAV:", "getetag") => {
                    xml_writer.write_event(Event::Start(BytesStart::new("D:getetag")))?;
                    xml_writer.write_event(Event::Text(BytesText::new(&format!(
                        "\"{}\"",
                        calendar.id
                    ))))?;
                    xml_writer.write_event(Event::End(BytesEnd::new("D:getetag")))?;
                }
                ("DAV:", "getcontenttype") => {
                    xml_writer.write_event(Event::Start(BytesStart::new("D:getcontenttype")))?;
                    xml_writer.write_event(Event::Text(BytesText::new(
                        "text/calendar; component=VCALENDAR",
                    )))?;
                    xml_writer.write_event(Event::End(BytesEnd::new("D:getcontenttype")))?;
                }
                ("DAV:", "current-user-privilege-set") => {
                    xml_writer.write_event(Event::Start(BytesStart::new(
                        "D:current-user-privilege-set",
                    )))?;
                    xml_writer.write_event(Event::Start(BytesStart::new("D:privilege")))?;
                    xml_writer.write_event(Event::Empty(BytesStart::new("D:read")))?;
                    xml_writer.write_event(Event::End(BytesEnd::new("D:privilege")))?;

                    // Only add write privilege if user owns the calendar or has write access
                    if calendar.owner_id == "current_user_id" {
                        // This should be replaced with actual user check
                        xml_writer.write_event(Event::Start(BytesStart::new("D:privilege")))?;
                        xml_writer.write_event(Event::Empty(BytesStart::new("D:write")))?;
                        xml_writer.write_event(Event::End(BytesEnd::new("D:privilege")))?;
                    }

                    xml_writer
                        .write_event(Event::End(BytesEnd::new("D:current-user-privilege-set")))?;
                }

                // CalDAV namespace properties
                ("urn:ietf:params:xml:ns:caldav", "supported-calendar-component-set") => {
                    xml_writer.write_event(Event::Start(BytesStart::new(
                        "C:supported-calendar-component-set",
                    )))?;
                    xml_writer.write_event(Event::Empty(
                        BytesStart::new("C:comp").with_attributes([("name", "VEVENT")]),
                    ))?;
                    xml_writer.write_event(Event::End(BytesEnd::new(
                        "C:supported-calendar-component-set",
                    )))?;
                }
                ("urn:ietf:params:xml:ns:caldav", "calendar-timezone") => {
                    xml_writer.write_event(Event::Empty(BytesStart::new("C:calendar-timezone")))?;
                }
                ("urn:ietf:params:xml:ns:caldav", "calendar-access") => {
                    xml_writer.write_event(Event::Empty(BytesStart::new("C:calendar-access")))?;
                }
                ("urn:ietf:params:xml:ns:caldav", "calendar-description") => {
                    if let Some(desc) = &calendar.description {
                        xml_writer
                            .write_event(Event::Start(BytesStart::new("C:calendar-description")))?;
                        xml_writer.write_event(Event::Text(BytesText::new(desc)))?;
                        xml_writer
                            .write_event(Event::End(BytesEnd::new("C:calendar-description")))?;
                    } else {
                        xml_writer
                            .write_event(Event::Empty(BytesStart::new("C:calendar-description")))?;
                    }
                }

                // CalendarServer namespace properties
                ("http://calendarserver.org/ns/", "calendar-color") => {
                    if let Some(color) = &calendar.color {
                        xml_writer
                            .write_event(Event::Start(BytesStart::new("CS:calendar-color")))?;
                        xml_writer.write_event(Event::Text(BytesText::new(color)))?;
                        xml_writer.write_event(Event::End(BytesEnd::new("CS:calendar-color")))?;
                    } else {
                        xml_writer
                            .write_event(Event::Empty(BytesStart::new("CS:calendar-color")))?;
                    }
                }

                // Custom properties from the calendar
                _ => {
                    // Check if it's a custom property
                    if let Some(value) = calendar.custom_properties.get(&prop.name) {
                        let prop_name = if prop.namespace == "http://calendarserver.org/ns/" {
                            format!("CS:{}", prop.name)
                        } else if prop.namespace == "urn:ietf:params:xml:ns:caldav" {
                            format!("C:{}", prop.name)
                        } else if prop.namespace == "DAV:" {
                            format!("D:{}", prop.name)
                        } else {
                            format!("{}:{}", prop.namespace, prop.name)
                        };

                        xml_writer.write_event(Event::Start(BytesStart::new(&prop_name)))?;
                        xml_writer.write_event(Event::Text(BytesText::new(value)))?;
                        xml_writer.write_event(Event::End(BytesEnd::new(&prop_name)))?;
                    } else {
                        // Property not found, write empty element
                        let prop_name = if prop.namespace == "http://calendarserver.org/ns/" {
                            format!("CS:{}", prop.name)
                        } else if prop.namespace == "urn:ietf:params:xml:ns:caldav" {
                            format!("C:{}", prop.name)
                        } else if prop.namespace == "DAV:" {
                            format!("D:{}", prop.name)
                        } else {
                            format!("{}:{}", prop.namespace, prop.name)
                        };

                        xml_writer.write_event(Event::Empty(BytesStart::new(&prop_name)))?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Generate PROPFIND response for a single calendar collection + its events
    pub fn generate_calendar_collection_propfind<W: Write>(
        writer: W,
        calendar: &CalendarDto,
        events: &[CalendarEventDto],
        request: &PropFindRequest,
        base_href: &str,
        depth: &str,
    ) -> Result<()> {
        let mut xml_writer = Writer::new(writer);

        xml_writer.write_event(Event::Start(
            BytesStart::new("D:multistatus").with_attributes([
                ("xmlns:D", "DAV:"),
                ("xmlns:C", "urn:ietf:params:xml:ns:caldav"),
                ("xmlns:CS", "http://calendarserver.org/ns/"),
            ]),
        ))?;

        // Write the calendar collection itself
        Self::write_calendar_response(&mut xml_writer, calendar, request, base_href)?;

        // If depth > 0, include event resources
        if depth != "0" {
            for event in events {
                // Write a basic DAV response for each event
                xml_writer.write_event(Event::Start(BytesStart::new("D:response")))?;

                let event_href = format!("{}{}.ics", base_href, event.ical_uid);
                xml_writer.write_event(Event::Start(BytesStart::new("D:href")))?;
                xml_writer.write_event(Event::Text(BytesText::new(&event_href)))?;
                xml_writer.write_event(Event::End(BytesEnd::new("D:href")))?;

                xml_writer.write_event(Event::Start(BytesStart::new("D:propstat")))?;
                xml_writer.write_event(Event::Start(BytesStart::new("D:prop")))?;

                // resourcetype (empty for non-collection)
                xml_writer.write_event(Event::Empty(BytesStart::new("D:resourcetype")))?;

                // getetag
                xml_writer.write_event(Event::Start(BytesStart::new("D:getetag")))?;
                xml_writer
                    .write_event(Event::Text(BytesText::new(&format!("\"{}\"", event.id))))?;
                xml_writer.write_event(Event::End(BytesEnd::new("D:getetag")))?;

                // getcontenttype
                xml_writer.write_event(Event::Start(BytesStart::new("D:getcontenttype")))?;
                xml_writer.write_event(Event::Text(BytesText::new(
                    "text/calendar; component=vevent",
                )))?;
                xml_writer.write_event(Event::End(BytesEnd::new("D:getcontenttype")))?;

                // getlastmodified
                xml_writer.write_event(Event::Start(BytesStart::new("D:getlastmodified")))?;
                xml_writer
                    .write_event(Event::Text(BytesText::new(&event.updated_at.to_rfc2822())))?;
                xml_writer.write_event(Event::End(BytesEnd::new("D:getlastmodified")))?;

                xml_writer.write_event(Event::End(BytesEnd::new("D:prop")))?;

                xml_writer.write_event(Event::Start(BytesStart::new("D:status")))?;
                xml_writer.write_event(Event::Text(BytesText::new("HTTP/1.1 200 OK")))?;
                xml_writer.write_event(Event::End(BytesEnd::new("D:status")))?;

                xml_writer.write_event(Event::End(BytesEnd::new("D:propstat")))?;
                xml_writer.write_event(Event::End(BytesEnd::new("D:response")))?;
            }
        }

        xml_writer.write_event(Event::End(BytesEnd::new("D:multistatus")))?;
        Ok(())
    }

    /// Generate a response for calendar events
    pub fn generate_calendar_events_response<W: Write>(
        writer: W,
        events: &[CalendarEventDto],
        request: &CalDavReportType,
        base_href: &str,
    ) -> Result<()> {
        let mut xml_writer = Writer::new(writer);

        // Start multistatus response
        xml_writer.write_event(Event::Start(
            BytesStart::new("D:multistatus").with_attributes([
                ("xmlns:D", "DAV:"),
                ("xmlns:C", "urn:ietf:params:xml:ns:caldav"),
                ("xmlns:CS", "http://calendarserver.org/ns/"),
            ]),
        ))?;

        // Determine which properties to include based on request type
        let props = match request {
            CalDavReportType::CalendarQuery { props, .. } => props.clone(),
            CalDavReportType::CalendarMultiget { props, .. } => props.clone(),
            CalDavReportType::SyncCollection { props, .. } => props.clone(),
        };

        // Add responses for events
        for event in events {
            // Create the event href based on its UID
            let href = format!("{}{}.ics", base_href, event.ical_uid);

            // Write event response
            Self::write_event_response(&mut xml_writer, event, &props, &href)?;
        }

        // End multistatus
        xml_writer.write_event(Event::End(BytesEnd::new("D:multistatus")))?;

        Ok(())
    }

    /// Write event properties as a response
    fn write_event_response<W: Write>(
        xml_writer: &mut Writer<W>,
        event: &CalendarEventDto,
        props: &[QualifiedName],
        href: &str,
    ) -> Result<()> {
        // Start response element
        xml_writer.write_event(Event::Start(BytesStart::new("D:response")))?;

        // Write href
        xml_writer.write_event(Event::Start(BytesStart::new("D:href")))?;
        xml_writer.write_event(Event::Text(BytesText::new(href)))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:href")))?;

        // Write propstat
        xml_writer.write_event(Event::Start(BytesStart::new("D:propstat")))?;

        // Start prop
        xml_writer.write_event(Event::Start(BytesStart::new("D:prop")))?;

        // If no specific props requested, return all common ones
        if props.is_empty() {
            Self::write_event_standard_props(xml_writer, event)?;
        } else {
            // Write specifically requested properties
            Self::write_event_requested_props(xml_writer, event, props)?;
        }

        // End prop
        xml_writer.write_event(Event::End(BytesEnd::new("D:prop")))?;

        // Write status
        xml_writer.write_event(Event::Start(BytesStart::new("D:status")))?;
        xml_writer.write_event(Event::Text(BytesText::new("HTTP/1.1 200 OK")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:status")))?;

        // End propstat
        xml_writer.write_event(Event::End(BytesEnd::new("D:propstat")))?;

        // End response
        xml_writer.write_event(Event::End(BytesEnd::new("D:response")))?;

        Ok(())
    }

    /// Write standard event properties
    fn write_event_standard_props<W: Write>(
        xml_writer: &mut Writer<W>,
        event: &CalendarEventDto,
    ) -> Result<()> {
        // Common WebDAV properties

        // Resource type (empty for non-collection)
        xml_writer.write_event(Event::Empty(BytesStart::new("D:resourcetype")))?;

        // ETag based on updated_at timestamp
        xml_writer.write_event(Event::Start(BytesStart::new("D:getetag")))?;
        xml_writer.write_event(Event::Text(BytesText::new(&format!("\"{}\"", event.id))))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:getetag")))?;

        // Content type
        xml_writer.write_event(Event::Start(BytesStart::new("D:getcontenttype")))?;
        xml_writer.write_event(Event::Text(BytesText::new(
            "text/calendar; component=VEVENT",
        )))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:getcontenttype")))?;

        // Last modified
        xml_writer.write_event(Event::Start(BytesStart::new("D:getlastmodified")))?;
        xml_writer.write_event(Event::Text(BytesText::new(&event.updated_at.to_rfc2822())))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:getlastmodified")))?;

        // CalDAV specific properties

        // Calendar data (iCalendar format)
        xml_writer.write_event(Event::Start(BytesStart::new("C:calendar-data")))?;
        // In a full implementation, we would generate a complete iCalendar component here
        // For now, we'll just provide a basic example
        let ical_data = format!(
            "BEGIN:VCALENDAR\r\n\
            VERSION:2.0\r\n\
            PRODID:-//OxiCloud//NONSGML Calendar//EN\r\n\
            BEGIN:VEVENT\r\n\
            UID:{}\r\n\
            SUMMARY:{}\r\n\
            DTSTART:{}\r\n\
            DTEND:{}\r\n\
            {}\
            DTSTAMP:{}\r\n\
            END:VEVENT\r\n\
            END:VCALENDAR\r\n",
            event.ical_uid,
            event.summary.replace("\n", "\\n"),
            event.start_time.format("%Y%m%dT%H%M%SZ"),
            event.end_time.format("%Y%m%dT%H%M%SZ"),
            event
                .rrule
                .as_ref()
                .map_or("".to_string(), |r| format!("RRULE:{}\r\n", r)),
            event.updated_at.format("%Y%m%dT%H%M%SZ"),
        );
        xml_writer.write_event(Event::Text(BytesText::new(&ical_data)))?;
        xml_writer.write_event(Event::End(BytesEnd::new("C:calendar-data")))?;

        Ok(())
    }

    /// Write requested event properties
    fn write_event_requested_props<W: Write>(
        xml_writer: &mut Writer<W>,
        event: &CalendarEventDto,
        props: &[QualifiedName],
    ) -> Result<()> {
        for prop in props {
            match (prop.namespace.as_str(), prop.name.as_str()) {
                // DAV namespace properties
                ("DAV:", "resourcetype") => {
                    xml_writer.write_event(Event::Empty(BytesStart::new("D:resourcetype")))?;
                }
                ("DAV:", "getetag") => {
                    xml_writer.write_event(Event::Start(BytesStart::new("D:getetag")))?;
                    xml_writer
                        .write_event(Event::Text(BytesText::new(&format!("\"{}\"", event.id))))?;
                    xml_writer.write_event(Event::End(BytesEnd::new("D:getetag")))?;
                }
                ("DAV:", "getcontenttype") => {
                    xml_writer.write_event(Event::Start(BytesStart::new("D:getcontenttype")))?;
                    xml_writer.write_event(Event::Text(BytesText::new(
                        "text/calendar; component=VEVENT",
                    )))?;
                    xml_writer.write_event(Event::End(BytesEnd::new("D:getcontenttype")))?;
                }
                ("DAV:", "getlastmodified") => {
                    xml_writer.write_event(Event::Start(BytesStart::new("D:getlastmodified")))?;
                    xml_writer
                        .write_event(Event::Text(BytesText::new(&event.updated_at.to_rfc2822())))?;
                    xml_writer.write_event(Event::End(BytesEnd::new("D:getlastmodified")))?;
                }

                // CalDAV namespace properties
                ("urn:ietf:params:xml:ns:caldav", "calendar-data") => {
                    xml_writer.write_event(Event::Start(BytesStart::new("C:calendar-data")))?;
                    // In a full implementation, we would generate a complete iCalendar component here
                    // For now, we'll just provide a basic example
                    let ical_data = format!(
                        "BEGIN:VCALENDAR\r\n\
                        VERSION:2.0\r\n\
                        PRODID:-//OxiCloud//NONSGML Calendar//EN\r\n\
                        BEGIN:VEVENT\r\n\
                        UID:{}\r\n\
                        SUMMARY:{}\r\n\
                        DTSTART:{}\r\n\
                        DTEND:{}\r\n\
                        {}\
                        DTSTAMP:{}\r\n\
                        END:VEVENT\r\n\
                        END:VCALENDAR\r\n",
                        event.ical_uid,
                        event.summary.replace("\n", "\\n"),
                        event.start_time.format("%Y%m%dT%H%M%SZ"),
                        event.end_time.format("%Y%m%dT%H%M%SZ"),
                        event
                            .rrule
                            .as_ref()
                            .map_or("".to_string(), |r| format!("RRULE:{}\r\n", r)),
                        event.updated_at.format("%Y%m%dT%H%M%SZ"),
                    );
                    xml_writer.write_event(Event::Text(BytesText::new(&ical_data)))?;
                    xml_writer.write_event(Event::End(BytesEnd::new("C:calendar-data")))?;
                }

                // Property not supported
                _ => {
                    // Write empty element
                    let prop_name = if prop.namespace == "http://calendarserver.org/ns/" {
                        format!("CS:{}", prop.name)
                    } else if prop.namespace == "urn:ietf:params:xml:ns:caldav" {
                        format!("C:{}", prop.name)
                    } else if prop.namespace == "DAV:" {
                        format!("D:{}", prop.name)
                    } else {
                        format!("{}:{}", prop.namespace, prop.name)
                    };

                    xml_writer.write_event(Event::Empty(BytesStart::new(&prop_name)))?;
                }
            }
        }

        Ok(())
    }

    /// Parse a MKCALENDAR XML request
    pub fn parse_mkcalendar<R: Read>(
        reader: R,
    ) -> Result<(String, Option<String>, Option<String>)> {
        let mut xml_reader = Reader::from_reader(BufReader::new(reader));
        xml_reader.config_mut().trim_text(true);

        let mut buffer = Vec::new();
        let mut in_mkcalendar = false;
        let mut in_set = false;
        let mut in_prop = false;
        let mut in_displayname = false;
        let mut in_description = false;
        let mut in_calendar_color = false;

        let mut displayname = String::new();
        let mut description = None;
        let mut color = None;

        loop {
            match xml_reader.read_event_into(&mut buffer) {
                Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    let name_str = std::str::from_utf8(name.as_ref()).unwrap_or("");

                    match name_str {
                        s if s == "mkcalendar" || s.ends_with(":mkcalendar") => {
                            in_mkcalendar = true
                        }
                        s if in_mkcalendar && (s == "set" || s.ends_with(":set")) => in_set = true,
                        s if in_set && (s == "prop" || s.ends_with(":prop")) => in_prop = true,
                        s if in_prop && (s == "displayname" || s.ends_with(":displayname")) => {
                            in_displayname = true
                        }
                        s if in_prop
                            && (s == "calendar-description"
                                || s.ends_with(":calendar-description")) =>
                        {
                            in_description = true
                        }
                        s if in_prop
                            && (s == "calendar-color" || s.ends_with(":calendar-color")) =>
                        {
                            in_calendar_color = true
                        }
                        _ => (),
                    }
                }
                Ok(Event::Text(e)) => {
                    let text = e.decode().unwrap_or_default();

                    if in_displayname {
                        displayname = text.to_string();
                    } else if in_description {
                        description = Some(text.to_string());
                    } else if in_calendar_color {
                        color = Some(text.to_string());
                    }
                }
                Ok(Event::End(ref e)) => {
                    let name = e.name();
                    let name_str = std::str::from_utf8(name.as_ref()).unwrap_or("");

                    match name_str {
                        s if s == "mkcalendar" || s.ends_with(":mkcalendar") => {
                            in_mkcalendar = false
                        }
                        s if s == "set" || s.ends_with(":set") => in_set = false,
                        s if s == "prop" || s.ends_with(":prop") => in_prop = false,
                        s if s == "displayname" || s.ends_with(":displayname") => {
                            in_displayname = false
                        }
                        s if s == "calendar-description"
                            || s.ends_with(":calendar-description") =>
                        {
                            in_description = false
                        }
                        s if s == "calendar-color" || s.ends_with(":calendar-color") => {
                            in_calendar_color = false
                        }
                        _ => (),
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(WebDavError::XmlError(e)),
                _ => (),
            }

            buffer.clear();
        }

        // If no displayname specified, generate a default one based on UUID
        if displayname.is_empty() {
            displayname = format!("Calendar {}", Uuid::new_v4());
        }

        Ok((displayname, description, color))
    }
}
