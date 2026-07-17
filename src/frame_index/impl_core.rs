use core::ops::{Div, Mul, Sub};

use rinia::numeric::{
    Floor, LossyCast, One, SaturatingAdd, SaturatingCast, SaturatingSub, SignedCast,
    SignedEquivalent, Zero,
};

use crate::{FrameIndex, FrameRate, Time, common};

impl<T> FrameIndex<T> {
    /// Creates a new [FrameIndex] from the given value.
    #[inline]
    pub fn new(value: T) -> Self {
        FrameIndex(value)
    }

    /// Returns the raw frame value.
    #[inline]
    pub const fn index(&self) -> T
    where
        T: Copy,
    {
        self.0
    }

    /// Consumes this [FrameIndex] value and returns the inner frame value.
    #[inline]
    pub fn into_index(self) -> T {
        self.0
    }

    /// Creates a [FrameIndex] from [Time] at the specified [FrameRate].
    #[inline]
    pub fn from_time<S>(time: Time<S>, rate: FrameRate<S>) -> Self
    where
        S: Copy + Mul<Output = S> + Floor + SaturatingCast<T>,
    {
        common::frame_from_time(time, rate)
    }

    /// Converts this [FrameIndex] to [Time] at the specified [FrameRate].
    #[inline]
    pub fn to_time<S>(self, rate: FrameRate<S>) -> Time<S>
    where
        T: Copy + LossyCast<S>,
        S: Copy + Div<Output = S>,
    {
        common::time_from_frame(self, rate)
    }

    /// Returns the signed frame offset from this index to another index.
    /// The result is positive when self is ahead of other, negative when behind, and zero when
    /// equal.
    #[inline]
    pub fn offset_from(self, other: Self) -> <T as SignedEquivalent>::Signed
    where
        T: SignedEquivalent + SignedCast<<T as SignedEquivalent>::Signed>,
        <T as SignedEquivalent>::Signed: Sub<Output = <T as SignedEquivalent>::Signed>,
    {
        self.0.signed_cast() - other.0.signed_cast()
    }

    /// Returns the next [FrameIndex], saturating at `T::MAX`.
    #[inline]
    pub fn next(&self) -> Self
    where
        T: One + SaturatingAdd<Output = T>,
        Self: Copy + SaturatingAdd<T, Output = Self>,
    {
        self.saturating_add_t(T::ONE)
    }

    /// Returns the previous [FrameIndex], saturating at `T::MIN`.
    #[inline]
    pub fn previous(&self) -> Self
    where
        T: One + SaturatingSub<Output = T>,
        Self: Copy + SaturatingSub<T, Output = Self>,
    {
        self.saturating_sub_t(T::ONE)
    }
}

// Default
impl<T> Default for FrameIndex<T>
where
    Self: Zero,
{
    fn default() -> Self {
        Self::ZERO
    }
}

// Conversion
impl<T> From<T> for FrameIndex<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl From<FrameIndex<u32>> for u32 {
    #[inline]
    fn from(frame: FrameIndex<u32>) -> Self {
        frame.index()
    }
}

impl From<FrameIndex<u64>> for u64 {
    #[inline]
    fn from(frame: FrameIndex<u64>) -> Self {
        frame.index()
    }
}

impl From<FrameIndex<usize>> for usize {
    #[inline]
    fn from(frame: FrameIndex<usize>) -> Self {
        frame.index()
    }
}

// Impl approx equality traits for FrameIndex<T>
rinia::impl_approx_eq_wrapper!([T], impl: FrameIndex<T>, item: T);

// Impl bytemuck for FrameIndex<T>
common::impl_bytemuck_basic!(
    [T],
    FrameIndex<T>,
    item: T,
);
