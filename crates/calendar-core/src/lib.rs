#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! Nusantara Calendar Core
//! 
//! This crate provides the core traits, error types, and utilities
//! for the nusantara-calendar workspace.

extern crate alloc;

// Re-export commonly used items
pub use thiserror::Error;

/// Core error types for calendar operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalendarError {
    /// Date is out of supported range
    OutOfRange,
    /// Invalid calendar parameters
    InvalidParameters,
    /// Feature not yet implemented
    NotImplemented,
    /// Arithmetic error
    ArithmeticError,
}

impl core::fmt::Display for CalendarError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CalendarError::OutOfRange => write!(f, "Date is out of supported range"),
            CalendarError::InvalidParameters => write!(f, "Invalid calendar parameters"),
            CalendarError::NotImplemented => write!(f, "Feature not yet implemented"),
            CalendarError::ArithmeticError => write!(f, "Arithmetic error"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CalendarError {}

/// Julian Day Number type alias
pub type JDN = i64;
