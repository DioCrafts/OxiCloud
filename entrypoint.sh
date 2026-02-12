#!/bin/sh
set -e

# Fix ownership of mounted volumes.
# When Docker creates named volumes they are owned by root, but the
# application runs as the unprivileged "oxicloud" user (UID 1001).
# This script runs as root, fixes permissions, then drops privileges.

STORAGE_DIR="/app/storage"
STATIC_DIR="/app/static"

# Ensure the storage directory exists and is writable by oxicloud
if [ -d "$STORAGE_DIR" ]; then
    chown -R oxicloud:oxicloud "$STORAGE_DIR"
fi

# Ensure static directory is readable
if [ -d "$STATIC_DIR" ]; then
    chown -R oxicloud:oxicloud "$STATIC_DIR"
fi

# Drop privileges and exec the main binary (or whatever was passed as CMD)
exec su-exec oxicloud "$@"
