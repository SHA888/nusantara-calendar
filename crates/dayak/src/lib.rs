#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! Dayak Calendar Implementation
//!
//! This crate provides the dayak calendar system as part of the
//! nusantara-calendar workspace.

extern crate alloc;

/// Placeholder implementation for dayak calendar
#[non_exhaustive]
pub struct DayakCalendar;

impl Default for DayakCalendar {
    fn default() -> Self {
        Self::new()
    }
}

impl DayakCalendar {
    /// Create a new dayak calendar instance
    pub fn new() -> Self {
        Self
    }
}
