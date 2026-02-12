# 03 - File System Safety

OxiCloud ensures data integrity and durability during file operations through atomic writes, fsync, and directory synchronization. The goal: writes either complete fully or not at all, data reaches persistent storage, and the system recovers from crashes or power loss.

---

## The Problem: Buffered I/O

Standard filesystem operations use buffered I/O by default:

```rust
// This operation may not immediately persist to disk
fs::write(path, content)
```

When an application writes data, the OS typically:

1. Accepts the write into memory buffers
2. Acknowledges completion to the application
3. Schedules the actual disk write for later

A crash during that window means data loss -- the data exists only in memory buffers that haven't been flushed.

---

## OxiCloud's Approach

All safety mechanisms live in the **FileSystemUtils** service.

### Atomic Write Pattern

Files are written using write-then-rename:

```rust
/// Writes data to a file with fsync to ensure durability
/// Uses a safe atomic write pattern: write to temp file, fsync, rename
pub async fn atomic_write<P: AsRef<Path>>(path: P, contents: &[u8]) -> Result<(), IoError>
```

Steps:
1. Write to a temporary file in the same directory
2. Call `fsync` to ensure data is on disk
3. Atomically rename the temp file to the target file
4. Sync the parent directory to ensure the rename is persisted

### Directory Synchronization

```rust
/// Creates directories with fsync
pub async fn create_dir_with_sync<P: AsRef<Path>>(path: P) -> Result<(), IoError>
```

Directories are created, their entries persisted to disk, and parent directories synchronized too.

### Rename and Delete Operations

```rust
/// Renames a file or directory with proper syncing
pub async fn rename_with_sync<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<(), IoError>

/// Removes a file with directory syncing
pub async fn remove_file_with_sync<P: AsRef<Path>>(path: P) -> Result<(), IoError>
```

Both complete the operation itself, then update and sync the parent directory entry.

---

## Implementation Details

### fsync on Files

```rust
// Write file content
file.write_all(contents).await?;

// Ensure data is synced to disk
file.flush().await?;
file.sync_all().await?;
```

`sync_all()` instructs the OS to flush data and metadata to the physical storage device.

### fsync on Directories

```rust
// Sync a directory to ensure its contents (entries) are durable
async fn sync_directory<P: AsRef<Path>>(path: P) -> Result<(), IoError> {
    let dir_file = OpenOptions::new().read(true).open(path).await?;
    dir_file.sync_all().await
}
```

Required after any operation that modifies directory entries (create, rename, delete).

---

## Usage in the Codebase

### File Write Repository

```rust
// Write the file to disk using atomic write with fsync
tokio::time::timeout(
    self.config.timeouts.file_write_timeout(),
    FileSystemUtils::atomic_write(&abs_path, &content)
).await
```

### File Move Operations

```rust
// Move the file physically with fsync
time::timeout(
    self.config.timeouts.file_timeout(),
    FileSystemUtils::rename_with_sync(&old_abs_path, &new_abs_path)
).await
```

### Directory Creation

```rust
// Ensure the parent directory exists with proper syncing
self.ensure_parent_directory(&abs_path).await?;

// Implementation uses FileSystemUtils
async fn ensure_parent_directory(&self, abs_path: &PathBuf) -> FileRepositoryResult<()> {
    if let Some(parent) = abs_path.parent() {
        time::timeout(
            self.config.timeouts.dir_timeout(),
            FileSystemUtils::create_dir_with_sync(parent)
        ).await
    }
}
```

---

## Benefits

1. **Data durability** -- critical data is synced to persistent storage
2. **Crash resilience** -- recovery from unexpected failures without data loss
3. **Consistency** -- file operations maintain a consistent filesystem state
4. **Atomic operations** -- file writes appear as all-or-nothing

---

## Performance Considerations

Syncing to disk costs more than buffered writes. OxiCloud mitigates this by:

1. Applying these measures only to critical operations
2. Using timeouts to prevent indefinite blocking
3. Implementing parallel processing for large files

The tradeoff favors safety for critical data while maintaining good performance for most operations.
