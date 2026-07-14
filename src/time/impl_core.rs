use core::ops::{Div, Mul};

use rinia::{
    Scalard, Scalarf,
    numeric::{Floor, IsFinite, LossyCast, SaturatingCast, Zero},
};

use crate::{Error, FrameIndex, FrameRate, Time, common};

impl<T> Time<T> {
    /// Creates [Time] from seconds without validation.
    ///
    /// This function allows invalid values to be used, which may lead to invalid results in
    /// subsequent calculations.
    ///
    /// Prefer [Self::from_seconds] for validated construction.
    #[inline]
    pub const fn new(seconds: T) -> Self {
        Self(seconds)
    }

    /// Creates [Time] from seconds with validation.
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
            Err(Error::InvalidValue("Time cannot be created from NaN or infinite values"))
        }
    }

    /// Creates [Time] from seconds with validation.
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

    /// Returns this [Time] value in seconds.
    #[inline]
    pub const fn seconds(&self) -> T
    where
        T: Copy,
    {
        self.0
    }

    /// Consumes this [Time] value and returns the inner seconds value.
    #[inline]
    pub fn into_seconds(self) -> T {
        self.0
    }

    /// Converts this time value to [FrameIndex] using the given [FrameRate].
    #[inline]
    pub fn to_frame<I>(self, rate: FrameRate<T>) -> FrameIndex<I>
    where
        T: Copy + Mul<Output = T> + Floor + SaturatingCast<I>,
    {
        common::frame_from_time(self, rate)
    }

    /// Creates [Time] from [FrameIndex] at the specified [FrameRate].
    #[inline]
    pub fn from_frame<I>(frame: FrameIndex<I>, rate: FrameRate<T>) -> Self
    where
        T: Copy + Div<Output = T>,
        I: Copy + LossyCast<T>,
    {
        common::time_from_frame(frame, rate)
    }
}

// Impl Default for Time<T>
impl<T> Default for Time<T>
where
    Self: Zero,
{
    fn default() -> Self {
        Self::ZERO
    }
}

// Conversion
impl<T> From<T> for Time<T> {
    fn from(seconds: T) -> Self {
        Self(seconds)
    }
}

impl From<Time<Scalarf>> for Scalarf {
    fn from(time: Time<Scalarf>) -> Self {
        time.0
    }
}

impl From<Time<Scalard>> for Scalard {
    fn from(time: Time<Scalard>) -> Self {
        time.0
    }
}

// Impl approx equality traits for Time<T>
common::impl_approx_forwarding!(Time<T>, 0);

// Impl bytemuck for Time<T>
common::impl_bytemuck_basic!(
    [T],
    Time<T>,
    item: T,
);
