use crate::Duration;

/// Trait for values that expose a [Duration].
pub trait HasDuration<T> {
    /// Returns this value's [Duration].
    fn duration(&self) -> Duration<T>;
}
