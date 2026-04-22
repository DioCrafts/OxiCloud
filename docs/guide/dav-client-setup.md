# DAV Client Setup

This page collects platform-specific connection steps for OxiCloud's WebDAV, CalDAV, and CardDAV endpoints.

## Connection Summary

| Use case | URL |
| --- | --- |
| WebDAV file access | `https://your-oxicloud-server/webdav/` |
| CalDAV calendar sync | `https://your-oxicloud-server/caldav` |
| CardDAV contact sync | `https://your-oxicloud-server/carddav` |

## WebDAV

### Windows Explorer

1. Open File Explorer
2. Right-click This PC and choose Add a network location or Map network drive
3. Enter `https://your-oxicloud-server/webdav/`
4. Provide your OxiCloud username and password

If Windows refuses the connection, check the `WebClient` service and verify these registry values under `HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\WebClient\Parameters`:

- `BasicAuthLevel = 2` when Basic auth is required
- `FileSizeLimitInBytes` if you need to allow larger transfers

### macOS Finder

1. Open Finder
2. Choose Go -> Connect to Server or press Cmd+K
3. Enter `https://your-oxicloud-server/webdav/`
4. Sign in with your OxiCloud credentials

### Linux

- GNOME Files: use `davs://your-oxicloud-server/webdav/`
- KDE Dolphin: use `webdavs://your-oxicloud-server/webdav/`
- `davfs2`: mount `https://your-oxicloud-server/webdav/` to a local directory

## CalDAV

### Apple Calendar

Use an advanced CalDAV account and point it at `https://your-oxicloud-server/caldav`.

### Thunderbird

Create a network calendar and use a CalDAV location such as:

```text
https://your-oxicloud-server/caldav/calendars/your-calendar-id
```

### Android with DAVx5

Use Login with URL and username, then point the base URL at `https://your-oxicloud-server/caldav`.

### Outlook on Windows

Use a CalDAV plugin such as CalDAV Synchronizer and register the calendar endpoint explicitly.

## CardDAV

### Apple Contacts

Create a CardDAV account using `https://your-oxicloud-server/carddav`.

### Thunderbird

Use a remote address book with a URL such as:

```text
https://your-oxicloud-server/carddav/address-books/your-address-book-id
```

### Android with DAVx5

Use the CardDAV base URL `https://your-oxicloud-server/carddav`.

### Outlook on Windows

Use a CardDAV-capable synchronizer and configure the remote address book endpoint explicitly.

## Troubleshooting

### WebDAV

- Make sure the URL includes `/webdav/`
- Use HTTPS in production
- Recheck credentials and the WebClient service on Windows

### CalDAV and CardDAV

- Use the full `/caldav` or `/carddav` base path
- Verify the calendar or address book identifier when the client asks for one
- If sync works on one client and not another, compare the exact URLs being used

## Related Pages

- [WebDAV](/guide/webdav)
- [CalDAV & CardDAV](/guide/caldav-carddav)