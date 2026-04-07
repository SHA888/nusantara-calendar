#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! # Balinese Saka Calendar (Kalender Bali)
//!
//! This crate provides a comprehensive implementation of the Balinese Saka Calendar
//! system with full integration to the `calendar-core` traits. It computes Pawukon
//! (210-day cycle), Wewaran (multi-cycle day names), Sasih (lunar months), Saka
//! year, Rahinan (ceremony dates), and traditional Wariga classification systems.
//!
//! ## Features
//!
//! - **Pawukon**: 30 Wuku × 210-day cycle
//! - **Wewaran**: 10 concurrent week cycles (Eka through Dasa Wara)
//! - **Sasih**: Lunar months with Penanggal/Pangelong/Purnama/Tilem
//! - **Saka Year**: Traditional Balinese calendar years
//! - **Rahinan**: Holy day detection (Galungan, Kuningan, Saraswati, etc.)
//! - **Wariga**: Traditional auspiciousness calculations
//! - **Calendar Integration**: Implements `calendar-core` traits for interoperability
//!
//! ## Quick Start
//!
//! ```rust
//! use balinese::BalineseDate;
//!
//! // Create from Gregorian date
//! let today = BalineseDate::from_ymd(2026, 3, 22).unwrap();
//! println!("{}", today.to_balinese_string());
//! // Output: Redite Pon Dukut, Sasih Kadasa, Saka 1948
//!
//! // Use calendar-core traits
//! use calendar_core::{CalendarDate, CalendarMetadata};
//!
//! let date = BalineseDate::from_gregorian(2026, 3, 22).unwrap();
//! println!("Calendar: {}", BalineseDate::calendar_name());
//! println!("Saka Year: {}", date.saka_year);
//! ```
//!
//! ## Data Sources
//!
//! This implementation is based on:
//! - I Made Bidja Alm. / I Md Agus Putra Wijaya, Kalender Bali 2026
//! - 50+ lontar Wariga manuscripts and 13 Kawi/Sanskrit/Balinese dictionaries
//! - Cross-validated against kalenderbali.org and BASAbali Wiki
//! - Astronomical calculations for sunrise and lunar phases
//!
//! ## Supported Date Range
//!
//! - **Gregorian Years**: 1800-2200 CE
//! - **Saka Years**: 1722-2322 Saka
//! - **JDN Range**: 2,378,496 - 2,540,587
//!
//! Out-of-range dates will return `CalendarError::OutOfRange`.

extern crate alloc;

// Re-export calendar-core types
pub use calendar_core::{CalendarDate, CalendarError, CalendarMetadata, HasAuspiciousness, JDN};

mod balinese_date;
mod error;
mod pawukon;
mod sasih;
mod wewaran;
mod wariga;

// Re-export main types
pub use balinese_date::BalineseDate;
pub use error::BalineseCalendarError;
pub use pawukon::Wuku;
pub use sasih::Sasih;
pub use wewaran::{Pancawara, Saptawara};
pub use wariga::{Activity, AuspiciousnessLevel};
