//! Adapters module for translating between external protocols and internal models

pub mod webdav_adapter;
pub mod caldav_adapter;
pub mod carddav_adapter;

#[cfg(test)]
mod caldav_adapter_test;
#[cfg(test)]
mod carddav_adapter_test;
