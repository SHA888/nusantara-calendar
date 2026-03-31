#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! Javanese Calendar Implementation
//!
//! This crate provides the Javanese calendar system as part of the
//! nusantara-calendar workspace.

extern crate alloc;

/// Placeholder implementation for Javanese calendar
///
/// This is a temporary placeholder for the Javanese calendar system.
/// The full implementation will be added in a future release.
#[non_exhaustive]
pub struct JawaCalendar;

impl Default for JawaCalendar {
    fn default() -> Self {
        Self::new()
    }
}

impl JawaCalendar {
    /// Create a new Javanese calendar instance
    pub fn new() -> Self {
        Self
    }
}
