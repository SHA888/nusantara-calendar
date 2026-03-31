#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! Sunda Calendar Implementation
//!
//! This crate provides the sunda calendar system as part of the
//! nusantara-calendar workspace.

extern crate alloc;

/// Placeholder implementation for sunda calendar
#[non_exhaustive]
pub struct SundaCalendar;

impl Default for SundaCalendar {
    fn default() -> Self {
        Self::new()
    }
}

impl SundaCalendar {
    /// Create a new sunda calendar instance
    pub fn new() -> Self {
        Self
    }
}
