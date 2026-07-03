macro_rules! impl_zero_forwarding {
    ($scalar:ty, $inner:tt) => {
        #[inline]
        pub fn zero() -> Self {
            Self(<$scalar>::zero())
        }

        #[inline]
        pub fn is_zero(&self) -> bool {
            self.$inner.is_zero()
        }
    };
}

macro_rules! impl_min_max_forwarding {
    ($scalar:ty, $inner:tt) => {
        #[inline]
        pub fn min(self, other: Self) -> Self {
            Self(self.$inner.min(other.$inner))
        }

        #[inline]
        pub fn max(self, other: Self) -> Self {
            Self(self.$inner.max(other.$inner))
        }
    };
}

macro_rules! impl_inner_op_family_forwarding {
    ($op:ident, $rhs:ty, $inner:tt) => {
        pastey::paste! {
            #[inline]
            pub const fn [<saturating_ $op>](self, rhs: $rhs) -> Self {
                Self(self.$inner.[<saturating_ $op>](rhs))
            }

            #[inline]
            pub const fn [<wrapping_ $op>](self, rhs: $rhs) -> Self {
                Self(self.$inner.[<wrapping_ $op>](rhs))
            }

            #[inline]
            pub const fn [<checked_ $op>](self, rhs: $rhs) -> Option<Self> {
                if let Some(value) = self.$inner.[<checked_ $op>](rhs) { Some(Self(value)) } else { None }
            }
        }
    };
}

macro_rules! impl_approx_forwarding {
    ($wrapper:ident<$scalar:ident>, $($inner:tt),+ $(,)?) => {
        impl<$scalar: crate::FloatScalar + approx::AbsDiffEq<Epsilon = $scalar>> approx::AbsDiffEq
            for $wrapper<$scalar>
        {
            type Epsilon = $scalar;

            #[inline]
            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                true $( && self.$inner.abs_diff_eq(&other.$inner, epsilon) )+
            }

            #[inline]
            fn default_epsilon() -> Self::Epsilon {
                <$scalar as approx::AbsDiffEq>::default_epsilon()
            }
        }

        impl<$scalar: crate::FloatScalar + approx::RelativeEq<Epsilon = $scalar>> approx::RelativeEq
            for $wrapper<$scalar>
        {
            #[inline]
            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                true $( && self.$inner.relative_eq(&other.$inner, epsilon, max_relative) )+
            }

            #[inline]
            fn default_max_relative() -> Self::Epsilon {
                <$scalar as approx::RelativeEq>::default_max_relative()
            }
        }

        impl<$scalar: crate::FloatScalar + approx::UlpsEq<Epsilon = $scalar>> approx::UlpsEq
            for $wrapper<$scalar>
        {
            #[inline]
            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                true $( && self.$inner.ulps_eq(&other.$inner, epsilon, max_ulps) )+
            }

            #[inline]
            fn default_max_ulps() -> u32 {
                <$scalar as approx::UlpsEq>::default_max_ulps()
            }
        }
    };
}

pub(crate) use impl_approx_forwarding;
pub(crate) use impl_inner_op_family_forwarding;
pub(crate) use impl_min_max_forwarding;
pub(crate) use impl_zero_forwarding;
