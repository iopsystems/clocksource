//! Precise times and durations using 64bit primitives and whole nanoseconds.
//!
//! Unlike `std::time`, these types always have a fixed size representation and
//! also includes atomic types.

mod atomic_duration;
mod atomic_instant;
mod atomic_unix_instant;
mod duration;
mod instant;
mod unix_instant;

pub use atomic_duration::AtomicDuration;
pub use atomic_duration::TryFromError as AtomicDurationTryFromError;
pub use atomic_instant::AtomicInstant;
pub use atomic_unix_instant::AtomicUnixInstant;
pub use atomic_unix_instant::TryFromError as AtomicUnixInstantTryFromError;
pub use duration::Duration;
pub use duration::TryFromError as DurationTryFromError;
pub use instant::Instant;
pub use unix_instant::TryFromError as UnixInstantTryFromError;
pub use unix_instant::UnixInstant;
