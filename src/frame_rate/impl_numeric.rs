use crate::{FrameRate, common};

common::impl_numeric_zero!(FrameRate<T>);
common::impl_numeric_bounded_max!(FrameRate<T>);
common::impl_numeric_min_max!(FrameRate<T>);
common::impl_numeric_is_finite!(FrameRate<T>);
