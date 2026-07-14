# Tempa

[![Tag](https://img.shields.io/github/v/tag/tailflare/tempa)](https://github.com/tailflare/tempa/tags)
[![Crates.io Version](https://img.shields.io/crates/v/tempa)](https://crates.io/crates/tempa)
[![docs.rs](https://img.shields.io/docsrs/tempa)](https://docs.rs/tempa)
[![Main CI Build Status](https://img.shields.io/github/actions/workflow/status/tailflare/tempa/ci.yml?label=main%20build)](https://github.com/tailflare/tempa/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/tempa)](#license)


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
```toml
tempa = "0.1.7"
```

### no_std
```toml
tempa = { version = "0.1.7", default-features = false }
```

### Example
```rust
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
```

## License

The Tempa project is licensed under either the Apache License, Version 2.0 or the MIT license, at your option.

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
