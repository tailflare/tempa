use core::ops::Div;

use rinia::numeric::{Floor, LossyCast, One, SaturatingAdd, SaturatingCast, SaturatingSub, Zero};

use crate::{FrameIndex, FrameRate, FrameStepper, Time, common};

impl<I, R> FrameStepper<I, R> {
    /// Creates a [FrameStepper] at frame 0 with the specified [FrameRate].
    #[inline]
    pub fn new(rate: FrameRate<R>) -> Self
    where
        I: Zero,
    {
        Self { frame: FrameIndex::ZERO, rate }
    }

    /// Returns the current [FrameIndex].
    #[inline]
    pub fn current(&self) -> FrameIndex<I>
    where
        I: Copy,
    {
        self.frame
    }

    /// Returns the current [FrameRate].
    #[inline]
    pub fn frame_rate(&self) -> FrameRate<R>
    where
        R: Copy,
    {
        self.rate
    }

    /// Returns the current [Time].
    #[inline]
    pub fn current_time(&self) -> Time<R>
    where
        I: Copy + LossyCast<R>,
        R: Copy + Div<Output = R>,
    {
        common::time_from_frame(self.frame, self.rate)
    }

    /// Steps by n frames.
    /// Positive values move forward, negative values move backward, and movement saturates within
    /// [FrameIndex] bounds.
    #[inline]
    pub fn step(&mut self, n: i32)
    where
        i32: SaturatingCast<I>,
        I: Copy + SaturatingAdd<Output = I> + SaturatingSub<Output = I>,
    {
        let delta: I = n.saturating_abs().saturating_cast();

        if n >= 0 {
            self.frame = self.frame.saturating_add_t(delta);
        } else {
            self.frame = self.frame.saturating_sub_t(delta);
        }
    }

    /// Steps forward by one frame.
    #[inline]
    pub fn step_forward(&mut self)
    where
        I: One + SaturatingAdd<Output = I>,
        FrameIndex<I>: Copy + SaturatingAdd<I, Output = FrameIndex<I>>,
    {
        self.frame = self.frame.next();
    }

    /// Steps backward by one frame.
    #[inline]
    pub fn step_backward(&mut self)
    where
        I: One + SaturatingSub<Output = I>,
        FrameIndex<I>: Copy + SaturatingSub<I, Output = FrameIndex<I>>,
    {
        self.frame = self.frame.previous();
    }

    /// Seeks to the specified [FrameIndex].
    #[inline]
    pub fn seek(&mut self, frame: FrameIndex<I>) {
        self.frame = frame;
    }

    /// Seeks to a raw frame value.
    #[inline]
    pub fn seek_raw(&mut self, raw: I) {
        self.frame = FrameIndex::new(raw);
    }

    /// Seeks to [Time], converted using the current [FrameRate].
    ///
    /// Use [Time] directly when sub-frame precision is required.
    #[inline]
    pub fn seek_time(&mut self, time: Time<R>)
    where
        R: Copy + core::ops::Mul<Output = R> + Floor + SaturatingCast<I>,
    {
        self.frame = common::frame_from_time(time, self.rate);
    }

    /// Sets a new [FrameRate].
    ///
    /// Reinterprets the current frame index under a new frame rate.
    /// Does not modify the current frame position.
    #[inline]
    pub fn set_frame_rate(&mut self, rate: FrameRate<R>) {
        self.rate = rate;
    }
}

// Impl bytemuck for FrameStepper<I, R>
common::impl_bytemuck_basic!(
    [I, R],
    FrameStepper<I, R>,
    item: FrameStepper<I, R>,
);
