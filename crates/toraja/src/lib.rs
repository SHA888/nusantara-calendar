#![cfg_attr(not(feature = "std"), no_std)]

//! toraja Calendar Implementation
//!
//! This crate provides the toraja calendar system as part of the
//! nusantara-calendar workspace.

extern crate alloc;

/// Placeholder implementation for toraja calendar
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
