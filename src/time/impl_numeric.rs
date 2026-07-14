use crate::{Time, common};

common::impl_numeric_zero!(Time<T>);
common::impl_numeric_bounded_min!(Time<T>);
common::impl_numeric_bounded_max!(Time<T>);
common::impl_numeric_min_max!(Time<T>);
common::impl_numeric_is_finite!(Time<T>);
