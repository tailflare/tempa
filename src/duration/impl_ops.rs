use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use rinia::{Scalard, Scalarf};

use crate::Duration;

// Duration<T> + Duration<T> = Duration<T>
impl<T> Add for Duration<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

// Duration<T> += Duration<T>
impl<T> AddAssign for Duration<T>
where
    T: AddAssign,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

// Duration<T> + T = Duration<T>
impl<T> Add<T> for Duration<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        Self(self.0 + rhs)
    }
}

// Duration<T> += T
impl<T> AddAssign<T> for Duration<T>
where
    T: AddAssign,
{
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        self.0 += rhs;
    }
}

// Duration<T> - Duration<T> = Duration<T>
impl<T> Sub for Duration<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

// Duration<T> -= Duration<T>
impl<T> SubAssign for Duration<T>
where
    T: SubAssign,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

// Duration<T> - T = Duration<T>
impl<T> Sub<T> for Duration<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        Self(self.0 - rhs)
    }
}

// Duration<T> -= T
impl<T> SubAssign<T> for Duration<T>
where
    T: SubAssign,
{
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        self.0 -= rhs;
    }
}

// Duration<T> / Duration<T> = T (ratio)
impl<T> Div for Duration<T>
where
    T: Div<Output = T>,
{
    type Output = T;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

// Duration<T> / T = Duration<T>
impl<T> Div<T> for Duration<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / rhs)
    }
}

// Druration<T> /= T
impl<T> DivAssign<T> for Duration<T>
where
    T: DivAssign,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.0 /= rhs;
    }
}

// Duration<T> * T = Duration<T>
impl<T> Mul<T> for Duration<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * rhs)
    }
}

// Duration<T> *= T
impl<T> MulAssign<T> for Duration<T>
where
    T: MulAssign,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= rhs;
    }
}

// Scalarf * Duration<Scalarf> = Duration<Scalarf>
impl Mul<Duration<Scalarf>> for Scalarf {
    type Output = Duration<Scalarf>;

    #[inline]
    fn mul(self, rhs: Duration<Scalarf>) -> Self::Output {
        Duration(self * rhs.0)
    }
}

// Scalard * Duration<Scalard> = Duration<Scalard>
impl Mul<Duration<Scalard>> for Scalard {
    type Output = Duration<Scalard>;

    #[inline]
    fn mul(self, rhs: Duration<Scalard>) -> Self::Output {
        Duration(self * rhs.0)
    }
}
