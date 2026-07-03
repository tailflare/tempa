# Tempa

[![Crates.io Version](https://img.shields.io/crates/v/tempa)](https://crates.io/crates/tempa)
[![docs.rs](https://img.shields.io/docsrs/tempa)](https://docs.rs/tempa)

A no_std library for working with time in animation and real-time systems.

It provides a small set of temporal primitives: time, duration, time ranges, frame rates, and frame-based stepping. Designed around deterministic conversion between continuous time and discrete frame indices.

This makes it suitable for animation systems, simulations, and engine tooling where frame-accurate behavior matters.

## Status

⚠️ Experimental / Work in Progress

The API is still evolving and may change significantly.

## Features

- no_std compatible
- Time and Duration types
- Frame rate representation
- Time ranges and range operations
- Deterministic frame <-> time conversion
- Frame-based stepping utilities

## Usage
### std
```
tempa = 0.1.1
```

### no_std
```
tempa = { version = 0.1.1, default-features = false }
```

### Example
```rust
// Create a frame stepper running at 60 FPS and a simple 10-second clip.
let rate = FrameRate::from_fps(60.0);
let mut stepper = FrameStepper::new(rate);
let clip = TimeRange::new(Time::from_seconds(0.0), Time::from_seconds(10.0));

// Advance the playhead by 120 frames (2 seconds at 60 FPS).
stepper.step(120);
assert_eq!(stepper.current(), FrameIndex::new(120));
assert_abs_diff_eq!(stepper.current_time().seconds(), 2.0, epsilon = 1e-12);

// Convert explicitly between discrete frames and continuous time.
let frame = conversion::frame_from_time(Time::from_seconds(1.25), rate);
assert_eq!(frame, FrameIndex::new(75));

let time = conversion::time_from_frame(frame, rate);
assert_abs_diff_eq!(time.seconds(), 1.25, epsilon = 1e-12);

// Seek to an arbitrary time. Times between frame boundaries are rounded
// down to the frame currently containing that instant.
stepper.seek_time(Time::from_seconds(2.75));
assert_eq!(stepper.current(), FrameIndex::new(165));
assert_abs_diff_eq!(stepper.current_time().seconds(), 2.75, epsilon = 1e-12);

// Normalize the playhead position within the clip.
let u = clip.normalize_time(stepper.current_time());
assert_abs_diff_eq!(u, 0.275, epsilon = 1e-12);

// Remap the same normalized position into another time range.
let outro = TimeRange::new(Time::from_seconds(8.0), Time::from_seconds(10.0));
let outro_time = clip.remap_time(stepper.current_time(), &outro);
assert_abs_diff_eq!(outro_time.seconds(), 8.55, epsilon = 1e-12);

// Construct ranges directly from scalar seconds.
let window: TimeRange<f64> = (1.0..3.5).into();
assert_abs_diff_eq!(window.duration().seconds(), 2.5, epsilon = 1e-12);

// Merge overlapping ranges, for example when combining timeline edits.
let a = TimeRange::new(Time::from_seconds(1.0), Time::from_seconds(4.0));
let b = TimeRange::new(Time::from_seconds(3.5), Time::from_seconds(6.0));
let merged = a.union(&b);
assert_eq!(merged, TimeRange::new(Time::from_seconds(1.0), Time::from_seconds(6.0)));

// Stepping past the beginning of the timeline saturates at frame zero.
stepper.step(-10_000);
assert_eq!(stepper.current(), FrameIndex::ZERO);
assert_abs_diff_eq!(stepper.current_time().seconds(), 0.0, epsilon = 1e-12);
```

## License

The Tempa project is licensed under either the Apache License, Version 2.0 or the MIT license, at your option.

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
