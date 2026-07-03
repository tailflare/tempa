use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{
    Duration, FloatScalar, FrameIndex, FrameRate, conversion,
    macros::{impl_approx_forwarding, impl_min_max_forwarding, impl_zero_forwarding},
};

/// Represents a continuous point in time measured in seconds, used as the primary unit for
/// temporal calculations.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Time<T: FloatScalar>(T);

impl<T: FloatScalar> Time<T> {
    /// Creates a new `Time` instance from the given number of seconds without
    /// performing any checks.
    ///
    /// This function allows invalid values (NaN or infinite) to be used,
    /// which may lead to undefined behavior in subsequent calculations.
    ///
    /// Prefer `from_seconds` for safe construction, as it ensures that the provided value is valid.
    #[inline]
    pub const fn from_seconds_unchecked(seconds: T) -> Self {
        Self(seconds)
    }

    /// Creates a new `Time` instance from the given number of seconds.
    ///
    /// # Panics
    /// This function will panic if the provided `seconds` value is NaN or infinite.
    #[inline]
    pub fn from_seconds(seconds: T) -> Self {
        assert!(seconds.is_finite(), "Time cannot be created from NaN or infinite values");
        Self(seconds)
    }

    /// Returns the raw value of this `Time` instance in seconds.
    #[inline]
    pub fn seconds(&self) -> T {
        self.0
    }

    /// Converts this timestamp to a `FrameIndex` at the given frame rate.
    #[inline]
    pub fn to_frame(self, rate: FrameRate<T>) -> FrameIndex {
        conversion::frame_from_time(self, rate)
    }

    /// Creates a new `Time` instance from the given `FrameIndex` at the specified frame rate.
    #[inline]
    pub fn from_frame(frame: FrameIndex, rate: FrameRate<T>) -> Self {
        conversion::time_from_frame(frame, rate)
    }

    /// Normalizes this timestamp between a `start` and `end` time boundary.
    /// Returns a clamped floating-point factor in the range `[0.0, 1.0]`.
    ///
    /// When the range is degenerate (start == end), the function returns `0.0` to avoid division
    /// by zero.
    #[inline]
    pub fn normalize_between(&self, start: Self, end: Self) -> T {
        let total = end - start;
        if total.seconds() > T::zero() { (*self - start) / total } else { T::zero() }
    }

    // Min/Max forwarding impls.
    impl_min_max_forwarding!(T, 0);

    // Zero forwarding impls.
    impl_zero_forwarding!(T, 0);
}

// Default
impl<T: FloatScalar> Default for Time<T> {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

// Raw type conversion
impl<T: FloatScalar> From<T> for Time<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self::from_seconds(value)
    }
}

impl From<Time<f32>> for f32 {
    #[inline]
    fn from(time: Time<f32>) -> Self {
        time.seconds()
    }
}

impl From<Time<f64>> for f64 {
    #[inline]
    fn from(time: Time<f64>) -> Self {
        time.seconds()
    }
}

// Equality forwarding
impl_approx_forwarding!(Time<T>, 0);

// Time - Time = Duration
impl<T: FloatScalar> Sub for Time<T> {
    type Output = Duration<T>;

    #[inline]
    fn sub(self, rhs: Self) -> Duration<T> {
        Duration::from_seconds(self.seconds() - rhs.seconds())
    }
}

// Time - ScalarFloat = Time
impl<T: FloatScalar> Sub<T> for Time<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: T) -> Self {
        Self::from_seconds(self.seconds() - rhs)
    }
}

// Time + Duration = Time
impl<T: FloatScalar> Add<Duration<T>> for Time<T> {
    type Output = Time<T>;

    #[inline]
    fn add(self, rhs: Duration<T>) -> Time<T> {
        Self::from_seconds(self.seconds() + rhs.seconds())
    }
}

// Time+= Duration
impl<T: FloatScalar> AddAssign<Duration<T>> for Time<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Duration<T>) {
        *self = *self + rhs;
    }
}

// Time - Duration = Time
impl<T: FloatScalar> Sub<Duration<T>> for Time<T> {
    type Output = Time<T>;

    #[inline]
    fn sub(self, rhs: Duration<T>) -> Time<T> {
        Self::from_seconds(self.seconds() - rhs.seconds())
    }
}

// Time -= Duration
impl<T: FloatScalar> SubAssign<Duration<T>> for Time<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Duration<T>) {
        *self = *self - rhs;
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;

    #[test]
    fn from_seconds_and_seconds_roundtrip() {
        let t = Time::from_seconds(1.25_f64);
        assert_eq!(t.seconds(), 1.25);
    }

    #[test]
    #[should_panic(expected = "Time cannot be created from NaN or infinite values")]
    fn from_seconds_panics_on_non_finite() {
        let _ = Time::from_seconds(f64::INFINITY);
    }

    #[test]
    fn frame_conversion_roundtrip() {
        let rate = FrameRate::from_fps(24.0_f64);
        let t = Time::from_seconds(2.5);

        let frame = t.to_frame(rate);
        let t2 = Time::from_frame(frame, rate);

        assert_eq!(frame, FrameIndex::new(60));
        assert_eq!(t2, Time::from_seconds(2.5));
    }

    #[test]
    fn normalize_between_handles_normal_and_degenerate_ranges() {
        let t = Time::from_seconds(5.0_f64);

        assert_abs_diff_eq!(
            t.normalize_between(Time::from_seconds(0.0), Time::from_seconds(10.0)),
            0.5,
            epsilon = 1e-12
        );
        assert_abs_diff_eq!(
            t.normalize_between(Time::from_seconds(10.0), Time::from_seconds(0.0)),
            0.0,
            epsilon = 1e-12
        );
        assert_abs_diff_eq!(
            t.normalize_between(Time::from_seconds(4.0), Time::from_seconds(4.0)),
            0.0,
            epsilon = 1e-12
        );
    }

    #[test]
    fn min_max_zero_and_default_work() {
        let a = Time::from_seconds(1.0_f64);
        let b = Time::from_seconds(3.0_f64);

        assert_eq!(a.min(b), a);
        assert_eq!(a.max(b), b);

        let z = Time::<f64>::zero();
        assert_eq!(Time::<f64>::default(), z);
        assert!(z.is_zero());
    }

    #[test]
    fn raw_conversions_work() {
        let t: Time<f64> = 2.0_f64.into();
        let raw: f64 = t.into();

        assert_eq!(raw, 2.0);
    }

    #[test]
    fn time_duration_arithmetic_and_assign_work() {
        let mut t = Time::from_seconds(10.0_f64);
        let d = Duration::from_seconds(2.5_f64);

        assert_eq!(t - Time::from_seconds(4.0), Duration::from_seconds(6.0));
        assert_eq!(t + d, Time::from_seconds(12.5));
        assert_eq!(t - d, Time::from_seconds(7.5));

        t += d;
        assert_eq!(t, Time::from_seconds(12.5));

        t -= d;
        assert_eq!(t, Time::from_seconds(10.0));
    }
}
