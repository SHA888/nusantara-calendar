//! Error types for the Balinese calendar implementation

use alloc::string::String;
use calendar_core::CalendarError;

/// Errors specific to the Balinese calendar implementation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BalineseCalendarError {
    /// Date is outside supported range
    OutOfRange(String),
    /// Invalid Gregorian date parameters
    InvalidGregorianDate(String),
    /// Invalid Saka year parameters
    InvalidSakaYear(String),
    /// Error in astronomical calculations
    AstronomicalError(String),
    /// Feature not yet implemented
    NotImplemented(String),
}

impl core::fmt::Display for BalineseCalendarError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OutOfRange(msg) => write!(f, "Date out of supported range: {msg}"),
            Self::InvalidGregorianDate(msg) => write!(f, "Invalid Gregorian date: {msg}"),
            Self::InvalidSakaYear(msg) => write!(f, "Invalid Saka year: {msg}"),
            Self::AstronomicalError(msg) => write!(f, "Astronomical calculation error: {msg}"),
            Self::NotImplemented(msg) => write!(f, "Feature not yet implemented: {msg}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BalineseCalendarError {}

impl From<BalineseCalendarError> for CalendarError {
    fn from(err: BalineseCalendarError) -> Self {
        match err {
            BalineseCalendarError::OutOfRange(msg) => CalendarError::OutOfRange(msg),
            BalineseCalendarError::InvalidGregorianDate(msg) => CalendarError::InvalidParameters(msg),
            BalineseCalendarError::InvalidSakaYear(msg) => CalendarError::InvalidParameters(msg),
            BalineseCalendarError::AstronomicalError(msg) => CalendarError::ArithmeticError(msg),
            BalineseCalendarError::NotImplemented(msg) => CalendarError::NotImplemented(msg),
        }
    }
}
