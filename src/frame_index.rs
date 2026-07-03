use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{FloatScalar, FrameRate, Time, conversion, macros::impl_inner_op_family_forwarding};

/// Represents a discrete, non-negative frame position in a timeline, typically used as an integer
/// index into a frame-based sequence.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct FrameIndex(u32);

impl FrameIndex {
    /// A constant representing the zero frame index.
    pub const ZERO: Self = Self(0);

    /// Creates a new `FrameIndex` instance from the given raw value.
    #[inline]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the raw value of this `FrameIndex` instance.
    #[inline]
    pub const fn get(&self) -> u32 {
        self.0
    }

    /// Creates a new `FrameIndex` instance from the given `Time` at the specified frame rate.
    #[inline]
    pub fn from_time<T: FloatScalar>(time: Time<T>, rate: FrameRate<T>) -> Self {
        conversion::frame_from_time(time, rate)
    }

    /// Converts this `FrameIndex` to a `Time` at the specified frame rate.
    #[inline]
    pub fn to_time<T: FloatScalar>(self, rate: FrameRate<T>) -> Time<T> {
        conversion::time_from_frame(self, rate)
    }

    /// Returns the offset in frames from this `FrameIndex` to another `FrameIndex`.
    /// The result is positive if `self` is ahead of `other`, negative if behind, and zero if they
    /// are the same.
    #[inline]
    pub fn offset_from(self, other: FrameIndex) -> i32 {
        let a = self.0 as i64;
        let b = other.0 as i64;
        (a - b) as i32
    }

    /// Returns the next `FrameIndex` in sequence, saturating at the maximum value of `u32::MAX`.
    #[inline]
    pub const fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }

    /// Returns the previous `FrameIndex` in sequence, saturating at zero.
    #[inline]
    pub const fn prev(self) -> Self {
        Self(self.0.saturating_sub(1))
    }

    // Add+Sub impls (saturating, wrapping, and checked).
    impl_inner_op_family_forwarding!(add, u32, 0);
    impl_inner_op_family_forwarding!(sub, u32, 0);
}

// Raw conversion
impl From<u32> for FrameIndex {
    #[inline]
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<FrameIndex> for u32 {
    #[inline]
    fn from(frame: FrameIndex) -> Self {
        frame.get()
    }
}

// FrameIndex + u32 = FrameIndex
impl Add<u32> for FrameIndex {
    type Output = Self;

    #[inline]
    fn add(self, rhs: u32) -> Self::Output {
        self.saturating_add(rhs)
    }
}

// FrameIndex += u32
impl AddAssign<u32> for FrameIndex {
    #[inline]
    fn add_assign(&mut self, rhs: u32) {
        *self = self.saturating_add(rhs);
    }
}

// FrameIndex - u32 = FrameIndex
impl Sub<u32> for FrameIndex {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: u32) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

// FrameIndex -= u32
impl SubAssign<u32> for FrameIndex {
    #[inline]
    fn sub_assign(&mut self, rhs: u32) {
        *self = self.saturating_sub(rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics_zero_new_get_next_prev() {
        assert_eq!(FrameIndex::ZERO.get(), 0);
        assert_eq!(FrameIndex::new(10).get(), 10);
        assert_eq!(FrameIndex::new(10).next(), FrameIndex::new(11));
        assert_eq!(FrameIndex::new(10).prev(), FrameIndex::new(9));
    }

    #[test]
    fn next_prev_saturate_at_bounds() {
        assert_eq!(FrameIndex::new(u32::MAX).next(), FrameIndex::new(u32::MAX));
        assert_eq!(FrameIndex::ZERO.prev(), FrameIndex::ZERO);
    }

    #[test]
    fn offset_from_reports_signed_difference() {
        assert_eq!(FrameIndex::new(10).offset_from(FrameIndex::new(4)), 6);
        assert_eq!(FrameIndex::new(4).offset_from(FrameIndex::new(10)), -6);
        assert_eq!(FrameIndex::new(8).offset_from(FrameIndex::new(8)), 0);
    }

    #[test]
    fn saturating_checked_wrapping_add_sub_work() {
        let x = FrameIndex::new(u32::MAX - 1);
        assert_eq!(x.saturating_add(10), FrameIndex::new(u32::MAX));
        assert_eq!(x.checked_add(1), Some(FrameIndex::new(u32::MAX)));
        assert_eq!(x.checked_add(2), None);
        assert_eq!(x.wrapping_add(2), FrameIndex::new(0));

        let y = FrameIndex::new(1);
        assert_eq!(y.saturating_sub(10), FrameIndex::ZERO);
        assert_eq!(y.checked_sub(1), Some(FrameIndex::ZERO));
        assert_eq!(y.checked_sub(2), None);
        assert_eq!(y.wrapping_sub(2), FrameIndex::new(u32::MAX));
    }

    #[test]
    fn add_sub_and_assign_use_saturating_behavior() {
        let mut i = FrameIndex::new(u32::MAX - 1);
        i += 10;
        assert_eq!(i, FrameIndex::new(u32::MAX));

        i -= u32::MAX;
        assert_eq!(i, FrameIndex::ZERO);

        assert_eq!(FrameIndex::new(5) + 10, FrameIndex::new(15));
        assert_eq!(FrameIndex::new(5) - 10, FrameIndex::ZERO);
    }

    #[test]
    fn conversion_to_and_from_time_matches_rate() {
        let rate = FrameRate::from_fps(30.0_f64);
        let frame = FrameIndex::new(45);
        let time = frame.to_time(rate);
        let frame2 = FrameIndex::from_time(time, rate);

        assert_eq!(time, Time::from_seconds(1.5));
        assert_eq!(frame2, frame);
    }
}
