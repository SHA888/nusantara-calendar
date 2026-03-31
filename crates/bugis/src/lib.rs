#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! Bugis Calendar Implementation
//!
//! This crate provides the bugis calendar system as part of the
//! nusantara-calendar workspace.

extern crate alloc;

/// Placeholder implementation for bugis calendar
#[non_exhaustive]
pub struct BugisCalendar;

impl Default for BugisCalendar {
    fn default() -> Self {
        Self::new()
    }
}

impl BugisCalendar {
    /// Create a new bugis calendar instance
    pub fn new() -> Self {
        Self
    }
}
