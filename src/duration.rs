use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

use crate::{
    FloatScalar, FrameRate,
    macros::{impl_approx_forwarding, impl_min_max_forwarding, impl_zero_forwarding},
};

/// Represents a continuous span of time in seconds, describing the length between two time points.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Duration<T: FloatScalar>(T);

/// Trait for entities that have a duration, allowing for retrieval of the duration value.
pub trait HasDuration<T: FloatScalar> {
    /// Returns the duration of this entity.
    fn duration(&self) -> Duration<T>;
}

impl<T: FloatScalar> Duration<T> {
    /// Creates a new `Duration` instance from the given number of seconds without
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

    /// Creates a new `Duration` instance from the given number of seconds.
    ///
    /// # Panics
    /// This function will panic if the provided `seconds` value is NaN or infinite.
    #[inline]
    pub fn from_seconds(seconds: T) -> Self {
        assert!(seconds.is_finite(), "Duration cannot be created from NaN or infinite values");
        Self(seconds)
    }

    /// Returns the reciprocal of this duration as a frame rate, which is the number of frames per
    /// second.
    #[inline]
    pub fn frame_rate(&self) -> FrameRate<T> {
        FrameRate::from_duration(*self)
    }

    /// Returns the raw value of this `Duration` instance in seconds.
    #[inline]
    pub fn seconds(&self) -> T {
        self.0
    }

    /// Returns the ratio of this `Duration` instance to another `Duration` instance.
    ///
    /// This is equivalent to dividing the two durations, yielding a dimensionless scalar value.
    #[inline]
    pub fn ratio(&self, other: Duration<T>) -> T {
        *self / other
    }

    // Min/Max forwarding impls.
    impl_min_max_forwarding!(T, 0);

    // Zero forwarding impls.
    impl_zero_forwarding!(T, 0);
}

// Default
impl<T: FloatScalar> Default for Duration<T> {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

// Conversion
impl<T: FloatScalar> From<T> for Duration<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self::from_seconds(value)
    }
}

impl From<Duration<f32>> for f32 {
    #[inline]
    fn from(duration: Duration<f32>) -> Self {
        duration.seconds()
    }
}

impl From<Duration<f64>> for f64 {
    #[inline]
    fn from(duration: Duration<f64>) -> Self {
        duration.seconds()
    }
}

// Equality forwarding
impl_approx_forwarding!(Duration<T>, 0);

// Duration + Duration = Duration
impl<T: FloatScalar> Add for Duration<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_seconds(self.seconds() + rhs.seconds())
    }
}

// Duration += Duration
impl<T: FloatScalar> AddAssign for Duration<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

/// Duration + Scalar = Duration
impl<T: FloatScalar> Add<T> for Duration<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: T) -> Self {
        Self::from_seconds(self.seconds() + rhs)
    }
}

/// Duration += Scalar
impl<T: FloatScalar> AddAssign<T> for Duration<T> {
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

// Duration - Duration = Duration
impl<T: FloatScalar> Sub for Duration<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_seconds(self.seconds() - rhs.seconds())
    }
}

// Duration -= Duration
impl<T: FloatScalar> SubAssign for Duration<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

/// Duration - Scalar = Duration
impl<T: FloatScalar> Sub<T> for Duration<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: T) -> Self {
        Self::from_seconds(self.seconds() - rhs)
    }
}

/// Duration -= Scalar
impl<T: FloatScalar> SubAssign<T> for Duration<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

// Duration / Duration = Scalar (ratio)
impl<T: FloatScalar> Div for Duration<T> {
    type Output = T;

    #[inline]
    fn div(self, rhs: Self) -> T {
        self.seconds() / rhs.seconds()
    }
}

// Duration / Scalar = Duration
impl<T: FloatScalar> Div<T> for Duration<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self::from_seconds(self.seconds() / rhs)
    }
}

// Duration * Scalar = Duration
impl<T: FloatScalar> Mul<T> for Duration<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self::from_seconds(self.seconds() * rhs)
    }
}

// Duration *= Scalar
impl<T: FloatScalar> MulAssign<T> for Duration<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

// Scalar * Duration = Duration
impl Mul<Duration<f32>> for f32 {
    type Output = Duration<f32>;

    #[inline]
    fn mul(self, rhs: Duration<f32>) -> Self::Output {
        Duration::from_seconds(self * rhs.seconds())
    }
}

impl Mul<Duration<f64>> for f64 {
    type Output = Duration<f64>;

    #[inline]
    fn mul(self, rhs: Duration<f64>) -> Self::Output {
        Duration::from_seconds(self * rhs.seconds())
    }
}

// Implement HasDuration for Duration itself
impl<T: FloatScalar> HasDuration<T> for Duration<T> {
    #[inline]
    fn duration(&self) -> Duration<T> {
        *self
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;

    #[test]
    fn from_seconds_and_seconds_roundtrip() {
        let d = Duration::from_seconds(2.5_f64);
        assert_eq!(d.seconds(), 2.5);
    }

    #[test]
    #[should_panic(expected = "Duration cannot be created from NaN or infinite values")]
    fn from_seconds_panics_on_nan() {
        let _ = Duration::from_seconds(f64::NAN);
    }

    #[test]
    fn frame_rate_roundtrip() {
        let d = Duration::from_seconds(0.5_f64);
        let r = d.frame_rate();
        let d2 = r.duration();

        assert_abs_diff_eq!(r.fps(), 2.0, epsilon = 1e-12);
        assert_abs_diff_eq!(d2.seconds(), 0.5, epsilon = 1e-12);
    }

    #[test]
    fn ratio_computes_dimensionless_value() {
        let a = Duration::from_seconds(6.0_f64);
        let b = Duration::from_seconds(2.0_f64);
        assert_abs_diff_eq!(a.ratio(b), 3.0, epsilon = 1e-12);
    }

    #[test]
    fn arithmetic_with_duration_and_scalar() {
        let a = Duration::from_seconds(3.0_f64);
        let b = Duration::from_seconds(1.5_f64);

        assert_abs_diff_eq!((a + b).seconds(), 4.5, epsilon = 1e-12);
        assert_abs_diff_eq!((a - b).seconds(), 1.5, epsilon = 1e-12);
        assert_abs_diff_eq!((a + 2.0).seconds(), 5.0, epsilon = 1e-12);
        assert_abs_diff_eq!((a - 0.5).seconds(), 2.5, epsilon = 1e-12);
        assert_abs_diff_eq!((a * 2.0).seconds(), 6.0, epsilon = 1e-12);
        assert_abs_diff_eq!((a / 2.0).seconds(), 1.5, epsilon = 1e-12);
        assert_abs_diff_eq!(a / b, 2.0, epsilon = 1e-12);
        assert_abs_diff_eq!((2.0_f64 * b).seconds(), 3.0, epsilon = 1e-12);
    }

    #[test]
    fn assign_ops_work() {
        let mut d = Duration::from_seconds(2.0_f64);
        d += Duration::from_seconds(1.0);
        d -= Duration::from_seconds(0.5);
        d += 1.0;
        d -= 0.5;
        d *= 2.0;

        assert_abs_diff_eq!(d.seconds(), 6.0, epsilon = 1e-12);
    }

    #[test]
    fn default_and_zero_behave_consistently() {
        let d = Duration::<f64>::default();
        assert_eq!(d, Duration::zero());
        assert!(d.is_zero());
    }

    #[test]
    fn min_max_select_expected_values() {
        let a = Duration::from_seconds(1.0_f64);
        let b = Duration::from_seconds(3.0_f64);

        assert_eq!(a.min(b), a);
        assert_eq!(a.max(b), b);
    }

    #[test]
    fn from_into_conversions_work() {
        let d: Duration<f64> = 1.25_f64.into();
        let raw: f64 = d.into();
        assert_abs_diff_eq!(raw, 1.25, epsilon = 1e-12);
    }

    #[test]
    fn has_duration_returns_self() {
        let d = Duration::from_seconds(4.0_f64);
        assert_eq!(HasDuration::duration(&d), d);
    }
}
