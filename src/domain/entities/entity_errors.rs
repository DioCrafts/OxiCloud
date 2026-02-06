//! Errores puros de entidades de dominio
//!
//! Este módulo define los errores específicos de las entidades de dominio
//! sin dependencias de frameworks externos, siguiendo los principios de
//! Clean Architecture.
//!
//! Los errores implementan manualmente `std::error::Error` y `std::fmt::Display`
//! para mantener el dominio libre de dependencias externas.

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

// ============================================================================
// FILE ERRORS
// ============================================================================

/// Errores que pueden ocurrir durante operaciones con entidades File
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileError {
    /// Ocurre cuando el nombre de archivo contiene caracteres inválidos o está vacío
    InvalidFileName(String),
    /// Ocurre cuando falla la validación de cualquier atributo de la entidad
    ValidationError(String),
}

impl Display for FileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            FileError::InvalidFileName(name) => write!(f, "Invalid file name: {}", name),
            FileError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl Error for FileError {}

/// Alias de tipo para resultados de operaciones con entidades File
pub type FileResult<T> = Result<T, FileError>;

// ============================================================================
// FOLDER ERRORS
// ============================================================================

/// Errores que pueden ocurrir durante operaciones con entidades Folder
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FolderError {
    /// Ocurre cuando el nombre de carpeta contiene caracteres inválidos o está vacío
    InvalidFolderName(String),
    /// Ocurre cuando falla la validación de cualquier atributo de la entidad
    ValidationError(String),
}

impl Display for FolderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            FolderError::InvalidFolderName(name) => write!(f, "Invalid folder name: {}", name),
            FolderError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl Error for FolderError {}

/// Alias de tipo para resultados de operaciones con entidades Folder
pub type FolderResult<T> = Result<T, FolderError>;

// ============================================================================
// USER ERRORS
// ============================================================================

/// Errores que pueden ocurrir durante operaciones con entidades User
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserError {
    /// Nombre de usuario inválido
    InvalidUsername(String),
    /// Contraseña inválida
    InvalidPassword(String),
    /// Error de validación general
    ValidationError(String),
    /// Error de autenticación
    AuthenticationError(String),
}

impl Display for UserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            UserError::InvalidUsername(msg) => write!(f, "Username inválido: {}", msg),
            UserError::InvalidPassword(msg) => write!(f, "Password inválido: {}", msg),
            UserError::ValidationError(msg) => write!(f, "Error en la validación: {}", msg),
            UserError::AuthenticationError(msg) => write!(f, "Error en la autenticación: {}", msg),
        }
    }
}

impl Error for UserError {}

/// Alias de tipo para resultados de operaciones con entidades User
pub type UserResult<T> = Result<T, UserError>;

// ============================================================================
// SHARE ERRORS
// ============================================================================

/// Errores que pueden ocurrir durante operaciones con entidades Share
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShareError {
    /// Token de compartición inválido
    InvalidToken(String),
    /// Fecha de expiración inválida
    InvalidExpiration(String),
    /// Error de validación general
    ValidationError(String),
}

impl Display for ShareError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ShareError::InvalidToken(msg) => write!(f, "Invalid token: {}", msg),
            ShareError::InvalidExpiration(msg) => write!(f, "Invalid expiration date: {}", msg),
            ShareError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl Error for ShareError {}

/// Alias de tipo para resultados de operaciones con entidades Share
pub type ShareResult<T> = Result<T, ShareError>;

// ============================================================================
// CALENDAR ERRORS
// ============================================================================

/// Errores que pueden ocurrir durante operaciones con entidades Calendar
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalendarError {
    /// Nombre de calendario inválido
    InvalidName(String),
    /// Código de color inválido
    InvalidColor(String),
    /// ID de propietario inválido
    InvalidOwnerId(String),
}

impl Display for CalendarError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            CalendarError::InvalidName(msg) => write!(f, "Invalid calendar name: {}", msg),
            CalendarError::InvalidColor(msg) => write!(f, "Invalid color code: {}", msg),
            CalendarError::InvalidOwnerId(msg) => write!(f, "Invalid owner ID: {}", msg),
        }
    }
}

impl Error for CalendarError {}

/// Alias de tipo para resultados de operaciones con entidades Calendar
pub type CalendarResult<T> = Result<T, CalendarError>;

// ============================================================================
// CALENDAR EVENT ERRORS
// ============================================================================

/// Errores que pueden ocurrir durante operaciones con entidades CalendarEvent
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalendarEventError {
    /// Resumen/título de evento inválido
    InvalidSummary(String),
    /// Fechas de evento inválidas
    InvalidDates(String),
    /// Regla de recurrencia inválida
    InvalidRecurrence(String),
    /// Datos iCalendar inválidos
    InvalidICalData(String),
}

impl Display for CalendarEventError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            CalendarEventError::InvalidSummary(msg) => write!(f, "Invalid event summary: {}", msg),
            CalendarEventError::InvalidDates(msg) => write!(f, "Invalid event dates: {}", msg),
            CalendarEventError::InvalidRecurrence(msg) => write!(f, "Invalid recurrence rule: {}", msg),
            CalendarEventError::InvalidICalData(msg) => write!(f, "Invalid iCalendar data: {}", msg),
        }
    }
}

impl Error for CalendarEventError {}

/// Alias de tipo para resultados de operaciones con entidades CalendarEvent
pub type CalendarEventResult<T> = Result<T, CalendarEventError>;

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_error_display() {
        let err = FileError::InvalidFileName("test.txt".to_string());
        assert_eq!(err.to_string(), "Invalid file name: test.txt");
        
        let err = FileError::ValidationError("size too large".to_string());
        assert_eq!(err.to_string(), "Validation error: size too large");
    }

    #[test]
    fn test_folder_error_display() {
        let err = FolderError::InvalidFolderName("my/folder".to_string());
        assert_eq!(err.to_string(), "Invalid folder name: my/folder");
    }

    #[test]
    fn test_user_error_display() {
        let err = UserError::InvalidUsername("".to_string());
        assert_eq!(err.to_string(), "Username inválido: ");
        
        let err = UserError::AuthenticationError("invalid credentials".to_string());
        assert_eq!(err.to_string(), "Error en la autenticación: invalid credentials");
    }

    #[test]
    fn test_share_error_display() {
        let err = ShareError::InvalidToken("abc123".to_string());
        assert_eq!(err.to_string(), "Invalid token: abc123");
    }

    #[test]
    fn test_calendar_error_display() {
        let err = CalendarError::InvalidColor("not-a-color".to_string());
        assert_eq!(err.to_string(), "Invalid color code: not-a-color");
    }

    #[test]
    fn test_calendar_event_error_display() {
        let err = CalendarEventError::InvalidDates("end before start".to_string());
        assert_eq!(err.to_string(), "Invalid event dates: end before start");
    }

    #[test]
    fn test_errors_implement_error_trait() {
        fn assert_error<E: Error>() {}
        
        assert_error::<FileError>();
        assert_error::<FolderError>();
        assert_error::<UserError>();
        assert_error::<ShareError>();
        assert_error::<CalendarError>();
        assert_error::<CalendarEventError>();
    }
}
