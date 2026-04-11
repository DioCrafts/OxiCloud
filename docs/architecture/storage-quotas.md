# Storage Quotas

OxiCloud supports per-user storage quotas to limit disk usage.

## Enabling Quotas

```bash
OXICLOUD_ENABLE_USER_STORAGE_QUOTAS=true
```

## How It Works

1. Each user has a `storage_quota` field (in bytes, `0` = unlimited)
2. On every file upload, the current usage is checked against the quota
3. If the upload would exceed the quota, it's rejected with a `413 Payload Too Large` error
4. Admins can view and set quotas via the admin panel or API

## API

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/admin/users/{id}/quota` | Get user's quota and current usage |
| PUT | `/api/admin/users/{id}/quota` | Set user's quota |

## Admin Panel

The admin panel (`/admin.html`) shows each user's current usage vs. quota with a visual progress bar.

## Deduplication Interaction

Storage usage is calculated based on **logical file size** (what the user uploaded), not physical blob size. This means:

- If two users upload the same 100 MB file, each user's quota is charged 100 MB
- But on disk, only one 100 MB blob exists

This ensures fair quota accounting while maintaining dedup benefits.
