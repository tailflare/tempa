use rinia::{
    Scalard, Scalarf,
    numeric::{IsFinite, Zero},
};

use crate::{Duration, Error, HasDuration, common};

impl<T> Duration<T> {
    /// Creates [Duration] from seconds without validation.
    ///
    /// This function allows invalid values to be used, which may lead to invalid results in
    /// subsequent calculations.
    ///
    /// Prefer [Self::from_seconds] for validated construction.
    #[inline]
    pub const fn new(seconds: T) -> Self {
        Self(seconds)
    }

    /// Creates [Duration] from seconds with validation.
    ///
    /// Returns an error if seconds is NaN or infinite.
    #[inline]
    pub fn try_from_seconds(seconds: T) -> Result<Self, Error>
    where
        T: Copy + IsFinite,
    {
        if seconds.is_finite() {
            Ok(Self(seconds))
        } else {
            Err(Error::InvalidValue("Duration cannot be created from NaN or infinite values"))
        }
    }

    /// Creates a [Duration] from seconds with validation.
    ///
    /// # Panics
    /// Panics if seconds is NaN or infinite.
    #[inline]
    pub fn from_seconds(seconds: T) -> Self
    where
        T: Copy + IsFinite,
    {
        Self::try_from_seconds(seconds).unwrap_or_else(|err| panic!("{err}"))
    }

    /// Returns this [Duration] value in seconds.
    #[inline]
    pub const fn seconds(&self) -> T
    where
        T: Copy,
    {
        self.0
    }

    /// Consumes this [Duration] value and returns the inner seconds value.
    #[inline]
    pub fn into_seconds(self) -> T {
        self.0
    }
}

// Impl Default for Duration<T>
impl<T> Default for Duration<T>
where
    Self: Zero,
{
    fn default() -> Self {
        Self::ZERO
    }
}

// Conversion
impl<T> From<T> for Duration<T> {
    fn from(seconds: T) -> Self {
        Self(seconds)
    }
}

impl From<Duration<Scalarf>> for Scalarf {
    fn from(duration: Duration<Scalarf>) -> Self {
        duration.0
    }
}

impl From<Duration<Scalard>> for Scalard {
    fn from(duration: Duration<Scalard>) -> Self {
        duration.0
    }
}

// Impl HasDuration for Duration itself
impl<T> HasDuration<T> for Duration<T>
where
    T: Copy,
{
    fn duration(&self) -> Duration<T> {
        *self
    }
}

// Impl approx equality traits for Duration<T>
common::impl_approx_forwarding!(Duration<T>, 0);

// Impl bytemuck for Duration<T>
common::impl_bytemuck_basic!(
    [T],
    Duration<T>,
    item: T,
);
