use core::ops::Div;

use rinia::{
    Scalard, Scalarf,
    numeric::{IsFinite, LossyCastFrom, One, Zero},
};

use crate::{Duration, Error, FrameRate, common};

impl<T> FrameRate<T> {
    /// Creates a [FrameRate] from frames per second without validation.
    ///
    /// This function allows invalid values (not finite, or non-positive) to be used,
    /// which may lead to invalid results in subsequent calculations.
    ///
    /// Prefer [Self::from_fps] for validated construction.
    #[inline]
    pub const fn new(fps: T) -> Self {
        Self(fps)
    }

    /// Creates [FrameRate] from frames per second with validation.
    ///
    /// Returns an error if fps is not finite, or non-positive.
    #[inline]
    pub fn try_from_fps(fps: T) -> Result<Self, Error>
    where
        T: Copy + IsFinite + PartialOrd + Zero,
    {
        if !fps.is_finite() {
            return Err(Error::InvalidValue(
                "FrameRate cannot be created from NaN or infinite values",
            ));
        }

        if fps <= T::ZERO {
            return Err(Error::InvalidValue(
                "FrameRate cannot be created from non-positive values",
            ));
        }

        Ok(Self(fps))
    }

    /// Creates a [FrameRate] from frames per second with validation.
    ///
    /// # Panics
    /// Panics if fps is not finite, or non-positive.
    #[inline]
    pub fn from_fps(fps: T) -> Self
    where
        T: Copy + IsFinite + PartialOrd + Zero,
    {
        Self::try_from_fps(fps).unwrap_or_else(|err| panic!("{err}"))
    }

    /// Returns frames per second.
    #[inline]
    pub fn fps(&self) -> T
    where
        T: Copy,
    {
        self.0
    }

    /// Consumes this [FrameRate] value and returns the inner frames per second value.
    #[inline]
    pub fn into_fps(self) -> T {
        self.0
    }

    /// Returns seconds per frame.
    #[inline]
    pub fn seconds_per_frame(&self) -> T
    where
        T: Copy + One + Div<Output = T>,
    {
        T::ONE / self.0
    }

    /// Consumes this [FrameRate] value and returns the seconds per frame value.
    #[inline]
    pub fn into_seconds_per_frame(self) -> T
    where
        T: One + Div<Output = T>,
    {
        T::ONE / self.0
    }

    /// Returns the duration of one frame as [Duration].
    #[inline]
    pub fn duration(&self) -> Duration<T>
    where
        T: Copy + One + Div<Output = T>,
    {
        Duration::new(self.seconds_per_frame())
    }

    /// Consumes this [FrameRate] value and returns the duration of one frame as [Duration].
    #[inline]
    pub fn into_duration(self) -> Duration<T>
    where
        T: One + Div<Output = T>,
    {
        Duration::new(self.into_seconds_per_frame())
    }

    /// Returns the dimensionless ratio of this frame rate to another.
    #[inline]
    pub fn ratio(&self, other: FrameRate<T>) -> T
    where
        T: Copy + Div<Output = T>,
    {
        *self / other
    }
}

// Default
impl<T> Default for FrameRate<T>
where
    T: LossyCastFrom<u16>,
{
    #[inline]
    fn default() -> Self {
        Self::fps_60()
    }
}

// Impl common frame rate constructors
crate::frame_rate::impl_rate_fps!(24, 25, 30, 48, 50, 60, 120, 144, 240);

// Impl approx equality traits for FrameRate<T>
rinia::impl_approx_eq_wrapper!([T], impl: FrameRate<T>, item: T);

// Impl bytemuck for FrameRate<T>
common::impl_bytemuck_basic!(
    [T],
    FrameRate<T>,
    item: T,
);

// Conversion
impl<T> From<T> for FrameRate<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl From<FrameRate<Scalarf>> for Scalarf {
    #[inline]
    fn from(frame_rate: FrameRate<Scalarf>) -> Self {
        frame_rate.fps()
    }
}

impl From<FrameRate<Scalard>> for Scalard {
    #[inline]
    fn from(frame_rate: FrameRate<Scalard>) -> Self {
        frame_rate.fps()
    }
}
