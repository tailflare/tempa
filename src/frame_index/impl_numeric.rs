use crate::{FrameIndex, common};

common::impl_numeric_zero!(FrameIndex<T>);
common::impl_numeric_bounded_min!(FrameIndex<T>);
common::impl_numeric_bounded_max!(FrameIndex<T>);
common::impl_numeric_min_max!(FrameIndex<T>);

rinia::impl_numeric_casts_wrapper!(FrameIndex<T>);
