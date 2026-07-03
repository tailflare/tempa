use crate::{FloatScalar, FrameIndex, FrameRate, Time, conversion};

/// Steps through timeline frames deterministically.
pub struct FrameStepper<T: FloatScalar> {
    frame: FrameIndex,
    rate: FrameRate<T>,
}

impl<T: FloatScalar> FrameStepper<T> {
    /// Creates a [FrameStepper] at frame 0 with the specified [FrameRate].
    pub fn new(rate: FrameRate<T>) -> Self {
        Self { frame: FrameIndex::ZERO, rate }
    }

    /// Returns the current [FrameIndex].
    pub fn current(&self) -> FrameIndex {
        self.frame
    }

    /// Returns the current [FrameRate].
    pub fn frame_rate(&self) -> FrameRate<T> {
        self.rate
    }

    /// Returns the current [Time].
    pub fn current_time(&self) -> Time<T> {
        conversion::time_from_frame(self.frame, self.rate)
    }

    /// Steps by n frames.
    /// Positive values move forward, negative values move backward, and movement saturates within
    /// [FrameIndex] bounds.
    pub fn step(&mut self, n: i32) {
        if n >= 0 {
            self.frame = self.frame.saturating_add(n as u32);
        } else {
            self.frame = self.frame.saturating_sub((-n) as u32);
        }
    }

    /// Steps forward by one frame.
    pub fn step_forward(&mut self) {
        self.frame = self.frame.next();
    }

    /// Steps backward by one frame.
    pub fn step_backward(&mut self) {
        self.frame = self.frame.prev();
    }

    /// Seeks to the specified [FrameIndex].
    pub fn seek(&mut self, frame: FrameIndex) {
        self.frame = frame;
    }

    /// Seeks to a raw frame value.
    pub fn seek_raw(&mut self, raw: u32) {
        self.frame = FrameIndex::new(raw);
    }

    /// Seeks to [Time], converted using the current [FrameRate].
    ///
    /// Use [Time] directly when sub-frame precision is required.
    pub fn seek_time(&mut self, time: Time<T>) {
        self.frame = conversion::frame_from_time(time, self.rate);
    }

    /// Sets a new [FrameRate].
    ///
    /// Reinterprets the current frame index under a new frame rate.
    /// Does not modify the current frame position.
    pub fn set_frame_rate(&mut self, rate: FrameRate<T>) {
        self.rate = rate;
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;

    #[test]
    fn new_starts_at_zero_and_exposes_rate() {
        let rate = FrameRate::from_fps(24.0_f64);
        let stepper = FrameStepper::new(rate);

        assert_eq!(stepper.current(), FrameIndex::ZERO);
        assert_eq!(stepper.frame_rate(), rate);
        assert_abs_diff_eq!(stepper.current_time().seconds(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn step_handles_positive_negative_and_zero() {
        let rate = FrameRate::from_fps(30.0_f64);
        let mut stepper = FrameStepper::new(rate);

        stepper.step(10);
        assert_eq!(stepper.current(), FrameIndex::new(10));

        stepper.step(-3);
        assert_eq!(stepper.current(), FrameIndex::new(7));

        stepper.step(0);
        assert_eq!(stepper.current(), FrameIndex::new(7));
    }

    #[test]
    fn step_saturates_at_bounds() {
        let rate = FrameRate::from_fps(60.0_f64);
        let mut stepper = FrameStepper::new(rate);

        stepper.step(-1);
        assert_eq!(stepper.current(), FrameIndex::ZERO);

        stepper.seek_raw(u32::MAX - 1);
        stepper.step(10);
        assert_eq!(stepper.current(), FrameIndex::new(u32::MAX));

        stepper.step(-i32::MAX);
        assert_eq!(stepper.current(), FrameIndex::new(u32::MAX - i32::MAX as u32));
    }

    #[test]
    fn step_forward_and_backward_move_by_one_with_saturation() {
        let rate = FrameRate::from_fps(30.0_f64);
        let mut stepper = FrameStepper::new(rate);

        stepper.step_backward();
        assert_eq!(stepper.current(), FrameIndex::ZERO);

        stepper.step_forward();
        stepper.step_forward();
        assert_eq!(stepper.current(), FrameIndex::new(2));

        stepper.step_backward();
        assert_eq!(stepper.current(), FrameIndex::new(1));
    }

    #[test]
    fn seek_and_seek_raw_set_exact_frame() {
        let rate = FrameRate::from_fps(24.0_f64);
        let mut stepper = FrameStepper::new(rate);

        stepper.seek(FrameIndex::new(42));
        assert_eq!(stepper.current(), FrameIndex::new(42));

        stepper.seek_raw(7);
        assert_eq!(stepper.current(), FrameIndex::new(7));
    }

    #[test]
    fn seek_time_uses_stepper_rate_and_clamps_negative() {
        let rate = FrameRate::from_fps(30.0_f64);
        let mut stepper = FrameStepper::new(rate);

        stepper.seek_time(Time::from_seconds(1.8));
        assert_eq!(stepper.current(), FrameIndex::new(54));

        stepper.seek_time(Time::from_seconds(-10.0));
        assert_eq!(stepper.current(), FrameIndex::ZERO);
    }

    #[test]
    fn current_time_reflects_frame_and_rate() {
        let rate = FrameRate::from_fps(50.0_f64);
        let mut stepper = FrameStepper::new(rate);
        stepper.seek_raw(25);

        assert_abs_diff_eq!(stepper.current_time().seconds(), 0.5, epsilon = 1e-12);
    }

    #[test]
    fn set_frame_rate_reinterprets_time_not_frame() {
        let mut stepper = FrameStepper::new(FrameRate::from_fps(20.0_f64));
        stepper.seek_raw(40);

        assert_abs_diff_eq!(stepper.current_time().seconds(), 2.0, epsilon = 1e-12);

        stepper.set_frame_rate(FrameRate::from_fps(10.0_f64));

        assert_eq!(stepper.current(), FrameIndex::new(40));
        assert_abs_diff_eq!(stepper.current_time().seconds(), 4.0, epsilon = 1e-12);
        assert_eq!(stepper.frame_rate(), FrameRate::from_fps(10.0));
    }
}
