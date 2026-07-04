use rinia::FloatScalar;

use crate::{FrameIndex, FrameRate, Time};

/// Converts a [FrameIndex] to [Time] using the given [FrameRate].
#[inline]
pub fn time_from_frame<T: FloatScalar>(frame: FrameIndex, rate: FrameRate<T>) -> Time<T> {
    Time::from_seconds(T::from_scalar(frame.get()) / rate.fps())
}

/// Converts a [Time] to the corresponding [FrameIndex] using the given [FrameRate],
/// rounding down to the frame that contains that time.
#[inline]
pub fn frame_from_time<T: FloatScalar>(time: Time<T>, rate: FrameRate<T>) -> FrameIndex {
    // floor: sample the frame currently containing this time
    let raw = (time.seconds() * rate.fps()).floor();

    // clamp negative time to frame 0 (animation convention)
    // should be impossible but we defensively clamp incase unchecked functions were used.
    let clamped = raw.max(T::ZERO);

    // saturate to u32::MAX to avoid overflow when converting to FrameIndex
    let max_u32 = T::from_scalar(u32::MAX);
    let frame_val = if clamped >= max_u32 { u32::MAX } else { clamped.as_scalar() };

    FrameIndex::new(frame_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_from_frame_converts_using_fps() {
        let frame = FrameIndex::new(90);
        let rate = FrameRate::from_fps(30.0_f64);

        let time = time_from_frame(frame, rate);

        assert_eq!(time, Time::from_seconds(3.0));
    }

    #[test]
    fn frame_from_time_floors_to_previous_whole_frame() {
        let time = Time::from_seconds(1.999_f64);
        let rate = FrameRate::from_fps(30.0_f64);

        let frame = frame_from_time(time, rate);

        assert_eq!(frame, FrameIndex::new(59));
    }

    #[test]
    fn frame_from_time_clamps_negative_values_to_zero() {
        let time = Time::from_seconds(-2.0_f64);
        let rate = FrameRate::from_fps(24.0_f64);

        let frame = frame_from_time(time, rate);

        assert_eq!(frame, FrameIndex::ZERO);
    }
}
