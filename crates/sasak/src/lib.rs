#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! Sasak Calendar Implementation
//!
//! This crate provides the sasak calendar system as part of the
//! nusantara-calendar workspace.

extern crate alloc;

/// Placeholder implementation for sasak calendar
#[non_exhaustive]
pub struct SasakCalendar;

impl Default for SasakCalendar {
    fn default() -> Self {
        Self::new()
    }
}

impl SasakCalendar {
    /// Create a new sasak calendar instance
    pub fn new() -> Self {
        Self
    }
}
