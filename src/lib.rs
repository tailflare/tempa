//! Tempa is a no_std temporal system for animation and simulation.
//! It provides [Time], [Duration], [TimeRange], [FrameRate], deterministic [FrameIndex]
//! stepping, and [FrameStepper] for frame-by-frame traversal.
//! It also includes traits for shared temporal behavior and conversion helpers.

#![no_std]
#![deny(rustdoc::broken_intra_doc_links)]

mod duration;
mod frame_index;
mod frame_rate;
mod frame_stepper;
pub(crate) mod macros;
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
    time::Time,
    time_range::{HasTimeRange, TimeRange},
};
