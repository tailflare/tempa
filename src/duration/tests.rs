#![cfg(test)]

use rinia::{
    Scalarf,
    numeric::{BoundedMax, BoundedMin, MinMax, Zero},
};

use crate::{Duration, Error, HasDuration};

mod construction {
    use super::*;

    #[test]
    fn new_and_seconds_roundtrip() {
        let d = Duration::new(1.25_f32);
        assert_eq!(d.seconds(), 1.25_f32);
        assert_eq!(d.into_seconds(), 1.25_f32);
    }

    #[test]
    fn from_and_into_conversions() {
        let d = Duration::from(2.5_f32);
        let s: Scalarf = d.into();
        assert_eq!(s, 2.5_f32);
    }

    #[test]
    fn try_from_seconds_accepts_finite_values() {
        assert_eq!(Duration::try_from_seconds(0.0_f32), Ok(Duration::new(0.0_f32)));
        assert_eq!(Duration::try_from_seconds(-3.0_f32), Ok(Duration::new(-3.0_f32)));
    }

    #[test]
    fn try_from_seconds_rejects_non_finite_values() {
        assert!(matches!(Duration::try_from_seconds(Scalarf::NAN), Err(Error::InvalidValue(_))));
        assert!(matches!(
            Duration::try_from_seconds(Scalarf::INFINITY),
            Err(Error::InvalidValue(_))
        ));
        assert!(matches!(
            Duration::try_from_seconds(Scalarf::NEG_INFINITY),
            Err(Error::InvalidValue(_))
        ));
    }

    #[test]
    #[should_panic(expected = "Duration cannot be created from NaN or infinite values")]
    fn from_seconds_panics_for_non_finite() {
        let _ = Duration::from_seconds(Scalarf::NAN);
    }
}

mod numeric {
    use rinia::numeric::{Cast, LossyCast, SaturatingCast, TryCast, TryExactCast};

    use super::*;

    #[test]
    fn zero_and_default_are_consistent() {
        assert_eq!(Duration::<Scalarf>::ZERO, Duration::new(0.0_f32));
        assert_eq!(Duration::<Scalarf>::default(), Duration::<Scalarf>::ZERO);
        assert_eq!(<Duration<Scalarf> as Zero>::ZERO, Duration::<Scalarf>::ZERO);
    }

    #[test]
    fn bounded_values_match_underlying_type() {
        assert_eq!(Duration::<Scalarf>::MIN.seconds(), Scalarf::MIN);
        assert_eq!(Duration::<Scalarf>::MAX.seconds(), Scalarf::MAX);
        assert_eq!(<Duration<Scalarf> as BoundedMin>::MIN, Duration::<Scalarf>::MIN);
        assert_eq!(<Duration<Scalarf> as BoundedMax>::MAX, Duration::<Scalarf>::MAX);
    }

    #[test]
    fn min_max_and_is_finite_behave_as_expected() {
        let a = Duration::new(1.0_f32);
        let b = Duration::new(2.0_f32);

        assert_eq!(a.min(b), a);
        assert_eq!(a.max(b), b);
        assert_eq!(a.minimum(b), a);
        assert_eq!(a.maximum(b), b);

        assert!(a.is_finite());
        assert!(!Duration::new(Scalarf::NEG_INFINITY).is_finite());
    }

    #[test]
    fn cast_variants_are_forwarded_to_inner_type() {
        let d_u8 = Duration::new(42_u8);

        let cast_u32: Duration<u32> = d_u8.cast();
        assert_eq!(cast_u32, Duration::new(42_u32));
        let cast_u32_trait: Duration<u32> = <Duration<u8> as Cast<Duration<u32>>>::cast(d_u8);
        assert_eq!(cast_u32_trait, Duration::new(42_u32));
        assert_eq!(Duration::<u32>::cast_from(d_u8), Duration::new(42_u32));

        let lossy_u8: Duration<u8> = d_u8.lossy_cast();
        assert_eq!(lossy_u8, Duration::new(42_u8));
        let lossy_u8_trait: Duration<u8> =
            <Duration<u8> as LossyCast<Duration<u8>>>::lossy_cast(d_u8);
        assert_eq!(lossy_u8_trait, Duration::new(42_u8));
        assert_eq!(Duration::<u8>::lossy_cast_from(d_u8), Duration::new(42_u8));

        let sat_u8: Duration<u8> = d_u8.saturating_cast();
        assert_eq!(sat_u8, Duration::new(42_u8));
        let sat_u8_trait: Duration<u8> =
            <Duration<u8> as SaturatingCast<Duration<u8>>>::saturating_cast(d_u8);
        assert_eq!(sat_u8_trait, Duration::new(42_u8));
        assert_eq!(Duration::<u8>::saturating_cast_from(d_u8), Duration::new(42_u8));

        let try_u32: Duration<u32> = d_u8.try_cast().expect("u8 to u32 should work");
        assert_eq!(try_u32, Duration::new(42_u32));
        let try_u32_trait: Duration<u32> = <Duration<u8> as TryCast<Duration<u32>>>::try_cast(d_u8)
            .expect("u8 to u32 should work");
        assert_eq!(try_u32_trait, Duration::new(42_u32));
        assert_eq!(
            Duration::<u32>::try_cast_from(d_u8).expect("u8 to u32 should work"),
            Duration::new(42_u32)
        );

        let exact_u32: Duration<u32> = d_u8.try_exact_cast().expect("exact cast should work");
        assert_eq!(exact_u32, Duration::new(42_u32));
        let exact_u32_trait: Duration<u32> =
            <Duration<u8> as TryExactCast<Duration<u32>>>::try_exact_cast(d_u8)
                .expect("exact cast should work");
        assert_eq!(exact_u32_trait, Duration::new(42_u32));
        assert_eq!(
            Duration::<u32>::try_exact_cast_from(d_u8).expect("exact cast should work"),
            Duration::new(42_u32)
        );
    }
}

mod ops {
    use super::*;

    #[test]
    fn add_sub_with_durations() {
        let a = Duration::new(10.0_f32);
        let b = Duration::new(2.5_f32);

        assert_eq!(a + b, Duration::new(12.5_f32));
        assert_eq!(a - b, Duration::new(7.5_f32));
    }

    #[test]
    fn add_sub_with_scalars() {
        let a = Duration::new(10.0_f32);

        assert_eq!(a + 1.5_f32, Duration::new(11.5_f32));
        assert_eq!(a - 3.0_f32, Duration::new(7.0_f32));
    }

    #[test]
    fn add_assign_and_sub_assign() {
        let mut d = Duration::new(3.0_f32);
        d += Duration::new(2.0_f32);
        assert_eq!(d, Duration::new(5.0_f32));

        d -= Duration::new(1.5_f32);
        assert_eq!(d, Duration::new(3.5_f32));

        d += 1.0_f32;
        assert_eq!(d, Duration::new(4.5_f32));

        d -= 0.5_f32;
        assert_eq!(d, Duration::new(4.0_f32));
    }

    #[test]
    fn multiply_divide_and_ratio() {
        let d = Duration::new(8.0_f32);

        assert_eq!(d / Duration::new(2.0_f32), 4.0_f32);
        assert_eq!(d / 4.0_f32, Duration::new(2.0_f32));
        assert_eq!(d * 1.5_f32, Duration::new(12.0_f32));
        assert_eq!(2.0_f32 * d, Duration::new(16.0_f32));
    }

    #[test]
    fn multiply_assign_and_divide_assign() {
        let mut d = Duration::new(9.0_f32);
        d *= 2.0_f32;
        assert_eq!(d, Duration::new(18.0_f32));

        d /= 3.0_f32;
        assert_eq!(d, Duration::new(6.0_f32));
    }
}

mod traits {
    use super::*;

    #[test]
    fn has_duration_for_duration_returns_self() {
        let d = Duration::new(3.25_f32);
        assert_eq!(HasDuration::duration(&d), d);
    }
}

mod approx {
    use rinia::{
        assert_approx_eq_abs, assert_approx_eq_rel, assert_approx_ne_abs, assert_approx_ne_rel,
    };

    use super::*;

    #[test]
    fn approx_eq_abs_is_forwarded_to_inner_seconds() {
        let a = Duration::new(1.0_f32);
        let b = Duration::new(1.0008_f32);

        assert_approx_eq_abs!(a, b, 0.001_f32);
        assert_approx_ne_abs!(a, b, 0.0001_f32);
    }

    #[test]
    fn approx_eq_rel_is_forwarded_to_inner_seconds() {
        let a = Duration::new(100.0_f32);
        let b = Duration::new(100.4_f32);

        assert_approx_eq_rel!(a, b, 0.005_f32);
        assert_approx_ne_rel!(a, b, 0.001_f32);
    }
}

mod compat {
    #[allow(unused_imports)]
    use super::*;

    #[cfg(feature = "bytemuck")]
    #[test]
    fn bytemuck_roundtrip() {
        let d = Duration::new(123_i32);
        let bytes = bytemuck::bytes_of(&d);
        let out = bytemuck::pod_read_unaligned::<Duration<i32>>(bytes);
        assert_eq!(out, d);
    }

    #[cfg(feature = "zerocopy")]
    #[test]
    fn zerocopy_roundtrip() {
        let d = Duration::new(45_u8);
        let bytes = <Duration<u8> as zerocopy::IntoBytes>::as_bytes(&d);
        let out = <Duration<u8> as zerocopy::FromBytes>::read_from_bytes(bytes)
            .expect("valid duration bytes");
        assert_eq!(out, d);
    }

    #[cfg(feature = "sakka")]
    #[test]
    fn sakka_roundtrip() {
        let d = Duration::new(789_i32);

        let mut writer = sakka::Writer::new(sakka::Endian::Little, ());
        <Duration<i32> as sakka::Encode>::encode(&d, &mut writer).expect("encode duration");
        let bytes = writer.finish();

        let mut reader = sakka::Reader::new(&bytes, sakka::Endian::Little, ());
        let out = <Duration<i32> as sakka::Decode>::decode(&mut reader).expect("decode duration");

        assert_eq!(out, d);
        assert!(reader.is_eof());
    }
}
