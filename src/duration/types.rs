#[cfg(feature = "zerocopy")]
use zerocopy::*;

/// Represents a span of time in seconds.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "sakka", derive(sakka::Encode, sakka::Decode))]
#[cfg_attr(feature = "zerocopy", derive(FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned))]
#[repr(transparent)]
pub struct Duration<T>(pub(crate) T);
