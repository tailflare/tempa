#![cfg(test)]

use crate::{FrameIndex, FrameRate, Time};

mod construction {
    use super::*;

    #[test]
    fn new_preserves_raw_value() {
        let frame = FrameIndex::new(42_u32);
        assert_eq!(frame.index(), 42);
        assert_eq!(frame.into_index(), 42);
    }

    #[test]
    fn from_and_into_conversions() {
        let frame = FrameIndex::from(24_u32);
        let raw: u32 = frame.into();
        assert_eq!(raw, 24);
    }

    #[test]
    fn frame_and_time_conversions_match_frame_rate() {
        let rate = FrameRate::new(24.0_f32);
        let frame = FrameIndex::<u32>::from_time(Time::new(1.75_f32), rate);
        assert_eq!(frame, FrameIndex::new(42_u32));

        let t = frame.to_time(rate);
        assert_eq!(t, Time::new(1.75_f32));
    }
}

mod numeric {
    use rinia::numeric::{
        BoundedMax, BoundedMin, Cast, LossyCast, MinMax, SaturatingCast, TryCast, TryExactCast,
        Zero,
    };

    use super::*;

    #[test]
    fn zero_and_default_are_consistent() {
        assert_eq!(FrameIndex::<u32>::ZERO, FrameIndex::new(0_u32));
        assert_eq!(FrameIndex::<u32>::default(), FrameIndex::<u32>::ZERO);
        assert_eq!(<FrameIndex<u32> as Zero>::ZERO, FrameIndex::<u32>::ZERO);
    }

    #[test]
    fn bounded_values_match_underlying_type() {
        assert_eq!(FrameIndex::<u32>::MIN.index(), u32::MIN);
        assert_eq!(FrameIndex::<u32>::MAX.index(), u32::MAX);
        assert_eq!(<FrameIndex<u32> as BoundedMin>::MIN, FrameIndex::<u32>::MIN);
        assert_eq!(<FrameIndex<u32> as BoundedMax>::MAX, FrameIndex::<u32>::MAX);
    }

    #[test]
    fn min_max_and_minimum_maximum_behave_as_expected() {
        let a = FrameIndex::new(1_u32);
        let b = FrameIndex::new(2_u32);

        assert_eq!(a.min(b), a);
        assert_eq!(a.max(b), b);
        assert_eq!(a.minimum(b), a);
        assert_eq!(a.maximum(b), b);
    }

    #[test]
    fn cast_variants_are_forwarded_to_inner_type() {
        let f_u8 = FrameIndex::new(42_u8);

        let cast_u32: FrameIndex<u32> = f_u8.cast();
        assert_eq!(cast_u32, FrameIndex::new(42_u32));
        let cast_u32_trait: FrameIndex<u32> = <FrameIndex<u8> as Cast<FrameIndex<u32>>>::cast(f_u8);
        assert_eq!(cast_u32_trait, FrameIndex::new(42_u32));
        assert_eq!(FrameIndex::<u32>::cast_from(f_u8), FrameIndex::new(42_u32));

        let lossy_u8: FrameIndex<u8> = f_u8.lossy_cast();
        assert_eq!(lossy_u8, FrameIndex::new(42_u8));
        let lossy_u8_trait: FrameIndex<u8> =
            <FrameIndex<u8> as LossyCast<FrameIndex<u8>>>::lossy_cast(f_u8);
        assert_eq!(lossy_u8_trait, FrameIndex::new(42_u8));
        assert_eq!(FrameIndex::<u8>::lossy_cast_from(f_u8), FrameIndex::new(42_u8));

        let sat_u8: FrameIndex<u8> = f_u8.saturating_cast();
        assert_eq!(sat_u8, FrameIndex::new(42_u8));
        let sat_u8_trait: FrameIndex<u8> =
            <FrameIndex<u8> as SaturatingCast<FrameIndex<u8>>>::saturating_cast(f_u8);
        assert_eq!(sat_u8_trait, FrameIndex::new(42_u8));
        assert_eq!(FrameIndex::<u8>::saturating_cast_from(f_u8), FrameIndex::new(42_u8));

        let try_u32: FrameIndex<u32> = f_u8.try_cast().expect("u8 to u32 should work");
        assert_eq!(try_u32, FrameIndex::new(42_u32));
        let try_u32_trait: FrameIndex<u32> =
            <FrameIndex<u8> as TryCast<FrameIndex<u32>>>::try_cast(f_u8)
                .expect("u8 to u32 should work");
        assert_eq!(try_u32_trait, FrameIndex::new(42_u32));
        assert_eq!(
            FrameIndex::<u32>::try_cast_from(f_u8).expect("u8 to u32 should work"),
            FrameIndex::new(42_u32)
        );

        let exact_u32: FrameIndex<u32> = f_u8.try_exact_cast().expect("exact cast should work");
        assert_eq!(exact_u32, FrameIndex::new(42_u32));
        let exact_u32_trait: FrameIndex<u32> =
            <FrameIndex<u8> as TryExactCast<FrameIndex<u32>>>::try_exact_cast(f_u8)
                .expect("exact cast should work");
        assert_eq!(exact_u32_trait, FrameIndex::new(42_u32));
        assert_eq!(
            FrameIndex::<u32>::try_exact_cast_from(f_u8).expect("exact cast should work"),
            FrameIndex::new(42_u32)
        );
    }
}

mod ops {
    use rinia::numeric::{SaturatingAdd, SaturatingSub};

    use super::*;

    #[test]
    fn add_sub_with_frames() {
        let a = FrameIndex::new(10_u32);
        let b = FrameIndex::new(3_u32);

        assert_eq!(a + b, FrameIndex::new(13_u32));
        assert_eq!(a - b, FrameIndex::new(7_u32));
    }

    #[test]
    fn add_sub_with_scalars() {
        let a = FrameIndex::new(10_u32);

        assert_eq!(a + 5_u32, FrameIndex::new(15_u32));
        assert_eq!(a - 2_u32, FrameIndex::new(8_u32));
    }

    #[test]
    fn add_assign_and_sub_assign_with_frames_and_scalars() {
        let mut frame = FrameIndex::new(10_u32);
        frame += FrameIndex::new(5_u32);
        assert_eq!(frame, FrameIndex::new(15_u32));

        frame -= FrameIndex::new(2_u32);
        assert_eq!(frame, FrameIndex::new(13_u32));

        frame += 7_u32;
        assert_eq!(frame, FrameIndex::new(20_u32));

        frame -= 4_u32;
        assert_eq!(frame, FrameIndex::new(16_u32));
    }

    #[test]
    fn saturating_add_and_sub_work_for_frames_and_scalars() {
        let max = FrameIndex::new(u32::MAX);
        assert_eq!(
            FrameIndex::saturating_add(max, FrameIndex::new(1_u32)),
            FrameIndex::new(u32::MAX)
        );
        assert_eq!(
            <FrameIndex<u32> as SaturatingAdd>::saturating_add(max, FrameIndex::new(1_u32)),
            FrameIndex::new(u32::MAX)
        );
        assert_eq!(FrameIndex::saturating_add_t(max, 1_u32), FrameIndex::new(u32::MAX));
        assert_eq!(
            <FrameIndex<u32> as SaturatingAdd<u32>>::saturating_add(max, 1_u32),
            FrameIndex::new(u32::MAX)
        );

        let zero = FrameIndex::new(0_u32);
        assert_eq!(
            FrameIndex::saturating_sub(zero, FrameIndex::new(1_u32)),
            FrameIndex::new(0_u32)
        );
        assert_eq!(
            <FrameIndex<u32> as SaturatingSub>::saturating_sub(zero, FrameIndex::new(1_u32)),
            FrameIndex::new(0_u32)
        );
        assert_eq!(FrameIndex::saturating_sub_t(zero, 1_u32), FrameIndex::new(0_u32));
        assert_eq!(
            <FrameIndex<u32> as SaturatingSub<u32>>::saturating_sub(zero, 1_u32),
            FrameIndex::new(0_u32)
        );
    }

    #[test]
    fn offset_next_and_previous_work() {
        let a = FrameIndex::new(10_u32);
        let b = FrameIndex::new(7_u32);

        assert_eq!(a.offset_from(b), 3_i32);
        assert_eq!(b.offset_from(a), -3_i32);

        assert_eq!(a.next(), FrameIndex::new(11_u32));
        assert_eq!(FrameIndex::new(u32::MAX).next(), FrameIndex::new(u32::MAX));

        assert_eq!(a.previous(), FrameIndex::new(9_u32));
        assert_eq!(FrameIndex::new(0_u32).previous(), FrameIndex::new(0_u32));
    }
}

mod approx {
    use rinia::{
        assert_approx_eq_abs_tol, assert_approx_eq_rel_tol, assert_approx_ne_abs_tol,
        assert_approx_ne_rel_tol,
    };

    use super::*;

    #[test]
    fn approx_eq_abs_and_rel_forward_to_inner_value() {
        let a = FrameIndex::new(1.0_f32);
        let b = FrameIndex::new(1.0008_f32);

        assert_approx_eq_abs_tol!(a, b, 0.001_f32);
        assert_approx_ne_abs_tol!(a, b, 0.0001_f32);

        let c = FrameIndex::new(100.0_f32);
        let d = FrameIndex::new(100.4_f32);

        assert_approx_eq_rel_tol!(c, d, 0.01_f32);
        assert_approx_ne_rel_tol!(c, d, 0.001_f32);
    }
}

mod compat {
    #[allow(unused_imports)]
    use super::*;

    #[cfg(feature = "bytemuck")]
    #[test]
    fn bytemuck_roundtrip() {
        let frame = FrameIndex::new(123_i32);
        let bytes = bytemuck::bytes_of(&frame);
        let out = bytemuck::pod_read_unaligned::<FrameIndex<i32>>(bytes);
        assert_eq!(out, frame);
    }

    #[cfg(feature = "zerocopy")]
    #[test]
    fn zerocopy_roundtrip() {
        let frame = FrameIndex::new(45_u8);
        let bytes = <FrameIndex<u8> as zerocopy::IntoBytes>::as_bytes(&frame);
        let out = <FrameIndex<u8> as zerocopy::FromBytes>::read_from_bytes(bytes)
            .expect("valid frame index bytes");
        assert_eq!(out, frame);
    }

    #[cfg(feature = "sakka")]
    #[test]
    fn sakka_roundtrip() {
        let frame = FrameIndex::new(789_i32);

        let mut writer = sakka::Writer::new(sakka::Endian::Little, ());
        <FrameIndex<i32> as sakka::Encode>::encode(&frame, &mut writer)
            .expect("encode frame index");
        let bytes = writer.finish();

        let mut reader = sakka::Reader::new(&bytes, sakka::Endian::Little, ());
        let out =
            <FrameIndex<i32> as sakka::Decode>::decode(&mut reader).expect("decode frame index");

        assert_eq!(out, frame);
        assert!(reader.is_eof());
    }
}
