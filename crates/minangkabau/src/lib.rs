#![cfg_attr(not(feature = "std"), no_std)]

//! minangkabau Calendar Implementation
//!
//! This crate provides the minangkabau calendar system as part of the
//! nusantara-calendar workspace.

extern crate alloc;

/// Placeholder implementation for minangkabau calendar
pub struct MinangkabauCalendar;

impl Default for MinangkabauCalendar {
    fn default() -> Self {
        Self::new()
    }
}

impl MinangkabauCalendar {
    /// Create a new minangkabau calendar instance
    pub fn new() -> Self {
        Self
    }
}
