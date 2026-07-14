#![cfg(test)]

/// A basic demonstration of using Tempa.
#[test]
fn tempa_example() {
    use rinia::assert_approx_eq_abs_tol;

    use crate::{FrameIndex, FrameRate, FrameStepper, Time, TimeRange};

    // Create a frame stepper running at 60 FPS and a simple 10-second clip.
    let rate = FrameRate::from_fps(60.0_f64);
    let mut stepper = FrameStepper::<u32, f64>::new(rate);
    let clip = TimeRange::new(Time::from_seconds(0.0_f64), Time::from_seconds(10.0_f64));

    // Advance the playhead by 120 frames (2 seconds at 60 FPS).
    stepper.step(120);
    assert_eq!(stepper.current(), FrameIndex::new(120_u32));
    assert_approx_eq_abs_tol!(stepper.current_time().seconds(), 2.0_f64, 1e-12_f64);

    // Convert explicitly between discrete frames and continuous time.
    let frame: FrameIndex<u32> = Time::from_seconds(1.25_f64).to_frame(rate);
    assert_eq!(frame, FrameIndex::new(75_u32));

    let time = Time::from_frame(frame, rate);
    assert_approx_eq_abs_tol!(time.seconds(), 1.25_f64, 1e-12_f64);

    // Seek to an arbitrary time. Times between frame boundaries are rounded
    // down to the frame currently containing that instant.
    stepper.seek_time(Time::from_seconds(2.75_f64));
    assert_eq!(stepper.current(), FrameIndex::new(165_u32));
    assert_approx_eq_abs_tol!(stepper.current_time().seconds(), 2.75_f64, 1e-12_f64);

    // Normalize the playhead position within the clip.
    let u = clip.normalize_time(stepper.current_time());
    assert_approx_eq_abs_tol!(u, 0.275_f64, 1e-12_f64);

    // Remap the same normalized position into another time range.
    let outro = TimeRange::new(Time::from_seconds(8.0_f64), Time::from_seconds(10.0_f64));
    let outro_time = clip.remap_time(stepper.current_time(), &outro);
    assert_approx_eq_abs_tol!(outro_time.seconds(), 8.55_f64, 1e-12_f64);

    // Construct ranges directly from scalar seconds.
    let window: TimeRange<f64> = (1.0..3.5).into();
    assert_approx_eq_abs_tol!(window.duration().seconds(), 2.5_f64, 1e-12_f64);

    // Merge overlapping ranges, for example when combining timeline edits.
    let a = TimeRange::new(Time::from_seconds(1.0_f64), Time::from_seconds(4.0_f64));
    let b = TimeRange::new(Time::from_seconds(3.5_f64), Time::from_seconds(6.0_f64));
    let merged = a.union(&b);
    assert_eq!(merged, TimeRange::new(Time::from_seconds(1.0_f64), Time::from_seconds(6.0_f64)));

    // Stepping past the beginning of the timeline saturates at frame zero.
    stepper.step(-10_000);
    assert_eq!(stepper.current(), FrameIndex::ZERO);
    assert_approx_eq_abs_tol!(stepper.current_time().seconds(), 0.0_f64, 1e-12_f64);
}
