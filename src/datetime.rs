//! Human readable datetimes.

use core::fmt::Display;
use time::OffsetDateTime;

/// A date and time representation, convertible from Unix timestamps.
///
/// When constructed from a `precise::UnixInstant`, sub-millisecond precision
/// is truncated in the display format.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime {
    dt: OffsetDateTime,
}

impl Display for DateTime {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        let date = self.dt.date();
        let time = self.dt.time();
        write!(
            f,
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}+00:00",
            date.year(),
            date.month() as u8,
            date.day(),
            time.hour(),
            time.minute(),
            time.second(),
            time.millisecond(),
        )
    }
}

impl From<crate::precise::UnixInstant> for DateTime {
    /// # Panics
    ///
    /// Panics if the timestamp is outside the representable range of the
    /// `time` crate (approximately years -9999 to 9999). All valid
    /// `precise::UnixInstant` values (up to year ~2554) are within range.
    fn from(other: crate::precise::UnixInstant) -> Self {
        DateTime {
            dt: OffsetDateTime::from_unix_timestamp_nanos(other.ns as i128).unwrap(),
        }
    }
}

impl From<crate::coarse::UnixInstant> for DateTime {
    /// # Panics
    ///
    /// Panics if the timestamp is outside the representable range of the
    /// `time` crate. All valid `coarse::UnixInstant` values (up to year ~2106)
    /// are within range.
    fn from(other: crate::coarse::UnixInstant) -> Self {
        DateTime {
            dt: OffsetDateTime::from_unix_timestamp(other.secs as i64).unwrap(),
        }
    }
}
