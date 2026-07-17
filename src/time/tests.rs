#![cfg(test)]

use rinia::{
    Scalarf,
    numeric::{BoundedMax, BoundedMin, MinMax, Zero},
};

use crate::{Duration, Error, FrameIndex, FrameRate, Time};

mod construction {
    use super::*;

    #[test]
    fn new_and_seconds_roundtrip() {
        let t = Time::new(1.25_f32);
        assert_eq!(t.seconds(), 1.25_f32);
        assert_eq!(t.into_seconds(), 1.25_f32);
    }

    #[test]
    fn from_and_into_conversions() {
        let t = Time::from(2.5_f32);
        let s: Scalarf = t.into();
        assert_eq!(s, 2.5_f32);
    }

    #[test]
    fn time_and_frame_conversions_match_frame_rate() {
        let rate = FrameRate::new(24.0_f32);
        let t = Time::new(1.75_f32);

        let frame: FrameIndex<f32> = t.to_frame(rate);
        assert_eq!(frame, FrameIndex::new(42.0_f32));

        let t_from_frame = Time::<f32>::from_frame(frame, rate);
        assert_eq!(t_from_frame, Time::new(1.75_f32));
    }

    #[test]
    fn try_from_seconds_accepts_finite_values() {
        assert_eq!(Time::try_from_seconds(0.0_f32), Ok(Time::new(0.0_f32)));
        assert_eq!(Time::try_from_seconds(-3.0_f32), Ok(Time::new(-3.0_f32)));
    }

    #[test]
    fn try_from_seconds_rejects_non_finite_values() {
        assert!(matches!(Time::try_from_seconds(Scalarf::NAN), Err(Error::InvalidValue(_))));
        assert!(matches!(Time::try_from_seconds(Scalarf::INFINITY), Err(Error::InvalidValue(_))));
        assert!(matches!(
            Time::try_from_seconds(Scalarf::NEG_INFINITY),
            Err(Error::InvalidValue(_))
        ));
    }

    #[test]
    #[should_panic(expected = "Time cannot be created from NaN or infinite values")]
    fn from_seconds_panics_for_non_finite() {
        let _ = Time::from_seconds(Scalarf::NAN);
    }
}

mod numeric {
    use rinia::numeric::{Cast, LossyCast, SaturatingCast, TryCast, TryExactCast};

    use super::*;

    #[test]
    fn zero_and_default_are_consistent() {
        assert_eq!(Time::<Scalarf>::ZERO, Time::new(0.0_f32));
        assert_eq!(Time::<Scalarf>::default(), Time::<Scalarf>::ZERO);
    }

    #[test]
    fn bounded_values_match_underlying_type() {
        assert_eq!(Time::<Scalarf>::MIN.seconds(), Scalarf::MIN);
        assert_eq!(Time::<Scalarf>::MAX.seconds(), Scalarf::MAX);
        assert_eq!(<Time<Scalarf> as BoundedMin>::MIN, Time::<Scalarf>::MIN);
        assert_eq!(<Time<Scalarf> as BoundedMax>::MAX, Time::<Scalarf>::MAX);
    }

    #[test]
    fn min_max_and_is_finite_behave_as_expected() {
        let a = Time::new(1.0_f32);
        let b = Time::new(2.0_f32);

        assert_eq!(a.min(b), a);
        assert_eq!(a.max(b), b);
        assert_eq!(a.minimum(b), a);
        assert_eq!(a.maximum(b), b);

        assert!(a.is_finite());
        assert!(!Time::new(Scalarf::INFINITY).is_finite());
    }

    #[test]
    fn zero_trait_matches_inherent_zero() {
        assert_eq!(<Time<Scalarf> as Zero>::ZERO, Time::<Scalarf>::ZERO);
    }

    #[test]
    fn cast_variants_are_forwarded_to_inner_type() {
        let t_u8 = Time::new(42_u8);

        let cast_u32: Time<u32> = t_u8.cast();
        assert_eq!(cast_u32, Time::new(42_u32));
        let cast_u32_trait: Time<u32> = <Time<u8> as Cast<Time<u32>>>::cast(t_u8);
        assert_eq!(cast_u32_trait, Time::new(42_u32));
        assert_eq!(Time::<u32>::cast_from(t_u8), Time::new(42_u32));

        let lossy_u8: Time<u8> = t_u8.lossy_cast();
        assert_eq!(lossy_u8, Time::new(42_u8));
        let lossy_u8_trait: Time<u8> = <Time<u8> as LossyCast<Time<u8>>>::lossy_cast(t_u8);
        assert_eq!(lossy_u8_trait, Time::new(42_u8));
        assert_eq!(Time::<u8>::lossy_cast_from(t_u8), Time::new(42_u8));

        let sat_u8: Time<u8> = t_u8.saturating_cast();
        assert_eq!(sat_u8, Time::new(42_u8));
        let sat_u8_trait: Time<u8> = <Time<u8> as SaturatingCast<Time<u8>>>::saturating_cast(t_u8);
        assert_eq!(sat_u8_trait, Time::new(42_u8));
        assert_eq!(Time::<u8>::saturating_cast_from(t_u8), Time::new(42_u8));

        let try_u32: Time<u32> = t_u8.try_cast().expect("u8 to u32 should work");
        assert_eq!(try_u32, Time::new(42_u32));
        let try_u32_trait: Time<u32> =
            <Time<u8> as TryCast<Time<u32>>>::try_cast(t_u8).expect("u8 to u32 should work");
        assert_eq!(try_u32_trait, Time::new(42_u32));
        assert_eq!(
            Time::<u32>::try_cast_from(t_u8).expect("u8 to u32 should work"),
            Time::new(42_u32)
        );

        let exact_u32: Time<u32> = t_u8.try_exact_cast().expect("exact cast should work");
        assert_eq!(exact_u32, Time::new(42_u32));
        let exact_u32_trait: Time<u32> =
            <Time<u8> as TryExactCast<Time<u32>>>::try_exact_cast(t_u8)
                .expect("exact cast should work");
        assert_eq!(exact_u32_trait, Time::new(42_u32));
        assert_eq!(
            Time::<u32>::try_exact_cast_from(t_u8).expect("exact cast should work"),
            Time::new(42_u32)
        );
    }
}

mod ops {
    use super::*;

    #[test]
    fn subtracting_two_times_yields_duration() {
        let dt = Time::new(5.0_f32) - Time::new(2.0_f32);
        assert_eq!(dt, Duration::new(3.0_f32));
    }

    #[test]
    fn add_and_sub_duration() {
        let t0 = Time::new(10.0_f32);
        let d = Duration::new(3.5_f32);

        assert_eq!(t0 + d, Time::new(13.5_f32));
        assert_eq!(t0 - d, Time::new(6.5_f32));
    }

    #[test]
    fn add_assign_and_sub_assign_duration() {
        let mut t = Time::new(4.0_f32);
        t += Duration::new(1.5_f32);
        assert_eq!(t, Time::new(5.5_f32));

        t -= Duration::new(2.0_f32);
        assert_eq!(t, Time::new(3.5_f32));
    }

    #[test]
    fn subtract_scalar_and_sub_assign_scalar() {
        let mut t = Time::new(7.0_f32);
        assert_eq!(t - 2.5_f32, Time::new(4.5_f32));

        t -= 1.0_f32;
        assert_eq!(t, Time::new(6.0_f32));
    }
}

mod approx {
    use rinia::{
        assert_approx_eq_abs_tol, assert_approx_eq_rel_tol, assert_approx_ne_abs_tol,
        assert_approx_ne_rel_tol,
    };

    use super::*;

    #[test]
    fn approx_eq_abs_is_forwarded_to_inner_seconds() {
        let a = Time::new(1.0_f32);
        let b = Time::new(1.0008_f32);

        assert_approx_eq_abs_tol!(a, b, 0.001_f32);
        assert_approx_ne_abs_tol!(a, b, 0.0001_f32);
    }

    #[test]
    fn approx_eq_rel_is_forwarded_to_inner_seconds() {
        let a = Time::new(100.0_f32);
        let b = Time::new(100.4_f32);

        assert_approx_eq_rel_tol!(a, b, 0.005_f32);
        assert_approx_ne_rel_tol!(a, b, 0.001_f32);
    }
}

mod compat {
    #[allow(unused_imports)]
    use super::*;

    #[cfg(feature = "bytemuck")]
    #[test]
    fn bytemuck_roundtrip() {
        let t = Time::new(123_i32);
        let bytes = bytemuck::bytes_of(&t);
        let out = bytemuck::pod_read_unaligned::<Time<i32>>(bytes);
        assert_eq!(out, t);
    }

    #[cfg(feature = "zerocopy")]
    #[test]
    fn zerocopy_roundtrip() {
        let t = Time::new(45_u8);
        let bytes = <Time<u8> as zerocopy::IntoBytes>::as_bytes(&t);
        let out =
            <Time<u8> as zerocopy::FromBytes>::read_from_bytes(bytes).expect("valid time bytes");
        assert_eq!(out, t);
    }

    #[cfg(feature = "sakka")]
    #[test]
    fn sakka_roundtrip() {
        let t = Time::new(789_i32);

        let mut writer = sakka::Writer::new(sakka::Endian::Little, ());
        <Time<i32> as sakka::Encode>::encode(&t, &mut writer).expect("encode time");
        let bytes = writer.finish();

        let mut reader = sakka::Reader::new(&bytes, sakka::Endian::Little, ());
        let out = <Time<i32> as sakka::Decode>::decode(&mut reader).expect("decode time");

        assert_eq!(out, t);
        assert!(reader.is_eof());
    }
}
