use core::ops::{Add, AddAssign, Sub, SubAssign};

use super::Duration;

/// A `coarse::Instant` is a measurement of a monotonically nondecreasing
/// clock. It is opaque and useful only with the duration types.
///
/// Unlike `std::time::Instant` the internal representation use only seconds in
/// a `u32` field to hold the clock reading. This means that they will wrap
/// after ~136 years.
///
/// As with `std::time::Instant`, instants are not guaranteed to be steady. They
/// are taken from a clock which is subject to phase and frequency adjustments.
/// This means that they may jump forward or speed up or slow down. Barring any
/// platform bugs, it is expected that they are always monotonically
/// nondecreasing.
///
/// The size of a `coarse::Instant` is always the same as a `u32`.
#[repr(transparent)]
#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant {
    pub(crate) secs: u32,
}

impl Instant {
    /// Create a new `Instant` from a whole number of seconds.
    pub const fn from_secs(secs: u32) -> Self {
        Self { secs }
    }

    /// Return an `Instant` that represents the current moment.
    pub fn now() -> Self {
        crate::sys::monotonic::coarse()
    }

    /// Return the elapsed time since the original timestamp.
    pub fn elapsed(&self) -> Duration {
        Self::now() - *self
    }

    /// Returns the inner value as whole seconds.
    pub const fn as_secs(&self) -> u32 {
        self.secs
    }

    /// Return the elapsed duration from some earlier timestamp until this
    /// timestamp.
    pub fn duration_since(&self, earlier: Self) -> Duration {
        *self - earlier
    }

    /// Returns the duration since `earlier`, or `None` if `earlier` is after `self`.
    pub fn checked_duration_since(&self, earlier: Self) -> Option<Duration> {
        self.secs
            .checked_sub(earlier.secs)
            .map(|secs| Duration { secs })
    }

    /// Subtracts a duration, returning `None` on underflow.
    pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
        self.secs
            .checked_sub(duration.secs)
            .map(|secs| Self { secs })
    }

    /// Adds a duration, returning `None` on overflow.
    pub fn checked_add(&self, duration: Duration) -> Option<Self> {
        self.secs
            .checked_add(duration.secs)
            .map(|secs| Self { secs })
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;

    fn add(self, rhs: Duration) -> Self::Output {
        Instant {
            secs: self.secs + rhs.secs,
        }
    }
}

impl Add<core::time::Duration> for Instant {
    type Output = Instant;

    fn add(self, rhs: core::time::Duration) -> Self::Output {
        Instant {
            secs: self.secs + rhs.as_secs() as u32,
        }
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    fn sub(self, rhs: Instant) -> Self::Output {
        Duration {
            secs: self.secs - rhs.secs,
        }
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        self.secs += rhs.secs;
    }
}

impl AddAssign<core::time::Duration> for Instant {
    fn add_assign(&mut self, rhs: core::time::Duration) {
        self.secs += rhs.as_secs() as u32;
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    fn sub(self, rhs: Duration) -> Self::Output {
        Instant {
            secs: self.secs - rhs.secs,
        }
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, rhs: Duration) {
        self.secs -= rhs.secs;
    }
}

impl Sub<core::time::Duration> for Instant {
    type Output = Instant;

    fn sub(self, rhs: core::time::Duration) -> Self::Output {
        Instant {
            secs: self.secs - rhs.as_secs() as u32,
        }
    }
}

impl SubAssign<core::time::Duration> for Instant {
    fn sub_assign(&mut self, rhs: core::time::Duration) {
        self.secs -= rhs.as_secs() as u32;
    }
}

#[derive(Debug)]
pub struct TryFromError {
    kind: TryFromErrorKind,
}

#[derive(Debug)]
enum TryFromErrorKind {
    Overflow,
}

impl TryFromError {
    const fn description(&self) -> &'static str {
        match self.kind {
            TryFromErrorKind::Overflow => "can not convert to Instant: value is too big",
        }
    }
}

impl core::fmt::Display for TryFromError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.description().fmt(f)
    }
}

impl std::error::Error for TryFromError {}

impl TryFrom<crate::precise::Instant> for Instant {
    type Error = TryFromError;

    fn try_from(other: crate::precise::Instant) -> Result<Self, Self::Error> {
        let other = other.ns / crate::precise::Duration::SECOND.as_nanos();
        if other > u32::MAX as u64 {
            Err(TryFromError {
                kind: TryFromErrorKind::Overflow,
            })
        } else {
            Ok(Self { secs: other as u32 })
        }
    }
}
