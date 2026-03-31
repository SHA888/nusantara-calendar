#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! Tengger Calendar Implementation
//!
//! This crate provides the tengger calendar system as part of the
//! nusantara-calendar workspace.

extern crate alloc;

/// Placeholder implementation for tengger calendar
#[non_exhaustive]
pub struct TenggerCalendar;

impl Default for TenggerCalendar {
    fn default() -> Self {
        Self::new()
    }
}

impl TenggerCalendar {
    /// Create a new tengger calendar instance
    pub fn new() -> Self {
        Self
    }
}
