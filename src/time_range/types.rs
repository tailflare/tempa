#[cfg(feature = "zerocopy")]
use zerocopy::*;

use crate::Time;

/// Represents a bounded time interval with a start and end [Time].
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "sakka", derive(sakka::Encode, sakka::Decode))]
#[cfg_attr(feature = "zerocopy", derive(FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned))]
#[repr(C)]
pub struct TimeRange<T> {
    pub(crate) start: Time<T>,
    pub(crate) end: Time<T>,
}
