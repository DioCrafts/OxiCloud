# CalDAV & CardDAV

OxiCloud provides built-in CalDAV (calendar) and CardDAV (contacts) servers — no extra apps or plugins needed.

## CalDAV (Calendars)

### Endpoint

```
https://your-server:8086/caldav/
```

### Protocol Compliance

- RFC 4791 (Calendar Access)
- RFC 5545 (iCalendar format)
- DAV capabilities: `1, 2, calendar-access`

### Route Structure

CalDAV is mounted at the top level, not under `/api`:

- `/caldav`
- `/caldav/`
- `/caldav/{*path}`

OxiCloud also exposes `/.well-known/caldav` and redirects it to `/caldav/`.

Typical resource shapes:

- `/caldav/` for the calendar home
- `/caldav/{calendar_id}/` for one calendar
- `/caldav/{calendar_id}/{ical_uid}.ics` for one event

### Supported Methods

- `OPTIONS`
- `PROPFIND`
- `REPORT`
- `MKCALENDAR`
- `PUT`
- `GET`
- `DELETE`
- `PROPPATCH`

### Client Setup

| Client | URL |
|--------|-----|
| Thunderbird | `https://your-server:8086/caldav/` |
| GNOME Calendar | `https://your-server:8086/caldav/` |
| Apple Calendar (macOS/iOS) | `https://your-server:8086/caldav/` |
| DAVx⁵ (Android) | `https://your-server:8086/` (auto-discovery) |

### Thunderbird Setup

1. Open Thunderbird → **Calendar** tab
2. Right-click → **New Calendar** → **On the Network**
3. Format: **CalDAV**
4. URL: `https://your-server:8086/caldav/`
5. Enter your OxiCloud credentials

---

## CardDAV (Contacts)

### Endpoint

```
https://your-server:8086/carddav/
```

### Protocol Compliance

- RFC 6352 (CardDAV)
- RFC 6350 (vCard 4.0)

### Route Structure

CardDAV is also mounted at the top level:

- `/carddav`
- `/carddav/`
- `/carddav/{*path}`

Typical resource shapes:

- `/carddav/` for the address book home
- `/carddav/{addressBookId}/` for one address book
- `/carddav/{addressBookId}/{contactId}.vcf` for one contact

### Supported Methods

- `OPTIONS`
- `PROPFIND`
- `REPORT`
- `MKCOL`
- `PUT`
- `GET`
- `DELETE`
- `PROPPATCH`

### Client Setup

| Client | URL |
|--------|-----|
| Thunderbird | `https://your-server:8086/carddav/` |
| GNOME Contacts | `https://your-server:8086/carddav/` |
| Apple Contacts (macOS/iOS) | `https://your-server:8086/carddav/` |
| DAVx⁵ (Android) | `https://your-server:8086/` (auto-discovery) |

### DAVx⁵ (Android) Setup

1. Install [DAVx⁵](https://www.davx5.com/) from F-Droid or Play Store
2. Add account → **Login with URL and user name**
3. Base URL: `https://your-server:8086/`
4. Enter your OxiCloud credentials
5. DAVx⁵ auto-discovers both CalDAV and CardDAV endpoints

::: info
DAVx⁵ file sync works. CalDAV/CardDAV support on DAVx⁵ is still being refined.
:::

## Client Setup

For platform-specific instructions, see [DAV Client Setup](/guide/dav-client-setup).
