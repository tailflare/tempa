#[cfg(feature = "zerocopy")]
use zerocopy::*;

/// Represents a frame rate in frames per second.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "sakka", derive(sakka::Encode, sakka::Decode))]
#[cfg_attr(feature = "zerocopy", derive(FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned))]
#[repr(transparent)]
pub struct FrameRate<T>(pub(crate) T);
