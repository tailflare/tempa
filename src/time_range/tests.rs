#![cfg(test)]

use rinia::Scalarf;

use crate::{Duration, Error, HasDuration, HasTimeRange, Time, TimeRange};

mod construction {
    use super::*;

    #[test]
    fn new_preserves_start_and_end() {
        let r = TimeRange::new(Time::new(1.0_f32), Time::new(3.0_f32));
        assert_eq!(r.start(), Time::new(1.0_f32));
        assert_eq!(r.end(), Time::new(3.0_f32));
    }

    #[test]
    fn try_new_accepts_finite_non_descending_values() {
        assert_eq!(
            TimeRange::try_new(Time::new(1.0_f32), Time::new(3.0_f32)),
            Ok(TimeRange::new(Time::new(1.0_f32), Time::new(3.0_f32)))
        );
        assert_eq!(
            TimeRange::try_new(Time::new(2.0_f32), Time::new(2.0_f32)),
            Ok(TimeRange::new(Time::new(2.0_f32), Time::new(2.0_f32)))
        );
    }

    #[test]
    fn try_new_rejects_non_finite_values() {
        assert!(matches!(
            TimeRange::try_new(Time::new(Scalarf::NAN), Time::new(1.0_f32)),
            Err(Error::InvalidValue(_))
        ));
        assert!(matches!(
            TimeRange::try_new(Time::new(0.0_f32), Time::new(Scalarf::INFINITY)),
            Err(Error::InvalidValue(_))
        ));
    }

    #[test]
    fn try_new_rejects_descending_ranges() {
        assert!(matches!(
            TimeRange::try_new(Time::new(5.0_f32), Time::new(4.0_f32)),
            Err(Error::InvalidValue(_))
        ));
    }

    #[test]
    #[should_panic(expected = "TimeRange cannot be created with start greater than end")]
    fn from_times_panics_for_descending_ranges() {
        let _ = TimeRange::from_times(Time::new(2.0_f32), Time::new(1.0_f32));
    }

    #[test]
    fn from_range_of_times_preserves_bounds() {
        let ops_range: core::ops::Range<Time<Scalarf>> = Time::new(1.5_f32)..Time::new(3.5_f32);
        let r_from_ops = TimeRange::from(ops_range);
        assert_eq!(r_from_ops, TimeRange::new(Time::new(1.5_f32), Time::new(3.5_f32)));

        let core_range: core::range::Range<Time<Scalarf>> =
            (Time::new(2.0_f32)..Time::new(4.0_f32)).into();
        let r_from_core = TimeRange::from(core_range);
        assert_eq!(r_from_core, TimeRange::new(Time::new(2.0_f32), Time::new(4.0_f32)));
    }

    #[test]
    fn from_range_of_scalars_wraps_values_as_time() {
        let ops_range: core::ops::Range<Scalarf> = 10.0_f32..20.0_f32;
        let r_from_ops = TimeRange::from(ops_range);
        assert_eq!(r_from_ops, TimeRange::new(Time::new(10.0_f32), Time::new(20.0_f32)));

        let core_range: core::range::Range<Scalarf> = (30.0_f32..40.0_f32).into();
        let r_from_core = TimeRange::from(core_range);
        assert_eq!(r_from_core, TimeRange::new(Time::new(30.0_f32), Time::new(40.0_f32)));
    }
}

mod accessors_and_ops {
    use super::*;

    #[test]
    fn into_parts_roundtrip() {
        let r = TimeRange::new(Time::new(3.0_f32), Time::new(8.0_f32));
        let (start, end) = r.into_parts();
        assert_eq!(start, Time::new(3.0_f32));
        assert_eq!(end, Time::new(8.0_f32));
    }

    #[test]
    fn duration_is_end_minus_start() {
        let r = TimeRange::new(Time::new(2.0_f32), Time::new(6.5_f32));
        assert_eq!(r.duration(), Duration::new(4.5_f32));
    }

    #[test]
    fn clamp_time_limits_to_bounds() {
        let r = TimeRange::new(Time::new(10.0_f32), Time::new(20.0_f32));

        assert_eq!(r.clamp_time(Time::new(5.0_f32)), Time::new(10.0_f32));
        assert_eq!(r.clamp_time(Time::new(15.0_f32)), Time::new(15.0_f32));
        assert_eq!(r.clamp_time(Time::new(25.0_f32)), Time::new(20.0_f32));
    }

    #[test]
    fn normalize_time_returns_unclamped_factor() {
        let r = TimeRange::new(Time::new(10.0_f32), Time::new(20.0_f32));

        assert_eq!(r.normalize_time(Time::new(10.0_f32)), 0.0_f32);
        assert_eq!(r.normalize_time(Time::new(15.0_f32)), 0.5_f32);
        assert_eq!(r.normalize_time(Time::new(20.0_f32)), 1.0_f32);
        assert_eq!(r.normalize_time(Time::new(25.0_f32)), 1.5_f32);
        assert_eq!(r.normalize_time(Time::new(5.0_f32)), -0.5_f32);
    }

    #[test]
    fn try_normalize_time_handles_degenerate_ranges() {
        let non_degenerate = TimeRange::new(Time::new(10.0_f32), Time::new(20.0_f32));
        assert_eq!(non_degenerate.try_normalize_time(Time::new(15.0_f32)), Some(0.5_f32));

        let degenerate = TimeRange::new(Time::new(3.0_f32), Time::new(3.0_f32));
        assert_eq!(degenerate.try_normalize_time(Time::new(3.0_f32)), None);
        assert_eq!(degenerate.try_normalize_time(Time::new(5.0_f32)), None);
    }

    #[test]
    fn default_is_zero_to_zero() {
        let r = TimeRange::<Scalarf>::default();
        assert_eq!(r, TimeRange::new(Time::new(0.0_f32), Time::new(0.0_f32)));
    }
}

mod advanced_ops {
    use super::*;

    #[test]
    fn center_shift_and_duration_range_behave_as_expected() {
        let r = TimeRange::new(Time::new(2.0_f32), Time::new(6.0_f32));

        assert_eq!(r.center(), Time::new(4.0_f32));

        let shifted = r.shift(Duration::new(1.5_f32));
        assert_eq!(shifted, TimeRange::new(Time::new(3.5_f32), Time::new(7.5_f32)));

        let duration_range = r.to_duration_range();
        assert_eq!(duration_range, TimeRange::new(Time::new(0.0_f32), Time::new(4.0_f32)));
    }

    #[test]
    fn scale_and_scale_from_start_clamp_negative_factor() {
        let r = TimeRange::new(Time::new(10.0_f32), Time::new(14.0_f32));

        assert_eq!(r.scale(2.0_f32), TimeRange::new(Time::new(8.0_f32), Time::new(16.0_f32)));
        assert_eq!(r.scale(-2.0_f32), TimeRange::new(Time::new(12.0_f32), Time::new(12.0_f32)));

        assert_eq!(
            r.scale_from_start(1.5_f32),
            TimeRange::new(Time::new(10.0_f32), Time::new(16.0_f32))
        );
        assert_eq!(
            r.scale_from_start(-3.0_f32),
            TimeRange::new(Time::new(10.0_f32), Time::new(10.0_f32))
        );
    }

    #[test]
    fn expand_and_pad_helpers_adjust_bounds() {
        let r = TimeRange::new(Time::new(10.0_f32), Time::new(20.0_f32));

        assert_eq!(
            r.expand(Duration::new(2.0_f32)),
            TimeRange::new(Time::new(8.0_f32), Time::new(22.0_f32))
        );
        assert_eq!(
            r.pad_start(Duration::new(3.0_f32)),
            TimeRange::new(Time::new(7.0_f32), Time::new(20.0_f32))
        );
        assert_eq!(
            r.pad_end(Duration::new(4.0_f32)),
            TimeRange::new(Time::new(10.0_f32), Time::new(24.0_f32))
        );
    }

    #[test]
    fn remap_and_try_remap_time_behave_correctly() {
        let src = TimeRange::new(Time::new(10.0_f32), Time::new(20.0_f32));
        let dst = TimeRange::new(Time::new(100.0_f32), Time::new(300.0_f32));

        assert_eq!(src.remap_time(Time::new(15.0_f32), &dst), Time::new(200.0_f32));
        assert_eq!(src.try_remap_time(Time::new(15.0_f32), &dst), Some(Time::new(200.0_f32)));

        let degenerate = TimeRange::new(Time::new(5.0_f32), Time::new(5.0_f32));
        assert_eq!(degenerate.try_remap_time(Time::new(5.0_f32), &dst), None);
    }

    #[test]
    fn split_contains_and_intersection_union_helpers_work() {
        let r = TimeRange::new(Time::new(10.0_f32), Time::new(20.0_f32));

        assert_eq!(
            r.split_at(Time::new(15.0_f32)),
            (
                Some(TimeRange::new(Time::new(10.0_f32), Time::new(15.0_f32))),
                Some(TimeRange::new(Time::new(15.0_f32), Time::new(20.0_f32)))
            )
        );
        assert_eq!(r.split_at(Time::new(10.0_f32)), (None, Some(r)));
        assert_eq!(r.split_at(Time::new(20.0_f32)), (Some(r), None));

        assert!(r.contains_time(Time::new(10.0_f32)));
        assert!(r.contains_time(Time::new(15.0_f32)));
        assert!(r.contains_time(Time::new(20.0_f32)));
        assert!(!r.contains_time(Time::new(25.0_f32)));

        let inner = TimeRange::new(Time::new(12.0_f32), Time::new(18.0_f32));
        let overlap = TimeRange::new(Time::new(18.0_f32), Time::new(25.0_f32));
        let disjoint = TimeRange::new(Time::new(30.0_f32), Time::new(40.0_f32));

        assert!(r.contains_range(&inner));
        assert!(inner.is_within(&r));
        assert!(r.intersects(&overlap));
        assert!(!r.intersects(&disjoint));

        assert_eq!(
            r.intersection(&overlap),
            Some(TimeRange::new(Time::new(18.0_f32), Time::new(20.0_f32)))
        );
        assert_eq!(r.intersection(&disjoint), None);

        assert_eq!(r.union(&overlap), TimeRange::new(Time::new(10.0_f32), Time::new(25.0_f32)));

        let mut in_place = r;
        in_place.union_in_place(&overlap);
        assert_eq!(in_place, TimeRange::new(Time::new(10.0_f32), Time::new(25.0_f32)));
    }

    #[test]
    fn include_and_aggregate_constructors_work() {
        let base = TimeRange::new(Time::new(10.0_f32), Time::new(20.0_f32));

        assert_eq!(
            base.include_time(Time::new(5.0_f32)),
            TimeRange::new(Time::new(5.0_f32), Time::new(20.0_f32))
        );

        let mut in_place = base;
        in_place.include_time_in_place(Time::new(25.0_f32));
        assert_eq!(in_place, TimeRange::new(Time::new(10.0_f32), Time::new(25.0_f32)));

        let ranges = [
            TimeRange::new(Time::new(10.0_f32), Time::new(12.0_f32)),
            TimeRange::new(Time::new(5.0_f32), Time::new(8.0_f32)),
            TimeRange::new(Time::new(11.0_f32), Time::new(30.0_f32)),
        ];

        assert_eq!(
            TimeRange::union_all(&ranges),
            Some(TimeRange::new(Time::new(5.0_f32), Time::new(30.0_f32)))
        );
        assert_eq!(
            TimeRange::union_iter(ranges),
            Some(TimeRange::new(Time::new(5.0_f32), Time::new(30.0_f32)))
        );

        let times = [Time::new(4.0_f32), Time::new(1.0_f32), Time::new(3.0_f32)];
        assert_eq!(
            TimeRange::from_time_slice(&times),
            Some(TimeRange::new(Time::new(1.0_f32), Time::new(4.0_f32)))
        );
        assert_eq!(
            TimeRange::from_time_iter(times),
            Some(TimeRange::new(Time::new(1.0_f32), Time::new(4.0_f32)))
        );
        assert_eq!(TimeRange::<Scalarf>::union_all(&[]), None);
        assert_eq!(TimeRange::<Scalarf>::from_time_slice(&[]), None);
    }
}

mod traits {
    use super::*;

    #[test]
    fn has_time_range_for_time_range_returns_some_values() {
        let r = TimeRange::new(Time::new(1.0_f32), Time::new(4.0_f32));
        assert_eq!(HasTimeRange::time_range(&r), Some(r));
        assert_eq!(HasTimeRange::duration(&r), Some(Duration::new(3.0_f32)));
    }

    #[test]
    fn has_duration_for_time_range_matches_inherent_duration() {
        let r = TimeRange::new(Time::new(1.0_f32), Time::new(4.5_f32));
        assert_eq!(HasDuration::duration(&r), Duration::new(3.5_f32));
    }
}

mod approx {
    use rinia::{
        assert_approx_eq_abs_tol, assert_approx_eq_rel_tol, assert_approx_ne_abs_tol,
        assert_approx_ne_rel_tol,
    };

    use super::*;

    #[test]
    fn approx_eq_abs_checks_both_start_and_end() {
        let a = TimeRange::new(Time::new(1.0_f32), Time::new(3.0_f32));
        let close = TimeRange::new(Time::new(1.0005_f32), Time::new(3.0005_f32));
        let far_end = TimeRange::new(Time::new(1.0005_f32), Time::new(3.01_f32));

        assert_approx_eq_abs_tol!(a, close, 0.001_f32);
        assert_approx_ne_abs_tol!(a, far_end, 0.001_f32);
    }

    #[test]
    fn approx_eq_rel_checks_both_start_and_end() {
        let a = TimeRange::new(Time::new(100.0_f32), Time::new(300.0_f32));
        let close = TimeRange::new(Time::new(100.2_f32), Time::new(300.6_f32));
        let far_start = TimeRange::new(Time::new(101.0_f32), Time::new(300.6_f32));

        assert_approx_eq_rel_tol!(a, close, 0.01_f32);
        assert_approx_ne_rel_tol!(a, far_start, 0.001_f32);
    }
}

mod compat {
    #[allow(unused_imports)]
    use super::*;

    #[cfg(feature = "bytemuck")]
    #[test]
    fn bytemuck_roundtrip() {
        let r = TimeRange::new(Time::new(10_i32), Time::new(20_i32));
        let bytes = bytemuck::bytes_of(&r);
        let out = bytemuck::pod_read_unaligned::<TimeRange<i32>>(bytes);
        assert_eq!(out, r);
    }

    #[cfg(feature = "zerocopy")]
    #[test]
    fn zerocopy_roundtrip() {
        let r = TimeRange::new(Time::new(30_u8), Time::new(40_u8));
        let bytes = <TimeRange<u8> as zerocopy::IntoBytes>::as_bytes(&r);
        let out = <TimeRange<u8> as zerocopy::FromBytes>::read_from_bytes(bytes)
            .expect("valid time range bytes");
        assert_eq!(out, r);
    }

    #[cfg(feature = "sakka")]
    #[test]
    fn sakka_roundtrip() {
        let r = TimeRange::new(Time::new(50_i32), Time::new(60_i32));

        let mut writer = sakka::Writer::new(sakka::Endian::Little, ());
        <TimeRange<i32> as sakka::Encode>::encode(&r, &mut writer).expect("encode time range");
        let bytes = writer.finish();

        let mut reader = sakka::Reader::new(&bytes, sakka::Endian::Little, ());
        let out =
            <TimeRange<i32> as sakka::Decode>::decode(&mut reader).expect("decode time range");

        assert_eq!(out, r);
        assert!(reader.is_eof());
    }
}
