use crate::{Duration, FloatScalar, HasDuration, Time, macros::impl_approx_forwarding};

/// Represents a bounded time interval with a start and end [Time].
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct TimeRange<T: FloatScalar> {
    start: Time<T>,
    end: Time<T>,
}

/// Trait for values that expose a [TimeRange].
pub trait HasTimeRange<T: FloatScalar> {
    /// Returns this value's [TimeRange], if applicable.
    fn time_range(&self) -> Option<TimeRange<T>>;

    /// Returns this value's [Duration], if applicable.
    #[inline]
    fn duration(&self) -> Option<Duration<T>> {
        self.time_range().map(|r| r.duration())
    }
}

impl<T: FloatScalar> TimeRange<T> {
    /// Creates a [TimeRange] from start and end without validation.
    ///
    /// This function allows invalid ranges (where start > end) to be used,
    /// which may lead to undefined behavior in subsequent calculations.
    ///
    /// Prefer [Self::new] for validated construction.
    #[inline]
    pub fn new_unchecked(start: Time<T>, end: Time<T>) -> Self {
        Self { start, end }
    }

    /// Creates a [TimeRange] from start and end.
    ///
    /// # Panics
    /// Panics if start is greater than end.
    #[inline]
    pub fn new(start: Time<T>, end: Time<T>) -> Self {
        assert!(start <= end, "TimeRange must satisfy start <= end");

        Self { start, end }
    }

    /// Returns the start [Time].
    #[inline]
    pub fn start(&self) -> Time<T> {
        self.start
    }

    /// Returns the end [Time].
    #[inline]
    pub fn end(&self) -> Time<T> {
        self.end
    }

    /// Returns this range's [Duration].
    #[inline]
    pub fn duration(&self) -> Duration<T> {
        self.end - self.start
    }

    /// Returns this range's [Duration].
    #[inline]
    pub fn len(&self) -> Duration<T> {
        self.duration()
    }

    /// Returns true when start and end are equal.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.duration().is_zero()
    }

    /// Clamps a [Time] to this range.
    #[inline]
    pub fn clamp_time(&self, t: Time<T>) -> Time<T> {
        t.max(self.start).min(self.end)
    }

    /// Normalizes a [Time] within this range.
    /// Returns an unclamped factor for non-degenerate ranges.
    #[inline]
    pub fn normalize_time(&self, t: Time<T>) -> T {
        t.normalize_between(self.start, self.end)
    }

    /// Returns the center [Time] of this range.
    #[inline]
    pub fn center(&self) -> Time<T> {
        self.start + (self.end - self.start) * T::raw(0.5)
    }

    /// Shifts this range by a [Duration].
    #[inline]
    pub fn shift(self, delta: Duration<T>) -> Self {
        Self { start: self.start + delta, end: self.end + delta }
    }

    /// Scales this range by a factor while keeping the center fixed.
    ///
    /// If the factor is negative, it will be clamped to zero, effectively collapsing the range.
    ///
    /// # Panics
    /// Panics if factor is NaN or infinite.
    #[inline]
    pub fn scale(self, factor: T) -> Self {
        assert!(factor.is_finite(), "Scale factor must be finite");

        let factor = factor.max(T::zero());
        let center = self.center();
        let half = (self.end - self.start) * (factor * T::raw(0.5));

        Self { start: center - half, end: center + half }
    }

    /// Scales this range by a factor while keeping the start fixed.
    ///
    /// If the factor is negative, it will be clamped to zero, effectively collapsing the range.
    ///
    /// # Panics
    /// Panics if factor is NaN or infinite.
    #[inline]
    pub fn scale_from_start(self, factor: T) -> Self {
        assert!(factor.is_finite(), "Scale factor must be finite");

        let factor = factor.max(T::zero());
        let duration = self.duration() * factor;
        Self { start: self.start, end: self.start + duration }
    }

    /// Expands this range by a [Duration] on both sides.
    #[inline]
    pub fn expand(self, amount: Duration<T>) -> Self {
        Self { start: self.start - amount, end: self.end + amount }
    }

    /// Pads the start of this range by a [Duration].
    #[inline]
    pub fn pad_start(self, amount: Duration<T>) -> Self {
        Self { start: self.start - amount, end: self.end }
    }

    /// Pads the end of this range by a [Duration].
    #[inline]
    pub fn pad_end(self, amount: Duration<T>) -> Self {
        Self { start: self.start, end: self.end + amount }
    }

    /// Remaps a [Time] from this range to a target range.
    #[inline]
    pub fn remap_time(&self, t: Time<T>, target: &Self) -> Time<T> {
        let u = self.normalize_time(t);
        target.start + (target.end - target.start) * u
    }

    /// Returns a [TimeRange] that starts at zero and preserves duration.
    #[inline]
    pub fn to_duration_range(&self) -> TimeRange<T> {
        let seconds = (self.end - self.start).seconds();
        let time = Time::from_seconds(seconds);
        Self { start: Time::zero(), end: time }
    }

    /// Splits this range at a [Time].
    #[inline]
    pub fn split_at(&self, t: Time<T>) -> (Option<Self>, Option<Self>) {
        if t <= self.start {
            (None, Some(*self))
        } else if t >= self.end {
            (Some(*self), None)
        } else {
            let left = Self { start: self.start, end: t };
            let right = Self { start: t, end: self.end };
            (Some(left), Some(right))
        }
    }

    /// Returns a new [TimeRange] that also includes a [Time].
    #[inline]
    pub fn include_time(&self, time: Time<T>) -> Self {
        let mut merged = *self;
        merged.include_time_in_place(time);
        merged
    }

    /// Expands this range to include a [Time].
    #[inline]
    pub fn include_time_in_place(&mut self, time: Time<T>) {
        self.start = self.start.min(time);
        self.end = self.end.max(time);
    }

    /// Returns true if this range contains a [Time].
    #[inline]
    pub fn contains_time(&self, time: Time<T>) -> bool {
        self.start <= time && time <= self.end
    }

    /// Returns true if this range fully contains another [TimeRange].
    #[inline]
    pub fn contains_range(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    /// Returns true if this range is fully contained within another [TimeRange].
    #[inline]
    pub fn is_within(&self, other: &Self) -> bool {
        other.contains_range(self)
    }

    /// Returns true if this range overlaps another [TimeRange].
    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    /// Returns the overlapping portion of this range and another [TimeRange].
    #[inline]
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if !self.intersects(other) {
            return None;
        }

        Some(Self { start: self.start.max(other.start), end: self.end.min(other.end) })
    }

    /// Returns a new [TimeRange] that encompasses this range and another.
    #[inline]
    pub fn union(&self, other: &Self) -> Self {
        let mut merged = *self;
        merged.union_in_place(other);
        merged
    }

    /// Expands this range to encompass another [TimeRange].
    #[inline]
    pub fn union_in_place(&mut self, other: &Self) {
        self.start = self.start.min(other.start);
        self.end = self.end.max(other.end);
    }

    /// Returns a [TimeRange] that encompasses all ranges in a slice.
    #[inline]
    pub fn union_all(ranges: &[Self]) -> Option<Self> {
        Self::union_iter(ranges.iter().copied())
    }

    /// Returns a [TimeRange] that encompasses all ranges from an iterator.
    #[inline]
    pub fn union_iter<I>(ranges: I) -> Option<Self>
    where
        I: IntoIterator<Item = Self>,
    {
        let mut iter = ranges.into_iter();
        let first = iter.next()?;

        let mut merged = first;

        for range in iter {
            merged.union_in_place(&range);
        }

        Some(merged)
    }

    /// Returns a [TimeRange] that encompasses all times in a slice.
    #[inline]
    pub fn from_times(times: &[Time<T>]) -> Option<Self> {
        Self::from_times_iter(times.iter().copied())
    }

    /// Returns a [TimeRange] that encompasses all times from an iterator.
    #[inline]
    pub fn from_times_iter<I>(times: I) -> Option<Self>
    where
        I: IntoIterator<Item = Time<T>>,
    {
        let mut iter = times.into_iter();
        let first = iter.next()?;

        let mut range = Self::new(first, first);

        for time in iter {
            range.include_time_in_place(time);
        }

        Some(range)
    }
}

// Implement HasTimeRange for TimeRange itself
impl<T: FloatScalar> HasTimeRange<T> for TimeRange<T> {
    #[inline]
    fn time_range(&self) -> Option<TimeRange<T>> {
        Some(*self)
    }
}

// Implement HasDuration for TimeRange
impl<T: FloatScalar> HasDuration<T> for TimeRange<T> {
    #[inline]
    fn duration(&self) -> Duration<T> {
        TimeRange::duration(self)
    }
}

// Range conversion
impl<T: FloatScalar> From<core::range::Range<T>> for TimeRange<T> {
    #[inline]
    fn from(range: core::range::Range<T>) -> Self {
        Self::new(Time::from_seconds(range.start), Time::from_seconds(range.end))
    }
}

impl<T: FloatScalar> From<core::range::Range<Time<T>>> for TimeRange<T> {
    #[inline]
    fn from(range: core::range::Range<Time<T>>) -> Self {
        Self::new(range.start, range.end)
    }
}

impl<T: FloatScalar> From<core::ops::Range<T>> for TimeRange<T> {
    #[inline]
    fn from(range: core::ops::Range<T>) -> Self {
        Self::new(Time::from_seconds(range.start), Time::from_seconds(range.end))
    }
}

impl<T: FloatScalar> From<core::ops::Range<Time<T>>> for TimeRange<T> {
    #[inline]
    fn from(range: core::ops::Range<Time<T>>) -> Self {
        Self::new(range.start, range.end)
    }
}

// Equality forwarding
impl_approx_forwarding!(TimeRange<T>, start, end);

#[cfg(test)]
mod tests {
    use approx::{AbsDiffEq, RelativeEq, UlpsEq, assert_abs_diff_eq};

    use super::*;

    fn t(v: f64) -> Time<f64> {
        Time::from_seconds(v)
    }

    fn d(v: f64) -> Duration<f64> {
        Duration::from_seconds(v)
    }

    #[test]
    fn new_start_end_duration_len_and_empty_work() {
        let r = TimeRange::new(t(1.0), t(4.0));
        assert_eq!(r.start(), t(1.0));
        assert_eq!(r.end(), t(4.0));
        assert_eq!(r.duration(), d(3.0));
        assert_eq!(r.len(), d(3.0));
        assert!(!r.is_empty());

        let empty = TimeRange::new(t(2.0), t(2.0));
        assert!(empty.is_empty());
    }

    #[test]
    #[should_panic(expected = "TimeRange must satisfy start <= end")]
    fn new_panics_when_start_greater_than_end() {
        let _ = TimeRange::new(t(5.0), t(4.0));
    }

    #[test]
    fn clamp_normalize_center_shift_and_scaling_work() {
        let r = TimeRange::new(t(2.0), t(6.0));

        assert_eq!(r.clamp_time(t(1.0)), t(2.0));
        assert_eq!(r.clamp_time(t(8.0)), t(6.0));
        assert_eq!(r.clamp_time(t(3.0)), t(3.0));

        assert_abs_diff_eq!(r.normalize_time(t(4.0)), 0.5, epsilon = 1e-12);
        assert_abs_diff_eq!(r.normalize_time(t(8.0)), 1.5, epsilon = 1e-12);
        assert_abs_diff_eq!(r.center().seconds(), 4.0, epsilon = 1e-12);

        let shifted = r.shift(d(2.0));
        assert_eq!(shifted, TimeRange::new(t(4.0), t(8.0)));

        let scaled = r.scale(0.5);
        assert_eq!(scaled, TimeRange::new(t(3.0), t(5.0)));

        let collapsed = r.scale(-3.0);
        assert_eq!(collapsed, TimeRange::new(t(4.0), t(4.0)));

        let from_start = r.scale_from_start(1.5);
        assert_eq!(from_start, TimeRange::new(t(2.0), t(8.0)));

        let collapsed_start = r.scale_from_start(-1.0);
        assert_eq!(collapsed_start, TimeRange::new(t(2.0), t(2.0)));
    }

    #[test]
    #[should_panic(expected = "Scale factor must be finite")]
    fn scale_panics_on_non_finite_factor() {
        let _ = TimeRange::new(t(0.0), t(1.0)).scale(f64::INFINITY);
    }

    #[test]
    fn padding_expansion_remap_and_duration_range_work() {
        let r = TimeRange::new(t(1.0), t(3.0));

        assert_eq!(r.expand(d(1.0)), TimeRange::new(t(0.0), t(4.0)));
        assert_eq!(r.pad_start(d(0.5)), TimeRange::new(t(0.5), t(3.0)));
        assert_eq!(r.pad_end(d(0.5)), TimeRange::new(t(1.0), t(3.5)));

        let target = TimeRange::new(t(10.0), t(20.0));
        assert_eq!(r.remap_time(t(2.0), &target), t(15.0));

        let duration_range = r.to_duration_range();
        assert_eq!(duration_range, TimeRange::new(t(0.0), t(2.0)));
    }

    #[test]
    fn split_union_include_and_relationship_methods_work() {
        let r = TimeRange::new(t(1.0), t(5.0));

        assert_eq!(r.split_at(t(0.0)), (None, Some(r)));
        assert_eq!(r.split_at(t(6.0)), (Some(r), None));

        let (left, right) = r.split_at(t(3.0));
        assert_eq!(left, Some(TimeRange::new(t(1.0), t(3.0))));
        assert_eq!(right, Some(TimeRange::new(t(3.0), t(5.0))));

        let other = TimeRange::new(t(4.0), t(8.0));
        assert_eq!(r.union(&other), TimeRange::new(t(1.0), t(8.0)));

        let mut in_place = r;
        in_place.union_in_place(&other);
        assert_eq!(in_place, TimeRange::new(t(1.0), t(8.0)));

        assert_eq!(r.include_time(t(-1.0)), TimeRange::new(t(-1.0), t(5.0)));

        let mut include_in_place = r;
        include_in_place.include_time_in_place(t(10.0));
        assert_eq!(include_in_place, TimeRange::new(t(1.0), t(10.0)));

        assert!(r.contains_time(t(2.0)));
        assert!(!r.contains_time(t(6.0)));

        let inner = TimeRange::new(t(2.0), t(4.0));
        assert!(r.contains_range(&inner));
        assert!(inner.is_within(&r));

        assert!(r.intersects(&other));
        assert_eq!(r.intersection(&other), Some(TimeRange::new(t(4.0), t(5.0))));

        let disjoint = TimeRange::new(t(9.0), t(10.0));
        assert!(!r.intersects(&disjoint));
        assert_eq!(r.intersection(&disjoint), None);
    }

    #[test]
    fn aggregate_constructors_cover_empty_and_non_empty_cases() {
        let a = TimeRange::new(t(1.0), t(3.0));
        let b = TimeRange::new(t(-2.0), t(2.0));
        let c = TimeRange::new(t(5.0), t(8.0));

        assert_eq!(TimeRange::<f64>::union_all(&[]), None);
        assert_eq!(TimeRange::union_all(&[a, b, c]), Some(TimeRange::new(t(-2.0), t(8.0))));

        assert_eq!(TimeRange::<f64>::union_iter(core::iter::empty()), None);
        assert_eq!(TimeRange::union_iter([a, b, c]), Some(TimeRange::new(t(-2.0), t(8.0))));

        assert_eq!(TimeRange::<f64>::from_times(&[]), None);
        assert_eq!(
            TimeRange::from_times(&[t(5.0), t(-1.0), t(2.0)]),
            Some(TimeRange::new(t(-1.0), t(5.0)))
        );

        assert_eq!(TimeRange::<f64>::from_times_iter(core::iter::empty()), None);
        assert_eq!(
            TimeRange::from_times_iter([t(3.0), t(3.0), t(10.0)]),
            Some(TimeRange::new(t(3.0), t(10.0)))
        );
    }

    #[test]
    fn trait_impls_return_expected_values() {
        let r = TimeRange::new(t(2.0), t(7.0));

        assert_eq!(HasTimeRange::time_range(&r), Some(r));
        assert_eq!(HasTimeRange::duration(&r), Some(d(5.0)));
        assert_eq!(HasDuration::duration(&r), d(5.0));
    }

    #[test]
    fn from_range_conversions_work() {
        let scalar_range = core::range::Range { start: 1.25_f64, end: 3.5_f64 };
        let from_scalar: TimeRange<f64> = TimeRange::from(scalar_range);
        assert_eq!(from_scalar.start(), t(1.25));
        assert_eq!(from_scalar.end(), t(3.5));

        let time_range = core::range::Range { start: t(2.0), end: t(6.0) };
        let from_time: TimeRange<f64> = TimeRange::from(time_range);
        assert_eq!(from_time.start(), t(2.0));
        assert_eq!(from_time.end(), t(6.0));
    }

    #[test]
    fn approx_traits_compare_start_and_end() {
        let a = TimeRange::new(t(1.0), t(2.0));
        let close = TimeRange::new(t(1.001), t(2.001));
        let far = TimeRange::new(t(1.1), t(2.1));

        assert!(a.abs_diff_eq(&close, 0.01));
        assert!(!a.abs_diff_eq(&far, 0.01));

        assert!(a.relative_eq(&close, 0.01, 0.01));
        assert!(!a.relative_eq(&far, 0.01, 0.01));

        assert!(a.ulps_eq(&a, f64::default_epsilon(), f64::default_max_ulps()));
    }
}
