use num_traits::{Float, NumCast, ToPrimitive};

/// Floating-point scalar type used by Tempa time APIs.
///
/// This trait intentionally restricts [Float] to a known subset of types
/// used by Tempa (f32, f64) to preserve consistent numerical behavior
/// across time, duration, and animation systems.
pub trait FloatScalar: Float {
    /// Converts a primitive numeric value into this scalar type.
    ///
    /// # Panics
    /// Panics if the value cannot be represented as this scalar type.
    #[inline]
    fn raw<V>(value: V) -> Self
    where
        V: ToPrimitive,
    {
        <Self as NumCast>::from(value).expect("numeric value must be representable for FloatScalar")
    }
}

// We officially provide support for f32 and f64 as scalar types.
impl FloatScalar for f32 {}
impl FloatScalar for f64 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_converts_primitive_values() {
        let a = f32::raw(42_u8);
        let b = f64::raw(-7_i16);

        assert_eq!(a, 42.0_f32);
        assert_eq!(b, -7.0_f64);
    }
}
