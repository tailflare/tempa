use core::ops::{Div, DivAssign, Mul, MulAssign};

use crate::FrameRate;

// FrameRate / FrameRate = T (ratio)
impl<T> Div for FrameRate<T>
where
    T: Copy + Div<Output = T>,
{
    type Output = T;

    #[inline]
    fn div(self, rhs: Self) -> T {
        self.fps() / rhs.fps()
    }
}

// FrameRate * T = FrameRate
impl<T> Mul<T> for FrameRate<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.fps() * rhs)
    }
}

// FrameRate *= T
impl<T> MulAssign<T> for FrameRate<T>
where
    T: Copy + Mul<Output = T>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        *self = Self::new(self.fps() * rhs);
    }
}

// FrameRate / T = FrameRate
impl<T> Div<T> for FrameRate<T>
where
    T: Copy + Div<Output = T>,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.fps() / rhs)
    }
}

// FrameRate /= T
impl<T> DivAssign<T> for FrameRate<T>
where
    T: Copy + Div<Output = T>,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        *self = Self::new(self.fps() / rhs);
    }
}
