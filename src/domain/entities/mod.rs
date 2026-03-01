pub mod calendar;
pub mod calendar_event;
pub mod contact;
pub mod device_code;
pub mod entity_errors;
pub mod file;
pub mod folder;
pub mod session;
pub mod share;
pub mod trashed_item;
pub mod user;

// Re-exportar errores de entidades para facilitar el uso
pub use entity_errors::{
    CalendarError, CalendarEventError, CalendarEventResult, CalendarResult, FileError, FileResult,
    FolderError, FolderResult, ShareError, ShareResult, UserError, UserResult,
};
