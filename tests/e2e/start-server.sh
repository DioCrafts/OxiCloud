#!/usr/bin/env bash
# Boot a clean test DB then start OxiCloud (passed as arguments).
# Used by playwright.config.ts as the webServer command so that both
# `npm test` and `npx playwright test` always start from an empty database.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
OXICLOUD_STORAGE_PATH="$REPO_ROOT/tests/e2e/storage"

# ensure storage is empty before starting
echo "Wipe $OXICLOUD_STORAGE_PATH to ensure clean startup"
rm -rf "$OXICLOUD_STORAGE_PATH"
mkdir -p "$OXICLOUD_STORAGE_PATH"

# Spawn database
bash "$REPO_ROOT/tests/common/spawn-db.sh"

# Replace the shell with the server process so Playwright's PID tracking works.
exec "$@"
