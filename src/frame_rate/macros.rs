macro_rules! impl_rate_fps {
	($($fps:literal),+ $(,)?) => {
		impl<T> FrameRate<T>
		where
			T: ::rinia::numeric::LossyCastFrom<u16>,
		{
			$(
				::pastey::paste! {
					#[doc = concat!("Returns a [FrameRate] of ", stringify!($fps), " frames per second.")]
					#[inline]
					pub fn [<fps_ $fps>]() -> Self {
						Self::new(T::lossy_cast_from($fps as u16))
					}
				}
			)+
		}
	};
}

pub(crate) use impl_rate_fps;
