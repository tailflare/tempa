use core::ops::{Add, AddAssign, Sub, SubAssign};

use rinia::numeric::{SaturatingAdd, SaturatingSub};

use crate::FrameIndex;

// FrameIndex + FrameIndex = FrameIndex
impl<T> Add for FrameIndex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

// FrameIndex += FrameIndex
impl<T> AddAssign for FrameIndex<T>
where
    T: AddAssign,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

// FrameIndex - FrameIndex = FrameIndex
impl<T> Sub for FrameIndex<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

// FrameIndex -= FrameIndex
impl<T> SubAssign for FrameIndex<T>
where
    T: SubAssign,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

// FrameIndex + T = FrameIndex
impl<T> Add<T> for FrameIndex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        Self(self.0 + rhs)
    }
}

// FrameIndex += T
impl<T> AddAssign<T> for FrameIndex<T>
where
    T: AddAssign,
{
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        self.0 += rhs;
    }
}

// FrameIndex - T = FrameIndex
impl<T> Sub<T> for FrameIndex<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        Self(self.0 - rhs)
    }
}

// FrameIndex -= T
impl<T> SubAssign<T> for FrameIndex<T>
where
    T: SubAssign,
{
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        self.0 -= rhs;
    }
}

// SaturatingAdd inherent
impl<T> FrameIndex<T>
where
    T: SaturatingAdd<Output = T>,
{
    /// Returns the result of saturating addition of two [FrameIndex] values.
    #[inline]
    pub fn saturating_add(self, rhs: Self) -> Self {
        let value = self.0.saturating_add(rhs.0);
        Self(value)
    }

    /// Returns the result of saturating addition of a [FrameIndex] value and a raw frame value.
    #[inline]
    pub fn saturating_add_t(self, rhs: T) -> Self {
        let value = self.0.saturating_add(rhs);
        Self(value)
    }
}

// FrameIndex<T>.saturating_add(FrameIndex<T>) = FrameIndex<T>
impl<T> SaturatingAdd for FrameIndex<T>
where
    T: SaturatingAdd<Output = T>,
{
    type Output = Self;

    #[inline]
    fn saturating_add(self, rhs: Self) -> Self::Output {
        FrameIndex::saturating_add(self, rhs)
    }
}

// FrameIndex<T>.saturating_add(FrameIndex<T>) = FrameIndex<T>
impl<T> SaturatingAdd<T> for FrameIndex<T>
where
    T: SaturatingAdd<Output = T>,
{
    type Output = Self;

    #[inline]
    fn saturating_add(self, rhs: T) -> Self::Output {
        FrameIndex::saturating_add_t(self, rhs)
    }
}

// SaturatingSub inherent
impl<T> FrameIndex<T>
where
    T: SaturatingSub<Output = T>,
{
    /// Returns the result of saturating subtraction of two [FrameIndex] values.
    #[inline]
    pub fn saturating_sub(self, rhs: Self) -> Self {
        let value = self.0.saturating_sub(rhs.0);
        Self(value)
    }

    /// Returns the result of saturating subtraction of a [FrameIndex] value and a raw frame value.
    #[inline]
    pub fn saturating_sub_t(self, rhs: T) -> Self {
        let value = self.0.saturating_sub(rhs);
        Self(value)
    }
}

// FrameIndex<T>.saturating_sub(FrameIndex<T>) = FrameIndex<T>
impl<T> SaturatingSub for FrameIndex<T>
where
    T: SaturatingSub<Output = T>,
{
    type Output = Self;

    #[inline]
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        FrameIndex::saturating_sub(self, rhs)
    }
}

// FrameIndex<T>.saturating_sub(T) = FrameIndex<T>
impl<T> SaturatingSub<T> for FrameIndex<T>
where
    T: SaturatingSub<Output = T>,
{
    type Output = Self;

    #[inline]
    fn saturating_sub(self, rhs: T) -> Self::Output {
        FrameIndex::saturating_sub_t(self, rhs)
    }
}
