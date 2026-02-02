//! Infrastructure Adapters
//! 
//! This module contains adapters that bridge the gap between domain repositories
//! and application ports. These adapters implement the application layer ports
//! using the infrastructure layer repositories.

pub mod calendar_storage_adapter;
pub mod contact_storage_adapter;

pub use calendar_storage_adapter::CalendarStorageAdapter;
pub use contact_storage_adapter::ContactStorageAdapter;
