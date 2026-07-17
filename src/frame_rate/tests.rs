#![cfg(test)]

use rinia::{Scalard, Scalarf};

use crate::{Error, FrameRate};

mod construction {
    use super::*;

    #[test]
    fn new_preserves_raw_value() {
        let fps = FrameRate::new(24.0_f32);
        assert_eq!(fps, FrameRate::new(24.0_f32));
    }

    #[test]
    fn try_from_fps_accepts_positive_finite_values() {
        assert_eq!(FrameRate::try_from_fps(60.0_f32), Ok(FrameRate::new(60.0_f32)));
        assert_eq!(FrameRate::try_from_fps(0.001_f32), Ok(FrameRate::new(0.001_f32)));
    }

    #[test]
    fn try_from_fps_rejects_non_finite_values() {
        assert!(matches!(FrameRate::try_from_fps(Scalarf::NAN), Err(Error::InvalidValue(_))));
        assert!(matches!(FrameRate::try_from_fps(Scalarf::INFINITY), Err(Error::InvalidValue(_))));
        assert!(matches!(
            FrameRate::try_from_fps(Scalarf::NEG_INFINITY),
            Err(Error::InvalidValue(_))
        ));
    }

    #[test]
    fn try_from_fps_rejects_non_positive_values() {
        assert!(matches!(FrameRate::try_from_fps(0.0_f32), Err(Error::InvalidValue(_))));
        assert!(matches!(FrameRate::try_from_fps(-24.0_f32), Err(Error::InvalidValue(_))));
    }

    #[test]
    #[should_panic(expected = "FrameRate cannot be created from non-positive values")]
    fn from_fps_panics_for_non_positive() {
        let _ = FrameRate::from_fps(0.0_f32);
    }

    #[test]
    #[should_panic(expected = "FrameRate cannot be created from NaN or infinite values")]
    fn from_fps_panics_for_non_finite() {
        let _ = FrameRate::from_fps(Scalarf::NAN);
    }

    #[test]
    fn default_is_60_fps() {
        assert_eq!(FrameRate::<Scalarf>::default(), FrameRate::new(60.0_f32));
    }

    #[test]
    fn generated_fps_constructors_return_expected_values() {
        assert_eq!(FrameRate::fps_24(), FrameRate::new(24.0_f32));
        assert_eq!(FrameRate::fps_25(), FrameRate::new(25.0_f32));
        assert_eq!(FrameRate::fps_30(), FrameRate::new(30.0_f32));
        assert_eq!(FrameRate::fps_48(), FrameRate::new(48.0_f32));
        assert_eq!(FrameRate::fps_50(), FrameRate::new(50.0_f32));
        assert_eq!(FrameRate::fps_60(), FrameRate::new(60.0_f32));
        assert_eq!(FrameRate::fps_120(), FrameRate::new(120.0_f32));
        assert_eq!(FrameRate::fps_144(), FrameRate::new(144.0_f32));
        assert_eq!(FrameRate::fps_240(), FrameRate::new(240.0_f32));
    }

    #[test]
    fn from_scalar_and_into_scalar_conversions_roundtrip() {
        let fps_f32 = FrameRate::from(24.0_f32);
        let raw_f32: Scalarf = fps_f32.into();
        assert_eq!(raw_f32, 24.0_f32);

        let fps_f64 = FrameRate::from(120.0_f64);
        let raw_f64: Scalard = fps_f64.into();
        assert_eq!(raw_f64, 120.0_f64);
    }
}

mod numeric {
    use rinia::numeric::{Cast, LossyCast, SaturatingCast, TryCast, TryExactCast};

    use super::*;

    #[test]
    fn cast_variants_are_forwarded_to_inner_type() {
        let r_u8 = FrameRate::new(42_u8);

        let cast_u32: FrameRate<u32> = r_u8.cast();
        assert_eq!(cast_u32, FrameRate::new(42_u32));
        let cast_u32_trait: FrameRate<u32> = <FrameRate<u8> as Cast<FrameRate<u32>>>::cast(r_u8);
        assert_eq!(cast_u32_trait, FrameRate::new(42_u32));
        assert_eq!(FrameRate::<u32>::cast_from(r_u8), FrameRate::new(42_u32));

        let lossy_u8: FrameRate<u8> = r_u8.lossy_cast();
        assert_eq!(lossy_u8, FrameRate::new(42_u8));
        let lossy_u8_trait: FrameRate<u8> =
            <FrameRate<u8> as LossyCast<FrameRate<u8>>>::lossy_cast(r_u8);
        assert_eq!(lossy_u8_trait, FrameRate::new(42_u8));
        assert_eq!(FrameRate::<u8>::lossy_cast_from(r_u8), FrameRate::new(42_u8));

        let sat_u8: FrameRate<u8> = r_u8.saturating_cast();
        assert_eq!(sat_u8, FrameRate::new(42_u8));
        let sat_u8_trait: FrameRate<u8> =
            <FrameRate<u8> as SaturatingCast<FrameRate<u8>>>::saturating_cast(r_u8);
        assert_eq!(sat_u8_trait, FrameRate::new(42_u8));
        assert_eq!(FrameRate::<u8>::saturating_cast_from(r_u8), FrameRate::new(42_u8));

        let try_u32: FrameRate<u32> = r_u8.try_cast().expect("u8 to u32 should work");
        assert_eq!(try_u32, FrameRate::new(42_u32));
        let try_u32_trait: FrameRate<u32> =
            <FrameRate<u8> as TryCast<FrameRate<u32>>>::try_cast(r_u8)
                .expect("u8 to u32 should work");
        assert_eq!(try_u32_trait, FrameRate::new(42_u32));
        assert_eq!(
            FrameRate::<u32>::try_cast_from(r_u8).expect("u8 to u32 should work"),
            FrameRate::new(42_u32)
        );

        let exact_u32: FrameRate<u32> = r_u8.try_exact_cast().expect("exact cast should work");
        assert_eq!(exact_u32, FrameRate::new(42_u32));
        let exact_u32_trait: FrameRate<u32> =
            <FrameRate<u8> as TryExactCast<FrameRate<u32>>>::try_exact_cast(r_u8)
                .expect("exact cast should work");
        assert_eq!(exact_u32_trait, FrameRate::new(42_u32));
        assert_eq!(
            FrameRate::<u32>::try_exact_cast_from(r_u8).expect("exact cast should work"),
            FrameRate::new(42_u32)
        );
    }
}

mod ops {
    use super::*;

    #[test]
    fn frame_rate_divided_by_frame_rate_returns_ratio() {
        let a = FrameRate::new(120.0_f32);
        let b = FrameRate::new(24.0_f32);
        let op_result = a / b;
        let method_result = a.ratio(b);
        assert_eq!(op_result, 5.0_f32);
        assert_eq!(method_result, 5.0_f32);
    }

    #[test]
    fn frame_rate_mul_scalar_returns_scaled_frame_rate() {
        let fps = FrameRate::new(24.0_f32);
        assert_eq!(fps * 2.0_f32, FrameRate::new(48.0_f32));
    }

    #[test]
    fn frame_rate_div_scalar_returns_scaled_frame_rate() {
        let fps = FrameRate::new(120.0_f32);
        assert_eq!(fps / 4.0_f32, FrameRate::new(30.0_f32));
    }

    #[test]
    fn mul_assign_scales_frame_rate_in_place() {
        let mut fps = FrameRate::new(30.0_f32);
        fps *= 2.0_f32;
        assert_eq!(fps, FrameRate::new(60.0_f32));
    }

    #[test]
    fn div_assign_scales_frame_rate_in_place() {
        let mut fps = FrameRate::new(120.0_f32);
        fps /= 2.0_f32;
        assert_eq!(fps, FrameRate::new(60.0_f32));
    }
}

mod approx {
    use rinia::{
        assert_approx_eq_abs, assert_approx_eq_rel, assert_approx_ne_abs, assert_approx_ne_rel,
    };

    use super::*;

    #[test]
    fn approx_eq_abs_is_forwarded_to_inner_fps() {
        let a = FrameRate::new(60.0_f32);
        let b = FrameRate::new(60.03_f32);

        assert_approx_eq_abs!(a, b, 0.05_f32);
        assert_approx_ne_abs!(a, b, 0.01_f32);
    }

    #[test]
    fn approx_eq_rel_is_forwarded_to_inner_fps() {
        let a = FrameRate::new(60.0_f32);
        let b = FrameRate::new(60.3_f32);

        assert_approx_eq_rel!(a, b, 0.01_f32);
        assert_approx_ne_rel!(a, b, 0.001_f32);
    }
}

mod compat {
    #[allow(unused_imports)]
    use super::*;

    #[cfg(feature = "bytemuck")]
    #[test]
    fn bytemuck_roundtrip() {
        let fps = FrameRate::new(123_i32);
        let bytes = bytemuck::bytes_of(&fps);
        let out = bytemuck::pod_read_unaligned::<FrameRate<i32>>(bytes);
        assert_eq!(out, fps);
    }

    #[cfg(feature = "zerocopy")]
    #[test]
    fn zerocopy_roundtrip() {
        let fps = FrameRate::new(45_u8);
        let bytes = <FrameRate<u8> as zerocopy::IntoBytes>::as_bytes(&fps);
        let out = <FrameRate<u8> as zerocopy::FromBytes>::read_from_bytes(bytes)
            .expect("valid frame rate bytes");
        assert_eq!(out, fps);
    }

    #[cfg(feature = "sakka")]
    #[test]
    fn sakka_roundtrip() {
        let fps = FrameRate::new(789_i32);

        let mut writer = sakka::Writer::new(sakka::Endian::Little, ());
        <FrameRate<i32> as sakka::Encode>::encode(&fps, &mut writer).expect("encode frame rate");
        let bytes = writer.finish();

        let mut reader = sakka::Reader::new(&bytes, sakka::Endian::Little, ());
        let out =
            <FrameRate<i32> as sakka::Decode>::decode(&mut reader).expect("decode frame rate");

        assert_eq!(out, fps);
        assert!(reader.is_eof());
    }
}
