//! # Nusantara Calendar
//! 
//! A comprehensive collection of Indonesian/Nusantara calendar systems with unified
//! `calendar-core` trait integration. This crate provides access to multiple traditional
//! calendar systems used throughout the Indonesian archipelago.
//! 
//! ## Features
//! 
//! This crate uses feature flags to minimize compilation time and binary size:
//! 
//! - `balinese` - Balinese Saka calendar (wraps official `balinese-calendar` crate)
//! - `jawa` - Javanese calendar system
//! - `hijriyah` - Islamic calendar
//! - `batak` - Batak calendar
//! - `sunda` - Sundanese calendar
//! - `tengger` - Tenggerese calendar
//! - `bugis` - Buginese calendar
//! - `sasak` - Sasak calendar
//! - `dayak` - Dayak calendar
//! - `toraja` - Torajan calendar
//! - `minangkabau` - Minangkabau calendar
//! - `chinese-nusantara` - Chinese calendar adapted for Indonesian context
//! - `dewasa-engine` - Auspiciousness calculation engine
//! 
//! ## Usage
//! 
//! Add this to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! nusantara-calendar = { version = "0.1", features = ["balinese", "jawa"] }
//! ```
//! 
//! ## Example
//! 
//! ```rust
//! use nusantara_calendar::balinese::BalineseDate;
//! use calendar_core::CalendarDate;
//! 
//! // Create a Balinese date
//! let date = BalineseDate::from_ymd(2026, 3, 19).unwrap();
//! println!("Saka year: {}", date.saka_year);
//! ```
//! 
//! ## License
//! 
//! Licensed under MIT or Apache-2.0.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

extern crate alloc;

// Re-export calendar-core traits
pub use calendar_core::{CalendarDate, CalendarMetadata, HasAuspiciousness, JDN, CalendarError};

// Calendar system modules (feature-gated)
#[cfg(feature = "balinese")]
pub mod balinese;

#[cfg(feature = "jawa")]
pub mod jawa;

#[cfg(feature = "hijriyah")]
pub mod hijriyah;

#[cfg(feature = "batak")]
pub mod batak;

#[cfg(feature = "sunda")]
pub mod sunda;

#[cfg(feature = "tengger")]
pub mod tengger;

#[cfg(feature = "bugis")]
pub mod bugis;

#[cfg(feature = "sasak")]
pub mod sasak;

#[cfg(feature = "dayak")]
pub mod dayak;

#[cfg(feature = "toraja")]
pub mod toraja;

#[cfg(feature = "minangkabau")]
pub mod minangkabau;

#[cfg(feature = "chinese-nusantara")]
pub mod chinese_nusantara;

#[cfg(feature = "dewasa-engine")]
pub mod dewasa_engine;

// Common error types
#[cfg(feature = "std")]
/// Box error type for dynamic error handling
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[cfg(not(feature = "std"))]
pub type BoxError = &'static str;
