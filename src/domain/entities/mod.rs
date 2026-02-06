pub mod calendar;
pub mod calendar_event;
pub mod contact;
pub mod entity_errors;
pub mod file;
pub mod folder;
pub mod user;
pub mod session;
pub mod share;
pub mod trashed_item;

// Re-exportar errores de entidades para facilitar el uso
pub use entity_errors::{
    FileError, FileResult,
    FolderError, FolderResult,
    UserError, UserResult,
    ShareError, ShareResult,
    CalendarError, CalendarResult,
    CalendarEventError, CalendarEventResult,
};