use core::ops::{Div, DivAssign, Mul, MulAssign};

use rinia::FloatScalar;
#[cfg(feature = "zerocopy")]
use zerocopy::*;

use crate::{
    Duration, HasDuration,
    macros::{impl_approx_forwarding, impl_bytemuck_transparent, impl_min_max_forwarding},
};

/// Represents a temporal frequency in frames per second.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "zerocopy", derive(FromBytes, Immutable, IntoBytes, KnownLayout))]
#[repr(transparent)]
pub struct FrameRate<T: FloatScalar>(T);

impl<T: FloatScalar> FrameRate<T> {
    /// Creates a [FrameRate] from frames per second without validation.
    ///
    /// This function allows invalid values (NaN, infinite, or non-positive) to be used,
    /// which may lead to undefined behavior in subsequent calculations.
    ///
    /// Prefer [Self::from_fps] for validated construction.
    #[inline]
    pub const fn from_fps_unchecked(fps: T) -> Self {
        Self(fps)
    }

    /// Creates a [FrameRate] from frames per second.
    ///
    /// # Panics
    /// Panics if fps is NaN, infinite, or non-positive.
    #[inline]
    pub fn from_fps(fps: T) -> Self {
        assert!(fps.is_finite(), "FrameRate must be finite");
        assert!(fps > T::ZERO, "FrameRate must be positive");
        Self(fps)
    }

    /// Creates a [FrameRate] from seconds per frame without validation.
    ///
    /// This function allows invalid values (NaN, infinite, or non-positive) to be used, which may
    /// lead to undefined behavior in subsequent calculations.
    ///
    /// Prefer [Self::from_seconds_per_frame] for validated construction.
    #[inline]
    pub fn from_seconds_per_frame_unchecked(spf: T) -> Self {
        Self::from_fps_unchecked(T::ONE / spf)
    }

    /// Creates a [FrameRate] from seconds per frame.
    ///
    /// # Panics
    /// Panics if seconds per frame is NaN, infinite, or non-positive.
    #[inline]
    pub fn from_seconds_per_frame(spf: T) -> Self {
        assert!(spf.is_finite(), "seconds_per_frame must be finite");
        assert!(spf > T::ZERO, "seconds_per_frame must be positive");

        Self::from_fps(T::ONE / spf)
    }

    /// Creates a [FrameRate] from [Duration] without validation.
    ///
    /// This function allows invalid values (NaN, infinite, or non-positive) to be used, which may
    /// lead to undefined behavior in subsequent calculations.
    ///
    /// Prefer [Self::from_duration] for validated construction.
    #[inline]
    pub fn from_duration_unchecked(duration: Duration<T>) -> Self {
        Self::from_seconds_per_frame_unchecked(duration.seconds())
    }

    /// Creates a [FrameRate] from [Duration].
    ///
    /// # Panics
    /// Panics if duration is non-positive.
    #[inline]
    pub fn from_duration(duration: Duration<T>) -> Self {
        assert!(duration > Duration::zero(), "Duration must be positive");
        Self::from_seconds_per_frame(duration.seconds())
    }

    /// Returns frames per second.
    #[inline]
    pub fn fps(&self) -> T {
        self.0
    }

    /// Returns seconds per frame.
    #[inline]
    pub fn seconds_per_frame(&self) -> T {
        T::ONE / self.0
    }

    /// Returns the duration of one frame as [Duration].
    #[inline]
    pub fn duration(&self) -> Duration<T> {
        Duration::from_seconds(self.seconds_per_frame())
    }

    /// Returns the dimensionless ratio of this frame rate to another.
    #[inline]
    pub fn ratio(&self, other: FrameRate<T>) -> T {
        *self / other
    }

    // Min/Max forwarding impls.
    impl_min_max_forwarding!(T, 0);
}

// Raw type conversion
impl<T: FloatScalar> From<T> for FrameRate<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self::from_fps(value)
    }
}

impl From<FrameRate<f32>> for f32 {
    #[inline]
    fn from(frame_rate: FrameRate<f32>) -> Self {
        frame_rate.fps()
    }
}

impl From<FrameRate<f64>> for f64 {
    #[inline]
    fn from(frame_rate: FrameRate<f64>) -> Self {
        frame_rate.fps()
    }
}

// Duration type conversion
impl<T: FloatScalar> From<Duration<T>> for FrameRate<T> {
    #[inline]
    fn from(value: Duration<T>) -> Self {
        Self::from_duration(value)
    }
}

impl<T: FloatScalar> From<FrameRate<T>> for Duration<T> {
    #[inline]
    fn from(value: FrameRate<T>) -> Self {
        value.duration()
    }
}

// Equality forwarding
impl_approx_forwarding!(FrameRate<T>, 0);

// Implement HasDuration for FrameRate
impl<T: FloatScalar> HasDuration<T> for FrameRate<T> {
    #[inline]
    fn duration(&self) -> Duration<T> {
        FrameRate::duration(self)
    }
}

// FrameRate / FrameRate = Scalar (ratio)
impl<T: FloatScalar> Div for FrameRate<T> {
    type Output = T;

    #[inline]
    fn div(self, rhs: Self) -> T {
        self.fps() / rhs.fps()
    }
}

// FrameRate * Scalar = FrameRate
impl<T: FloatScalar> Mul<T> for FrameRate<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self::from_fps(self.fps() * rhs)
    }
}

// FrameRate *= Scalar
impl<T: FloatScalar> MulAssign<T> for FrameRate<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        *self = Self::from_fps(self.fps() * rhs);
    }
}

// FrameRate / Scalar = FrameRate
impl<T: FloatScalar> Div<T> for FrameRate<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self::from_fps(self.fps() / rhs)
    }
}

// FrameRate /= Scalar
impl<T: FloatScalar> DivAssign<T> for FrameRate<T> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        *self = Self::from_fps(self.fps() / rhs);
    }
}

// Common Defaults
macro_rules! impl_common_fps_defaults {
    ($scalar:ty) => {
        impl FrameRate<$scalar> {
            /// A frame rate of 24 frames per second.
            pub const FPS_24: Self = Self::from_fps_unchecked(24.0);

            /// A frame rate of 30 frames per second.
            pub const FPS_30: Self = Self::from_fps_unchecked(30.0);

            /// A frame rate of 60 frames per second.
            pub const FPS_60: Self = Self::from_fps_unchecked(60.0);

            /// A frame rate of 120 frames per second.
            pub const FPS_120: Self = Self::from_fps_unchecked(120.0);

            /// A frame rate of 240 frames per second.
            pub const FPS_240: Self = Self::from_fps_unchecked(240.0);
        }
    };
}

impl_common_fps_defaults!(f32);
impl_common_fps_defaults!(f64);

// Bytemuck impl
impl_bytemuck_transparent!(FrameRate<T>, T);

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;

    #[test]
    fn from_fps_and_getters_work() {
        let r = FrameRate::from_fps(50.0_f64);
        assert_abs_diff_eq!(r.fps(), 50.0, epsilon = 1e-12);
        assert_abs_diff_eq!(r.seconds_per_frame(), 0.02, epsilon = 1e-12);
    }

    #[test]
    fn from_seconds_per_frame_and_duration_roundtrip() {
        let r = FrameRate::from_seconds_per_frame(0.25_f64);
        let d = r.duration();
        let r2 = FrameRate::from_duration(d);

        assert_abs_diff_eq!(r.fps(), 4.0, epsilon = 1e-12);
        assert_abs_diff_eq!(d.seconds(), 0.25, epsilon = 1e-12);
        assert_abs_diff_eq!(r2.fps(), 4.0, epsilon = 1e-12);
    }

    #[test]
    #[should_panic(expected = "FrameRate must be positive")]
    fn from_fps_panics_on_non_positive() {
        let _ = FrameRate::from_fps(0.0_f64);
    }

    #[test]
    #[should_panic(expected = "seconds_per_frame must be positive")]
    fn from_seconds_per_frame_panics_on_non_positive() {
        let _ = FrameRate::from_seconds_per_frame(-1.0_f64);
    }

    #[test]
    #[should_panic(expected = "Duration must be positive")]
    fn from_duration_panics_on_non_positive_duration() {
        let _ = FrameRate::<f64>::from_duration(Duration::zero());
    }

    #[test]
    fn ratio_and_arithmetic_ops_work() {
        let mut r = FrameRate::from_fps(30.0_f64);
        let base = FrameRate::from_fps(10.0_f64);

        assert_abs_diff_eq!(r.ratio(base), 3.0, epsilon = 1e-12);
        assert_abs_diff_eq!((r * 2.0).fps(), 60.0, epsilon = 1e-12);
        assert_abs_diff_eq!((r / 3.0).fps(), 10.0, epsilon = 1e-12);

        r *= 2.0;
        assert_abs_diff_eq!(r.fps(), 60.0, epsilon = 1e-12);

        r /= 4.0;
        assert_abs_diff_eq!(r.fps(), 15.0, epsilon = 1e-12);
    }

    #[test]
    fn min_max_choose_expected_values() {
        let a = FrameRate::from_fps(24.0_f64);
        let b = FrameRate::from_fps(60.0_f64);

        assert_eq!(a.min(b), a);
        assert_eq!(a.max(b), b);
    }

    #[test]
    fn from_into_conversions_work() {
        let r: FrameRate<f64> = 48.0_f64.into();
        let raw: f64 = r.into();
        assert_abs_diff_eq!(raw, 48.0, epsilon = 1e-12);

        let d: Duration<f64> = r.into();
        assert_abs_diff_eq!(d.seconds(), 1.0 / 48.0, epsilon = 1e-12);

        let r2: FrameRate<f64> = d.into();
        assert_abs_diff_eq!(r2.fps(), 48.0, epsilon = 1e-12);
    }

    #[test]
    fn has_duration_and_common_defaults_are_valid() {
        let d = HasDuration::duration(&FrameRate::<f64>::FPS_24);
        assert_abs_diff_eq!(d.seconds(), 1.0 / 24.0, epsilon = 1e-12);

        assert_abs_diff_eq!(FrameRate::<f64>::FPS_30.fps(), 30.0, epsilon = 1e-12);
        assert_abs_diff_eq!(FrameRate::<f64>::FPS_60.fps(), 60.0, epsilon = 1e-12);
        assert_abs_diff_eq!(FrameRate::<f64>::FPS_120.fps(), 120.0, epsilon = 1e-12);
        assert_abs_diff_eq!(FrameRate::<f64>::FPS_240.fps(), 240.0, epsilon = 1e-12);
    }

    #[cfg(feature = "bytemuck")]
    #[test]
    fn bytemuck_traits_are_implemented() {
        fn assert_impl<
            T: bytemuck::Pod + bytemuck::Zeroable + bytemuck::TransparentWrapper<f32>,
        >() {
        }

        assert_impl::<FrameRate<f32>>();
    }

    #[cfg(feature = "bytemuck")]
    #[test]
    fn bytemuck_roundtrip_works() {
        let value = FrameRate::from_fps(48.0_f32);
        let bytes = bytemuck::bytes_of(&value);
        let decoded = *bytemuck::from_bytes::<FrameRate<f32>>(bytes);

        assert_eq!(decoded, value);
    }

    #[cfg(feature = "zerocopy")]
    #[test]
    fn zerocopy_traits_are_implemented() {
        fn assert_impl<
            T: zerocopy::FromBytes + zerocopy::IntoBytes + zerocopy::KnownLayout + zerocopy::Immutable,
        >() {
        }

        assert_impl::<FrameRate<f32>>();
    }

    #[cfg(feature = "zerocopy")]
    #[test]
    fn zerocopy_roundtrip_works() {
        let value = FrameRate::from_fps(120.0_f32);
        let bytes = <FrameRate<f32> as zerocopy::IntoBytes>::as_bytes(&value);
        let decoded = <FrameRate<f32> as zerocopy::FromBytes>::ref_from_bytes(bytes)
            .expect("FrameRate bytes should decode");

        assert_eq!(*decoded, value);
    }
}
