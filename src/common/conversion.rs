use core::ops::{Div, Mul};

use rinia::numeric::{Floor, LossyCast, SaturatingCast};

use crate::{FrameIndex, FrameRate, Time};

/// Converts a [FrameIndex] to [Time] using the given [FrameRate].
#[inline]
pub fn time_from_frame<T, I>(frame: FrameIndex<I>, rate: FrameRate<T>) -> Time<T>
where
    I: Copy + LossyCast<T>,
    T: Copy + Div<Output = T>,
{
    Time::new(frame.index().lossy_cast() / rate.fps())
}

/// Converts a [Time] to [FrameIndex] using the given [FrameRate].
#[inline]
pub fn frame_from_time<T, I>(time: Time<T>, rate: FrameRate<T>) -> FrameIndex<I>
where
    T: Copy + Mul<Output = T> + Floor + SaturatingCast<I>,
{
    let frame = (time.seconds() * rate.fps()).floor();
    FrameIndex::new(frame.saturating_cast())
}
