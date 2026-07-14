use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{Duration, Time};

// Time<T> - Time<T> = Duration<T>
impl<T> Sub for Time<T>
where
    T: Sub<Output = T>,
{
    type Output = Duration<T>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Duration(self.0 - rhs.0)
    }
}

// Time<T> - T = Time<T>
impl<T> Sub<T> for Time<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        Self(self.0 - rhs)
    }
}

// Time<T> -= T
impl<T> SubAssign<T> for Time<T>
where
    T: SubAssign,
{
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        self.0 -= rhs;
    }
}

// Time<T> + Duration<T> = Time<T>
impl<T> Add<Duration<T>> for Time<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Duration<T>) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

// Time<T> += Duration<T>
impl<T> AddAssign<Duration<T>> for Time<T>
where
    T: AddAssign,
{
    #[inline]
    fn add_assign(&mut self, rhs: Duration<T>) {
        self.0 += rhs.0;
    }
}

// Time<T> - Duration<T> = Time<T>
impl<T> Sub<Duration<T>> for Time<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Duration<T>) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

// Time<T> -= Duration<T>
impl<T> SubAssign<Duration<T>> for Time<T>
where
    T: SubAssign,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Duration<T>) {
        self.0 -= rhs.0;
    }
}
