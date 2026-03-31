#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! Minangkabau Calendar Implementation
//!
//! This crate provides the minangkabau calendar system as part of the
//! nusantara-calendar workspace.

extern crate alloc;

/// Placeholder implementation for minangkabau calendar
#[non_exhaustive]
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
