use crate::{Duration, common};

common::impl_numeric_zero!(Duration<T>);
common::impl_numeric_bounded_min!(Duration<T>);
common::impl_numeric_bounded_max!(Duration<T>);
common::impl_numeric_min_max!(Duration<T>);
common::impl_numeric_is_finite!(Duration<T>);

rinia::impl_numeric_casts_wrapper!(Duration<T>);
