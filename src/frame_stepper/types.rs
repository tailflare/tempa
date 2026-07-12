#[cfg(feature = "zerocopy")]
use zerocopy::*;

use crate::{FrameIndex, FrameRate};

/// Steps through timeline frames deterministically.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "sakka", derive(sakka::Encode, sakka::Decode))]
#[cfg_attr(feature = "zerocopy", derive(FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned))]
#[repr(C)]
pub struct FrameStepper<I, R> {
    pub(crate) frame: FrameIndex<I>,
    pub(crate) rate: FrameRate<R>,
}
