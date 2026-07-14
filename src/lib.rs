#![no_std]
#![deny(rustdoc::broken_intra_doc_links)]

mod common;
mod duration;
mod error;
mod example;
mod frame_index;
mod frame_rate;
mod frame_stepper;
mod time;
mod time_range;

pub use self::{
    duration::{Duration, HasDuration},
    error::Error,
    frame_index::FrameIndex,
    frame_rate::FrameRate,
    frame_stepper::FrameStepper,
    time::Time,
    time_range::{HasTimeRange, TimeRange},
};
