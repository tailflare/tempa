macro_rules! impl_approx_forwarding {
	($wrapper:ident<$scalar:ident>, $($inner:tt),+ $(,)?) => {
		// ApproxEqAbs trait
        impl<$scalar> ::rinia::algebra::ApproxEqAbs for $wrapper<$scalar>
		where
			$scalar: Copy + ::rinia::algebra::ApproxEqAbs<Tolerance = $scalar>,
		{
			type Tolerance = $scalar;

			const DEFAULT_TOLERANCE_ABS: Self::Tolerance =
				<$scalar as ::rinia::algebra::ApproxEqAbs>::DEFAULT_TOLERANCE_ABS;

			#[inline]
			fn approx_eq_abs_tol(self, other: Self, tol: Self::Tolerance) -> bool {
				true $( && ::rinia::algebra::ApproxEqAbs::approx_eq_abs_tol(self.$inner, other.$inner, tol) )+
			}
		}

        // ApproxEqRel trait
		impl<$scalar> ::rinia::algebra::ApproxEqRel for $wrapper<$scalar>
		where
			$scalar: Copy + ::rinia::algebra::ApproxEqRel<Tolerance = $scalar>,
		{
			type Tolerance = $scalar;

			const DEFAULT_TOLERANCE_REL: Self::Tolerance =
				<$scalar as ::rinia::algebra::ApproxEqRel>::DEFAULT_TOLERANCE_REL;

			#[inline]
			fn approx_eq_rel_tol(self, other: Self, tol: Self::Tolerance) -> bool {
				true $( && ::rinia::algebra::ApproxEqRel::approx_eq_rel_tol(self.$inner, other.$inner, tol) )+
			}
		}
	};
}

pub(crate) use impl_approx_forwarding;
