use crate::{Duration, TimeRange};

/// Trait for values that expose a [TimeRange].
pub trait HasTimeRange<T> {
    /// Returns this value's [TimeRange], if applicable.
    fn time_range(&self) -> Option<TimeRange<T>>;

    /// Returns this value's [Duration], if applicable.
    fn duration(&self) -> Option<Duration<T>>;
}
