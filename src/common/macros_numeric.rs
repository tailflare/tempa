macro_rules! impl_numeric_zero {
    ($wrapper:ident<$t:ident>) => {
        impl<$t> $wrapper<$t>
        where
            $t: ::rinia::numeric::Zero,
        {
            pub const ZERO: Self = Self(<$t as ::rinia::numeric::Zero>::ZERO);
        }

        impl<$t> ::rinia::numeric::Zero for $wrapper<$t>
        where
            $t: ::rinia::numeric::Zero,
        {
            const ZERO: Self = $wrapper::ZERO;
        }
    };
}

macro_rules! impl_numeric_bounded_min {
    ($wrapper:ident<$t:ident>) => {
        impl<$t> $wrapper<$t>
        where
            $t: ::rinia::numeric::BoundedMin,
        {
            pub const MIN: Self = Self(<$t as ::rinia::numeric::BoundedMin>::MIN);
        }

        impl<$t> ::rinia::numeric::BoundedMin for $wrapper<$t>
        where
            $t: ::rinia::numeric::BoundedMin,
        {
            const MIN: Self = $wrapper::MIN;
        }
    };
}

macro_rules! impl_numeric_bounded_max {
    ($wrapper:ident<$t:ident>) => {
        impl<$t> $wrapper<$t>
        where
            $t: ::rinia::numeric::BoundedMax,
        {
            pub const MAX: Self = Self(<$t as ::rinia::numeric::BoundedMax>::MAX);
        }

        impl<$t> ::rinia::numeric::BoundedMax for $wrapper<$t>
        where
            $t: ::rinia::numeric::BoundedMax,
        {
            const MAX: Self = $wrapper::MAX;
        }
    };
}

macro_rules! impl_numeric_min_max {
    ($wrapper:ident<$t:ident>) => {
        impl<$t> $wrapper<$t>
        where
            $t: ::rinia::numeric::MinMax,
        {
            pub fn min(self, other: Self) -> Self {
                Self(<$t as ::rinia::numeric::MinMax>::minimum(self.0, other.0))
            }

            pub fn max(self, other: Self) -> Self {
                Self(<$t as ::rinia::numeric::MinMax>::maximum(self.0, other.0))
            }
        }

        impl<$t> ::rinia::numeric::MinMax for $wrapper<$t>
        where
            $t: ::rinia::numeric::MinMax,
        {
            fn minimum(self, other: Self) -> Self {
                $wrapper::min(self, other)
            }

            fn maximum(self, other: Self) -> Self {
                $wrapper::max(self, other)
            }
        }
    };
}

macro_rules! impl_numeric_is_finite {
    ($wrapper:ident<$t:ident>) => {
        impl<$t> $wrapper<$t>
        where
            $t: Copy + ::rinia::numeric::IsFinite,
        {
            pub fn is_finite(self) -> bool {
                <$t as ::rinia::numeric::IsFinite>::is_finite(self.0)
            }
        }

        impl<$t> ::rinia::numeric::IsFinite for $wrapper<$t>
        where
            $t: Copy + ::rinia::numeric::IsFinite,
        {
            fn is_finite(self) -> bool {
                $wrapper::is_finite(self)
            }
        }
    };
}

pub(crate) use impl_numeric_bounded_max;
pub(crate) use impl_numeric_bounded_min;
pub(crate) use impl_numeric_is_finite;
pub(crate) use impl_numeric_min_max;
pub(crate) use impl_numeric_zero;
