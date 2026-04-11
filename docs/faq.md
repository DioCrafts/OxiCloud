# FAQ

## General

### What is OxiCloud?

OxiCloud is a self-hosted cloud platform written in Rust. It provides file storage, calendar sync, contacts sync, and office editing from a single binary.

### How does it compare to NextCloud?

OxiCloud uses ~20× less RAM, produces a ~25× smaller Docker image, and starts in under 1 second. The trade-off is fewer plugins — OxiCloud builds everything into the core. See the [comparison table](/guide/#oxicloud-vs-nextcloud).

### What hardware do I need?

Minimum: 1 vCPU, 512 MB RAM, and a few GB of disk for PostgreSQL + file storage. OxiCloud runs comfortably on a Raspberry Pi 4.

---

## Installation

### Which database is supported?

PostgreSQL 13 or later. SQLite is not supported — PostgreSQL's ltree, array operations, and concurrent access are essential.

### Can I use MySQL / MariaDB?

Not currently. PostgreSQL is the only supported database.

### How do I update?

Pull the latest image and restart:

```bash
docker compose pull
docker compose up -d
```

---

## Features

### Can I sync files from my desktop?

WebDAV is built-in. You can mount OxiCloud in Windows Explorer, macOS Finder, or Linux file managers. A dedicated desktop sync client is on the roadmap.

### Does CalDAV/CardDAV work with my phone?

On Android, use [DAVx⁵](https://www.davx5.com/). On iOS, CalDAV and CardDAV work via the built-in Accounts settings.

### Can I edit Office documents?

Yes, via WOPI integration with Collabora Online or OnlyOffice. See [WOPI configuration](/config/wopi).

### Is there end-to-end encryption?

Not yet — it's on the roadmap. Files are stored unencrypted on the server (standard for self-hosted solutions that support server-side processing like thumbnails and search).

---

## Troubleshooting

### I can't connect via WebDAV

1. Check that the URL is `http(s)://host:8086/webdav/` (note the trailing slash)
2. Ensure authentication is correct (HTTP Basic)
3. Check server logs: `docker compose logs oxicloud`

### Uploads fail for large files

OxiCloud automatically switches to chunked upload for files over 200 MB. If you're behind a reverse proxy, ensure it allows large request bodies:

```nginx
client_max_body_size 0;  # unlimited
proxy_read_timeout 600s;
```

### OIDC login doesn't redirect back

Verify that `OXICLOUD_OIDC_REDIRECT_URI` matches exactly what's configured in your identity provider, and that `OXICLOUD_OIDC_FRONTEND_URL` points to your actual frontend URL.
