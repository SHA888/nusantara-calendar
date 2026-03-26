#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! hijriyah Calendar Implementation
//!
//! This crate provides the hijriyah calendar system as part of the
//! nusantara-calendar workspace.

extern crate alloc;

/// Placeholder implementation for hijri calendar
pub struct HijriyahCalendar;

impl Default for HijriyahCalendar {
    fn default() -> Self {
        Self::new()
    }
}

impl HijriyahCalendar {
    /// Create a new Hijri calendar instance
    pub fn new() -> Self {
        Self
    }
}
