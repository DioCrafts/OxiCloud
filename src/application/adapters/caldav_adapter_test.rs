#[cfg(test)]
mod tests {
    use crate::application::adapters::caldav_adapter::{CalDavAdapter, CalDavReportType};
    use crate::application::adapters::webdav_adapter::{
        PropFindRequest, PropFindType, QualifiedName,
    };
    use crate::application::dtos::calendar_dto::{CalendarDto, CalendarEventDto};
    use chrono::{TimeZone, Utc};
    use std::collections::HashMap;
    use std::io::Cursor;

    fn sample_calendar() -> CalendarDto {
        CalendarDto {
            id: "cal-001".to_string(),
            name: "Personal".to_string(),
            owner_id: "user-001".to_string(),
            description: Some("My personal calendar".to_string()),
            color: Some("#FF0000".to_string()),
            is_public: false,
            created_at: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2025, 1, 15, 12, 0, 0).unwrap(),
            custom_properties: HashMap::new(),
        }
    }

    fn sample_event() -> CalendarEventDto {
        CalendarEventDto {
            id: "evt-001".to_string(),
            calendar_id: "cal-001".to_string(),
            summary: "Team Meeting".to_string(),
            description: Some("Weekly team sync".to_string()),
            location: Some("Conference Room A".to_string()),
            start_time: Utc.with_ymd_and_hms(2025, 6, 15, 10, 0, 0).unwrap(),
            end_time: Utc.with_ymd_and_hms(2025, 6, 15, 11, 0, 0).unwrap(),
            all_day: false,
            rrule: None,
            ical_uid: "uid-evt-001@oxicloud".to_string(),
            created_at: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        }
    }

    // ========================
    // MKCALENDAR parsing tests
    // ========================

    #[test]
    fn test_parse_mkcalendar_full() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <C:mkcalendar xmlns:D="DAV:" xmlns:C="urn:ietf:params:xml:ns:caldav">
            <D:set>
                <D:prop>
                    <D:displayname>Work Calendar</D:displayname>
                    <C:calendar-description>Work related events</C:calendar-description>
                    <A:calendar-color xmlns:A="http://apple.com/ns/ical/">#0000FF</A:calendar-color>
                </D:prop>
            </D:set>
        </C:mkcalendar>"#;

        let result = CalDavAdapter::parse_mkcalendar(Cursor::new(xml));
        assert!(
            result.is_ok(),
            "Failed to parse MKCALENDAR: {:?}",
            result.err()
        );
        let (name, desc, color) = result.unwrap();
        assert_eq!(name, "Work Calendar");
        assert_eq!(desc, Some("Work related events".to_string()));
        assert_eq!(color, Some("#0000FF".to_string()));
    }

    #[test]
    fn test_parse_mkcalendar_name_only() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <C:mkcalendar xmlns:D="DAV:" xmlns:C="urn:ietf:params:xml:ns:caldav">
            <D:set>
                <D:prop>
                    <D:displayname>Minimal Calendar</D:displayname>
                </D:prop>
            </D:set>
        </C:mkcalendar>"#;

        let result = CalDavAdapter::parse_mkcalendar(Cursor::new(xml));
        assert!(result.is_ok());
        let (name, desc, color) = result.unwrap();
        assert_eq!(name, "Minimal Calendar");
        assert!(desc.is_none());
        assert!(color.is_none());
    }

    // ========================
    // REPORT parsing tests
    // ========================

    #[test]
    fn test_parse_calendar_query_report() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <C:calendar-query xmlns:D="DAV:" xmlns:C="urn:ietf:params:xml:ns:caldav">
            <D:prop>
                <D:getetag/>
                <C:calendar-data/>
            </D:prop>
            <C:filter>
                <C:comp-filter name="VCALENDAR">
                    <C:comp-filter name="VEVENT">
                        <C:time-range start="2025-06-01T00:00:00Z" end="2025-07-01T00:00:00Z"/>
                    </C:comp-filter>
                </C:comp-filter>
            </C:filter>
        </C:calendar-query>"#;

        let result = CalDavAdapter::parse_report(Cursor::new(xml));
        assert!(result.is_ok(), "Failed to parse report: {:?}", result.err());

        match result.unwrap() {
            CalDavReportType::CalendarQuery { time_range, props } => {
                assert!(time_range.is_some(), "Time range should be parsed");
                let (start, end) = time_range.unwrap();
                assert_eq!(start, Utc.with_ymd_and_hms(2025, 6, 1, 0, 0, 0).unwrap());
                assert_eq!(end, Utc.with_ymd_and_hms(2025, 7, 1, 0, 0, 0).unwrap());
                assert!(!props.is_empty(), "Props should not be empty");
            }
            other => panic!("Expected CalendarQuery, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_calendar_multiget_report() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <C:calendar-multiget xmlns:D="DAV:" xmlns:C="urn:ietf:params:xml:ns:caldav">
            <D:prop>
                <D:getetag/>
                <C:calendar-data/>
            </D:prop>
            <D:href>/caldav/cal-001/evt-001.ics</D:href>
            <D:href>/caldav/cal-001/evt-002.ics</D:href>
        </C:calendar-multiget>"#;

        let result = CalDavAdapter::parse_report(Cursor::new(xml));
        assert!(
            result.is_ok(),
            "Failed to parse multiget: {:?}",
            result.err()
        );

        match result.unwrap() {
            CalDavReportType::CalendarMultiget { hrefs, props } => {
                assert_eq!(hrefs.len(), 2);
                assert_eq!(hrefs[0], "/caldav/cal-001/evt-001.ics");
                assert_eq!(hrefs[1], "/caldav/cal-001/evt-002.ics");
                assert!(!props.is_empty());
            }
            other => panic!("Expected CalendarMultiget, got {:?}", other),
        }
    }

    // ========================
    // PROPFIND response tests
    // ========================

    #[test]
    fn test_generate_calendars_propfind_response() {
        let calendars = vec![sample_calendar()];
        let request = PropFindRequest {
            prop_find_type: PropFindType::AllProp,
        };

        let mut output = Vec::new();
        let result = CalDavAdapter::generate_calendars_propfind_response(
            &mut output,
            &calendars,
            &request,
            "/caldav/",
        );

        assert!(
            result.is_ok(),
            "Failed to generate propfind response: {:?}",
            result.err()
        );

        let xml_str = String::from_utf8(output).expect("Invalid UTF-8 in response");
        assert!(
            xml_str.contains("multistatus"),
            "Response should contain multistatus element"
        );
        assert!(
            xml_str.contains("Personal"),
            "Response should contain calendar name"
        );
        assert!(
            xml_str.contains("cal-001"),
            "Response should contain calendar ID in href"
        );
    }

    #[test]
    fn test_generate_calendar_collection_propfind_depth_0() {
        let calendar = sample_calendar();
        let events = vec![sample_event()];
        let request = PropFindRequest {
            prop_find_type: PropFindType::AllProp,
        };

        let mut output = Vec::new();
        let result = CalDavAdapter::generate_calendar_collection_propfind(
            &mut output,
            &calendar,
            &events,
            &request,
            "/caldav/cal-001",
            "0",
        );

        assert!(
            result.is_ok(),
            "Failed to generate collection propfind: {:?}",
            result.err()
        );

        let xml_str = String::from_utf8(output).expect("Invalid UTF-8");
        assert!(xml_str.contains("multistatus"), "Should have multistatus");
        assert!(xml_str.contains("Personal"), "Should have calendar name");
        // Depth 0 should NOT include individual event resources
    }

    #[test]
    fn test_generate_calendar_collection_propfind_depth_1() {
        let calendar = sample_calendar();
        let events = vec![sample_event()];
        let request = PropFindRequest {
            prop_find_type: PropFindType::AllProp,
        };

        let mut output = Vec::new();
        let result = CalDavAdapter::generate_calendar_collection_propfind(
            &mut output,
            &calendar,
            &events,
            &request,
            "/caldav/cal-001",
            "1",
        );

        assert!(
            result.is_ok(),
            "Failed to generate depth-1 propfind: {:?}",
            result.err()
        );

        let xml_str = String::from_utf8(output).expect("Invalid UTF-8");
        assert!(xml_str.contains("multistatus"), "Should have multistatus");
        assert!(xml_str.contains("Personal"), "Should have calendar name");
        // Depth 1 should include event resources
        assert!(
            xml_str.contains("evt-001"),
            "Depth 1 should include event resources"
        );
    }

    // ========================
    // Calendar events response tests
    // ========================

    #[test]
    fn test_generate_calendar_events_response() {
        let events = vec![sample_event()];
        let report = CalDavReportType::CalendarQuery {
            time_range: None,
            props: vec![
                QualifiedName {
                    namespace: "DAV:".to_string(),
                    name: "getetag".to_string(),
                },
                QualifiedName {
                    namespace: "urn:ietf:params:xml:ns:caldav".to_string(),
                    name: "calendar-data".to_string(),
                },
            ],
        };

        let mut output = Vec::new();
        let result = CalDavAdapter::generate_calendar_events_response(
            &mut output,
            &events,
            &report,
            "/caldav/cal-001",
        );

        assert!(
            result.is_ok(),
            "Failed to generate events response: {:?}",
            result.err()
        );

        let xml_str = String::from_utf8(output).expect("Invalid UTF-8");
        assert!(xml_str.contains("multistatus"), "Should have multistatus");
        assert!(xml_str.contains("evt-001"), "Should reference event ID");
        assert!(
            xml_str.contains("BEGIN:VCALENDAR"),
            "Should contain iCal data"
        );
        assert!(
            xml_str.contains("VEVENT"),
            "Should contain VEVENT component"
        );
        assert!(
            xml_str.contains("Team Meeting"),
            "Should contain event summary"
        );
    }

    #[test]
    fn test_generate_empty_events_response() {
        let events: Vec<CalendarEventDto> = vec![];
        let report = CalDavReportType::CalendarQuery {
            time_range: None,
            props: vec![],
        };

        let mut output = Vec::new();
        let result = CalDavAdapter::generate_calendar_events_response(
            &mut output,
            &events,
            &report,
            "/caldav/cal-001",
        );

        assert!(
            result.is_ok(),
            "Empty events should still produce valid response"
        );
        let xml_str = String::from_utf8(output).expect("Invalid UTF-8");
        assert!(
            xml_str.contains("multistatus"),
            "Should have multistatus even for empty"
        );
    }
}
