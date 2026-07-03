//! Tempa is a no_std temporal system for animation and simulation, providing time, duration, time ranges, frame rates, and deterministic frame-index stepping.

#![no_std]

mod duration;
mod frame_index;
mod frame_rate;
mod frame_stepper;
pub(crate) mod macros;
mod scalar;
mod time;
mod time_range;

#[cfg(test)]
mod example;

pub mod conversion;

pub use self::{
    duration::{Duration, HasDuration},
    frame_index::FrameIndex,
    frame_rate::FrameRate,
    frame_stepper::FrameStepper,
    scalar::FloatScalar,
    time::Time,
    time_range::{HasTimeRange, TimeRange},
};
