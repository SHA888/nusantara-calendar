#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! batak Calendar Implementation
//!
//! This crate provides the batak calendar system as part of the
//! nusantara-calendar workspace.

extern crate alloc;

/// Placeholder implementation for batak calendar
pub struct BatakCalendar;

impl Default for BatakCalendar {
    fn default() -> Self {
        Self::new()
    }
}

impl BatakCalendar {
    /// Create a new batak calendar instance
    pub fn new() -> Self {
        Self
    }
}
