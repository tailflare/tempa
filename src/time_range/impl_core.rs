use core::ops::{Add, Div, Mul, Sub};

use rinia::numeric::{Half, IsFinite, MinMax, Zero};

use crate::{Duration, HasDuration, HasTimeRange, Time, TimeRange, common};

impl<T> TimeRange<T> {
    /// Creates a [TimeRange] from start and end without validation.
    ///
    /// This function allows invalid values to be used, which may lead to invalid results in
    /// subsequent calculations.
    ///
    /// Prefer [Self::try_new] for validated construction.
    #[inline]
    pub fn new(start: Time<T>, end: Time<T>) -> Self {
        Self { start, end }
    }

    /// Creates a [TimeRange] from start and end with validation.
    ///
    /// Returns an error if start or end is not finite, or if start is greater than end.
    #[inline]
    pub fn try_new(start: Time<T>, end: Time<T>) -> Result<Self, crate::Error>
    where
        T: Copy + IsFinite + PartialOrd,
    {
        if !start.is_finite() || !end.is_finite() {
            return Err(crate::Error::InvalidValue(
                "TimeRange cannot be created from NaN or infinite values",
            ));
        }

        if start > end {
            return Err(crate::Error::InvalidValue(
                "TimeRange cannot be created with start greater than end",
            ));
        }

        Ok(Self { start, end })
    }

    /// Tries to create a [TimeRange] from start and end with validation.
    ///
    /// # Panics
    /// Panics if start or end is not finite, or if start is greater than end.
    #[inline]
    pub fn from_times(start: Time<T>, end: Time<T>) -> Self
    where
        T: Copy + IsFinite + PartialOrd,
    {
        Self::try_new(start, end).unwrap_or_else(|err| panic!("{err}"))
    }

    /// Returns the start [Time].
    #[inline]
    pub fn start(&self) -> Time<T>
    where
        T: Copy,
    {
        self.start
    }

    /// Returns the end [Time].
    #[inline]
    pub fn end(&self) -> Time<T>
    where
        T: Copy,
    {
        self.end
    }

    /// Consumes this [TimeRange] and returns the start and end [Time] values.
    #[inline]
    pub fn into_parts(self) -> (Time<T>, Time<T>) {
        (self.start, self.end)
    }

    /// Returns this range's [Duration].
    #[inline]
    pub fn duration(&self) -> Duration<T>
    where
        Time<T>: Copy + Sub<Output = Duration<T>>,
    {
        self.end - self.start
    }

    /// Clamps a [Time] to this range.
    #[inline]
    pub fn clamp_time(&self, t: Time<T>) -> Time<T>
    where
        Time<T>: Copy + MinMax,
    {
        t.maximum(self.start).minimum(self.end)
    }

    /// Normalizes a [Time] within this range.
    /// Returns an unclamped factor for non-degenerate ranges.
    #[inline]
    pub fn normalize_time(&self, t: Time<T>) -> T
    where
        Time<T>: Copy + Sub<Output = Duration<T>>,
        Duration<T>: Div<Output = T>,
    {
        (t - self.start) / (self.end - self.start)
    }

    /// Tries to normalize a [Time] within this range.
    /// Returns None for degenerate ranges.
    #[inline]
    pub fn try_normalize_time(&self, t: Time<T>) -> Option<T>
    where
        T: Zero,
        Time<T>: Copy + Sub<Output = Duration<T>>,
        Duration<T>: PartialEq + Zero + Div<Output = T>,
    {
        let duration = self.end - self.start;

        if duration == Duration::ZERO { None } else { Some((t - self.start) / duration) }
    }

    /// Returns the center [Time] of this range.
    #[inline]
    pub fn center(&self) -> Time<T>
    where
        T: Half,
        Time<T>: Copy + Sub<Output = Duration<T>> + Add<Duration<T>, Output = Time<T>>,
        Duration<T>: Mul<T, Output = Duration<T>>,
    {
        self.start + ((self.end - self.start) * T::HALF)
    }

    /// Shifts this range by a [Duration].
    #[inline]
    pub fn shift(self, delta: Duration<T>) -> Self
    where
        Time<T>: Copy + Add<Duration<T>, Output = Time<T>>,
        Duration<T>: Copy,
    {
        Self { start: self.start + delta, end: self.end + delta }
    }

    /// Scales this range by a factor while keeping the center fixed.
    ///
    /// If the factor is negative, it will be clamped to zero, effectively collapsing the range.
    #[inline]
    pub fn scale(self, factor: T) -> Self
    where
        T: Copy + Zero + Half + MinMax + Mul<Output = T>,
        Time<T>: Sub<Output = Duration<T>>
            + Add<Duration<T>, Output = Time<T>>
            + Sub<Duration<T>, Output = Time<T>>,
        Duration<T>: Mul<T, Output = Duration<T>>,
    {
        let factor = factor.maximum(T::ZERO);
        let center = self.center();
        let half = (self.end - self.start) * (factor * T::HALF);

        Self { start: center - half, end: center + half }
    }

    /// Scales this range by a factor while keeping the start fixed.
    ///
    /// If the factor is negative, it will be clamped to zero, effectively collapsing the range.
    #[inline]
    pub fn scale_from_start(self, factor: T) -> Self
    where
        T: Copy + Zero + MinMax,
        Time<T>: Sub<Output = Duration<T>>
            + Add<Duration<T>, Output = Time<T>>
            + Sub<Duration<T>, Output = Time<T>>,
        Duration<T>: Mul<T, Output = Duration<T>>,
    {
        let factor = factor.maximum(T::ZERO);
        let duration = self.duration() * factor;
        Self { start: self.start, end: self.start + duration }
    }

    /// Expands this range by a [Duration] on both sides.
    #[inline]
    pub fn expand(self, amount: Duration<T>) -> Self
    where
        Time<T>: Sub<Duration<T>, Output = Time<T>> + Add<Duration<T>, Output = Time<T>>,
        Duration<T>: Copy,
    {
        Self { start: self.start - amount, end: self.end + amount }
    }

    /// Pads the start of this range by a [Duration].
    #[inline]
    pub fn pad_start(self, amount: Duration<T>) -> Self
    where
        Time<T>: Sub<Duration<T>, Output = Time<T>>,
    {
        Self { start: self.start - amount, end: self.end }
    }

    /// Pads the end of this range by a [Duration].
    #[inline]
    pub fn pad_end(self, amount: Duration<T>) -> Self
    where
        Time<T>: Add<Duration<T>, Output = Time<T>>,
    {
        Self { start: self.start, end: self.end + amount }
    }

    /// Remaps a [Time] from this range to a target range.
    #[inline]
    pub fn remap_time(&self, t: Time<T>, target: &Self) -> Time<T>
    where
        T: Copy,
        Time<T>: Copy + Sub<Output = Duration<T>> + Add<Duration<T>, Output = Time<T>>,
        Duration<T>: Zero + PartialEq + Mul<T, Output = Duration<T>> + Div<Output = T>,
    {
        let u = self.normalize_time(t);
        target.start + (target.end - target.start) * u
    }

    /// Tries to remap a [Time] from this range to a target range.
    /// Returns None if this range is degenerate.
    #[inline]
    pub fn try_remap_time(&self, t: Time<T>, target: &Self) -> Option<Time<T>>
    where
        T: Copy + Zero,
        Time<T>: Sub<Output = Duration<T>> + Add<Duration<T>, Output = Time<T>>,
        Duration<T>: Zero + PartialEq + Mul<T, Output = Duration<T>> + Div<Output = T>,
    {
        let u = self.try_normalize_time(t)?;
        Some(target.start + (target.end - target.start) * u)
    }

    /// Returns a [TimeRange] that starts at zero and preserves duration.
    #[inline]
    pub fn to_duration_range(&self) -> TimeRange<T>
    where
        T: Copy + Zero,
        Time<T>: Copy + Sub<Output = Duration<T>>,
    {
        let seconds = (self.end - self.start).seconds();
        let time = Time::new(seconds);
        Self { start: Time::ZERO, end: time }
    }

    /// Splits this range at a [Time].
    #[inline]
    pub fn split_at(&self, t: Time<T>) -> (Option<Self>, Option<Self>)
    where
        Self: Copy,
        Time<T>: Copy + PartialOrd,
    {
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
    pub fn include_time(&self, time: Time<T>) -> Self
    where
        Self: Copy,
        Time<T>: Copy + MinMax,
    {
        let mut merged = *self;
        merged.include_time_in_place(time);
        merged
    }

    /// Expands this range to include a [Time].
    #[inline]
    pub fn include_time_in_place(&mut self, time: Time<T>)
    where
        Time<T>: Copy + MinMax,
    {
        self.start = self.start.minimum(time);
        self.end = self.end.maximum(time);
    }

    /// Returns true if this range contains a [Time].
    #[inline]
    pub fn contains_time(&self, time: Time<T>) -> bool
    where
        Time<T>: PartialOrd,
    {
        self.start <= time && time <= self.end
    }

    /// Returns true if this range fully contains another [TimeRange].
    #[inline]
    pub fn contains_range(&self, other: &Self) -> bool
    where
        Time<T>: PartialOrd,
    {
        self.start <= other.start && other.end <= self.end
    }

    /// Returns true if this range is fully contained within another [TimeRange].
    #[inline]
    pub fn is_within(&self, other: &Self) -> bool
    where
        Time<T>: PartialOrd,
    {
        other.contains_range(self)
    }

    /// Returns true if this range overlaps another [TimeRange].
    #[inline]
    pub fn intersects(&self, other: &Self) -> bool
    where
        Time<T>: PartialOrd,
    {
        self.start <= other.end && other.start <= self.end
    }

    /// Returns the overlapping portion of this range and another [TimeRange].
    #[inline]
    pub fn intersection(&self, other: &Self) -> Option<Self>
    where
        Time<T>: Copy + PartialOrd + MinMax,
    {
        if !self.intersects(other) {
            return None;
        }

        Some(Self { start: self.start.maximum(other.start), end: self.end.minimum(other.end) })
    }

    /// Returns a new [TimeRange] that encompasses this range and another.
    #[inline]
    pub fn union(&self, other: &Self) -> Self
    where
        Self: Copy,
        Time<T>: Copy + MinMax,
    {
        let mut merged = *self;
        merged.union_in_place(other);
        merged
    }

    /// Expands this range to encompass another [TimeRange].
    #[inline]
    pub fn union_in_place(&mut self, other: &Self)
    where
        Time<T>: Copy + MinMax,
    {
        self.start = self.start.minimum(other.start);
        self.end = self.end.maximum(other.end);
    }

    /// Returns a [TimeRange] that encompasses all ranges in a slice.
    #[inline]
    pub fn union_all(ranges: &[Self]) -> Option<Self>
    where
        Self: Copy,
        Time<T>: Copy + MinMax,
    {
        Self::union_iter(ranges.iter().copied())
    }

    /// Returns a [TimeRange] that encompasses all ranges from an iterator.
    #[inline]
    pub fn union_iter<I>(ranges: I) -> Option<Self>
    where
        I: IntoIterator<Item = Self>,
        Time<T>: Copy + MinMax,
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
    pub fn from_time_slice(times: &[Time<T>]) -> Option<Self>
    where
        Time<T>: Copy + MinMax,
    {
        Self::from_time_iter(times.iter().copied())
    }

    /// Returns a [TimeRange] that encompasses all times from an iterator.
    #[inline]
    pub fn from_time_iter<I>(times: I) -> Option<Self>
    where
        I: IntoIterator<Item = Time<T>>,
        Time<T>: Copy + MinMax,
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

// Impl Default for TimeRange<T>
impl<T> Default for TimeRange<T>
where
    Time<T>: Zero,
{
    fn default() -> Self {
        Self { start: Time::<T>::ZERO, end: Time::<T>::ZERO }
    }
}

// Implement HasRange for TimeRange itself
impl<T> HasTimeRange<T> for TimeRange<T>
where
    T: Copy,
    Time<T>: Copy + Sub<Output = Duration<T>>,
{
    #[inline]
    fn time_range(&self) -> Option<TimeRange<T>> {
        Some(*self)
    }

    #[inline]
    fn duration(&self) -> Option<Duration<T>> {
        Some(self.duration())
    }
}

// Implement HasDuration for TimeRange
impl<T> HasDuration<T> for TimeRange<T>
where
    Time<T>: Copy + Sub<Output = Duration<T>>,
{
    #[inline]
    fn duration(&self) -> Duration<T> {
        self.duration()
    }
}

// Impl approx equality traits for TimeRange<T>
common::impl_approx_forwarding!(TimeRange<T>, start, end);

// Impl bytemuck for TimeRange<T>
common::impl_bytemuck_basic!(
    [T],
    TimeRange<T>,
    item: T,
);

// Conversion
impl<T> From<core::range::Range<Time<T>>> for TimeRange<T> {
    #[inline]
    fn from(range: core::range::Range<Time<T>>) -> Self {
        Self::new(range.start, range.end)
    }
}

impl<T> From<core::ops::Range<Time<T>>> for TimeRange<T> {
    #[inline]
    fn from(range: core::ops::Range<Time<T>>) -> Self {
        Self::new(range.start, range.end)
    }
}

impl<T> From<core::range::Range<T>> for TimeRange<T> {
    #[inline]
    fn from(range: core::range::Range<T>) -> Self {
        Self::new(Time::new(range.start), Time::new(range.end))
    }
}

impl<T> From<core::ops::Range<T>> for TimeRange<T> {
    #[inline]
    fn from(range: core::ops::Range<T>) -> Self {
        Self::new(Time::new(range.start), Time::new(range.end))
    }
}
