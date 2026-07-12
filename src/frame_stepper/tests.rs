#![cfg(test)]

use rinia::assert_approx_eq_abs_tol;

use crate::{FrameIndex, FrameRate, FrameStepper, Time};

mod construction {
    use super::*;

    #[test]
    fn new_starts_at_zero_and_uses_given_rate() {
        let rate = FrameRate::new(24.0_f32);
        let stepper = FrameStepper::<u32, f32>::new(rate);

        assert_eq!(stepper.current(), FrameIndex::new(0_u32));
        assert_eq!(stepper.frame_rate(), rate);
    }
}

mod stepping {
    use super::*;

    #[test]
    fn step_moves_forward_and_backward() {
        let mut stepper = FrameStepper::<u32, f32>::new(FrameRate::new(24.0_f32));

        stepper.step(10);
        assert_eq!(stepper.current(), FrameIndex::new(10_u32));

        stepper.step(-3);
        assert_eq!(stepper.current(), FrameIndex::new(7_u32));
    }

    #[test]
    fn step_saturates_at_bounds() {
        let mut stepper = FrameStepper::<u32, f32>::new(FrameRate::new(24.0_f32));

        stepper.step(-1);
        assert_eq!(stepper.current(), FrameIndex::new(0_u32));

        stepper.seek_raw(u32::MAX - 1);
        stepper.step(10);
        assert_eq!(stepper.current(), FrameIndex::new(u32::MAX));
    }

    #[test]
    fn step_forward_and_backward_use_single_frame_steps() {
        let mut stepper = FrameStepper::<u32, f32>::new(FrameRate::new(24.0_f32));

        stepper.step_forward();
        stepper.step_forward();
        assert_eq!(stepper.current(), FrameIndex::new(2_u32));

        stepper.step_backward();
        assert_eq!(stepper.current(), FrameIndex::new(1_u32));

        stepper.seek_raw(0_u32);
        stepper.step_backward();
        assert_eq!(stepper.current(), FrameIndex::new(0_u32));
    }
}

mod seeking_and_time {
    use super::*;

    #[test]
    fn seek_and_seek_raw_update_current_frame() {
        let mut stepper = FrameStepper::<u32, f32>::new(FrameRate::new(24.0_f32));

        stepper.seek(FrameIndex::new(12_u32));
        assert_eq!(stepper.current(), FrameIndex::new(12_u32));

        stepper.seek_raw(5_u32);
        assert_eq!(stepper.current(), FrameIndex::new(5_u32));
    }

    #[test]
    fn seek_time_and_current_time_roundtrip_at_rate() {
        let mut stepper = FrameStepper::<u32, f32>::new(FrameRate::new(24.0_f32));

        stepper.seek_time(Time::new(1.5_f32));
        assert_eq!(stepper.current(), FrameIndex::new(36_u32));

        let t = stepper.current_time();
        assert_approx_eq_abs_tol!(t.seconds(), 1.5_f32, 1e-6_f32);
    }
}

mod frame_rate_updates {
    use super::*;

    #[test]
    fn set_frame_rate_updates_rate_without_changing_frame() {
        let mut stepper = FrameStepper::<u32, f32>::new(FrameRate::new(24.0_f32));
        stepper.seek_raw(48_u32);

        stepper.set_frame_rate(FrameRate::new(60.0_f32));

        assert_eq!(stepper.current(), FrameIndex::new(48_u32));
        assert_eq!(stepper.frame_rate(), FrameRate::new(60.0_f32));
        assert_approx_eq_abs_tol!(stepper.current_time().seconds(), 0.8_f32, 1e-6_f32);
    }
}
