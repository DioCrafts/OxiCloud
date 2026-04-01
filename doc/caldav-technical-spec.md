# 26 - CalDAV Technical Spec

CalDAV (RFC 4791) provides calendar synchronization. Clients like Thunderbird, Apple Calendar, GNOME Calendar, and DAVx5 (Android) connect to manage calendars and events via standard CalDAV.

## Protocol Compliance

- **DAV compliance**: `1, 2, calendar-access`
- **RFC 4791**: Calendar Access (CalDAV)
- **iCalendar**: RFC 5545 (VEVENT parsing and generation)

## Endpoint Structure

All CalDAV endpoints are mounted at the top level (not under `/api`):

```
ANY /caldav          → handle_caldav_methods_root
ANY /caldav/         → handle_caldav_methods_root
ANY /caldav/{*path}  → handle_caldav_methods
```

### Path Hierarchy

| Path | Resource | Description |
|---|---|---|
| `/caldav/` | Calendar home | User's calendar collection |
| `/caldav/{calendar_id}/` | Calendar | Individual calendar |
| `/caldav/{calendar_id}/{ical_uid}.ics` | Event | Individual calendar event |

### Supported HTTP Methods

| Method | Description |
|---|---|
| `OPTIONS` | Returns DAV capabilities and allowed methods |
| `PROPFIND` | List calendars, calendar properties, events |
| `REPORT` | CalendarQuery, CalendarMultiget, SyncCollection |
| `MKCALENDAR` | Create a new calendar |
| `PUT` | Create/update events (iCalendar format) |
| `GET` | Retrieve individual event as iCalendar |
| `DELETE` | Delete calendars or events |
| `PROPPATCH` | Update calendar properties |

## Architecture

| Layer | Component | File |
|---|---|---|
| Domain Entity | **Calendar**, **CalendarEvent** | `src/domain/entities/calendar.rs`, `calendar_event.rs` |
| Domain Repository | **CalendarRepository**, **CalendarEventRepository** | `src/domain/repositories/calendar_repository.rs`, `calendar_event_repository.rs` |
| Application Port | **CalendarUseCase**, **CalendarStoragePort** | `src/application/ports/calendar_ports.rs` |
| Application Service | **CalendarService** | `src/application/services/calendar_service.rs` |
| Application Adapter | **CalDavAdapter** (XML parsing/generation) | `src/application/adapters/caldav_adapter.rs` |
| Infrastructure | **CalendarPgRepository**, **CalendarEventPgRepository** | `src/infrastructure/repositories/pg/` |
| Interfaces | **CalDavHandler** | `src/interfaces/api/handlers/caldav_handler.rs` |

## Domain Entities

### Calendar

```rust
pub struct Calendar {
    id: Uuid,
    name: String,
    owner_id: String,
    description: Option<String>,
    color: Option<String>,               // #RRGGBB format
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    custom_properties: HashMap<String, String>,
}
```

Validation: **name** non-empty, **owner_id** non-empty, **color** must be `#RRGGBB` hex format.

### CalendarEvent

```rust
pub struct CalendarEvent {
    id: Uuid,
    calendar_id: Uuid,
    summary: String,
    description: Option<String>,
    location: Option<String>,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    all_day: bool,
    rrule: Option<String>,               // iCal RRULE format (e.g., "FREQ=WEEKLY")
    ical_uid: String,                    // unique iCal identifier
    ical_data: String,                   // full VEVENT block
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
```

Validation: **summary** non-empty, `end_time >= start_time`, **rrule** starts with `FREQ=`, **ical_data** contains `BEGIN:VEVENT`/`END:VEVENT`.

The constructor `from_ical(calendar_id, ical_data)` parses iCalendar data to extract summary, dates, location, etc.

## REPORT Types

The CalDAV **REPORT** method supports three report types:

### CalendarQuery

Filters events by time range. Used for initial sync and view rendering.

```xml
<C:calendar-query xmlns:C="urn:ietf:params:xml:ns:caldav">
  <D:prop xmlns:D="DAV:">
    <D:getetag/>
    <C:calendar-data/>
  </D:prop>
  <C:filter>
    <C:comp-filter name="VCALENDAR">
      <C:comp-filter name="VEVENT">
        <C:time-range start="20240101T000000Z" end="20240201T000000Z"/>
      </C:comp-filter>
    </C:comp-filter>
  </C:filter>
</C:calendar-query>
```

### CalendarMultiget

Fetches specific events by href. Used for selective sync.

```xml
<C:calendar-multiget xmlns:C="urn:ietf:params:xml:ns:caldav">
  <D:prop xmlns:D="DAV:">
    <D:getetag/>
    <C:calendar-data/>
  </D:prop>
  <D:href>/caldav/cal-1/event-1.ics</D:href>
  <D:href>/caldav/cal-1/event-2.ics</D:href>
</C:calendar-multiget>
```

### SyncCollection

Incremental sync using sync tokens. Used for ongoing synchronization.

```xml
<D:sync-collection xmlns:D="DAV:">
  <D:sync-token>sync-token-value</D:sync-token>
  <D:prop>
    <D:getetag/>
  </D:prop>
</D:sync-collection>
```

## XML Namespaces

| Prefix | Namespace |
|---|---|
| `D:` | `DAV:` |
| `C:` | `urn:ietf:params:xml:ns:caldav` |
| `CS:` | `http://calendarserver.org/ns/` |

## CalDAV Adapter

**CalDavAdapter** in `src/application/adapters/caldav_adapter.rs` handles all XML parsing and generation:

- `parse_report(reader)` -> **CalDavReportType**
- `parse_mkcalendar(reader)` -> `(name, description, color)`
- `generate_calendars_propfind_response(...)` -- multi-calendar PROPFIND
- `generate_calendar_collection_propfind(...)` -- single calendar with events
- `generate_calendar_events_response(...)` -- REPORT response

## Calendar Sharing

Calendars support sharing with access levels:

| Level | Permissions |
|---|---|
| `read` | View calendar and events |
| `write` | Create, modify, delete events |
| `owner` | Full control including sharing and deletion |

## Database Schema

```sql
CREATE SCHEMA IF NOT EXISTS caldav;

-- Calendars
CREATE TABLE caldav.calendars (
    id          UUID PRIMARY KEY,
    name        TEXT NOT NULL,
    owner_id    VARCHAR(36) REFERENCES auth.users(id) ON DELETE CASCADE,
    description TEXT,
    color       VARCHAR(9),
    is_public   BOOLEAN DEFAULT FALSE,
    ctag        VARCHAR(64) DEFAULT '0',
    created_at  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Calendar Events
CREATE TABLE caldav.calendar_events (
    id          UUID PRIMARY KEY,
    calendar_id UUID REFERENCES caldav.calendars(id) ON DELETE CASCADE,
    summary     TEXT NOT NULL,
    description TEXT,
    location    TEXT,
    start_time  TIMESTAMPTZ NOT NULL,
    end_time    TIMESTAMPTZ NOT NULL,
    all_day     BOOLEAN DEFAULT FALSE,
    rrule       TEXT,
    ical_uid    VARCHAR(255) NOT NULL,
    ical_data   TEXT,
    etag        VARCHAR(64),
    created_at  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Calendar Shares
CREATE TABLE caldav.calendar_shares (
    id              SERIAL PRIMARY KEY,
    calendar_id     UUID REFERENCES caldav.calendars(id) ON DELETE CASCADE,
    user_id         VARCHAR(36) REFERENCES auth.users(id) ON DELETE CASCADE,
    access_level    VARCHAR(10) DEFAULT 'read',
    created_at      TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(calendar_id, user_id)
);

-- Calendar Properties
CREATE TABLE caldav.calendar_properties (
    id              SERIAL PRIMARY KEY,
    calendar_id     UUID REFERENCES caldav.calendars(id) ON DELETE CASCADE,
    property_name   TEXT NOT NULL,
    property_value  TEXT NOT NULL,
    UNIQUE(calendar_id, property_name)
);
```

Indexes: `idx_calendars_owner_id`, `idx_calendar_events_calendar_id`, `idx_calendar_events_ical_uid`, `idx_calendar_events_time_range (start_time, end_time)`.

## Client Configuration

See `dav-client-setup.md` for client-specific configuration instructions.

CalDAV URL: `https://oxicloud.example.com/caldav/`
