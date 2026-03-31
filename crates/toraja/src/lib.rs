#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! Toraja Calendar Implementation
//!
//! This crate provides the toraja calendar system as part of the
//! nusantara-calendar workspace.

extern crate alloc;

/// Placeholder implementation for toraja calendar
#[non_exhaustive]
pub struct TorajaCalendar;

impl Default for TorajaCalendar {
    fn default() -> Self {
        Self::new()
    }
}

impl TorajaCalendar {
    /// Create a new toraja calendar instance
    pub fn new() -> Self {
        Self
    }
}
