#[cfg(feature = "bytemuck")]
macro_rules! impl_bytemuck_basic {
	([$($generic:tt)*], $type:ty, item: $item:ty $(,)?) => {

		unsafe impl<$($generic)*> ::bytemuck::Zeroable for $type
		where
			$item: ::bytemuck::Zeroable,
		{
		}

		unsafe impl<$($generic)*> ::bytemuck::Pod for $type
		where
			$item: ::bytemuck::Pod,
		{
		}
	};
}

#[cfg(not(feature = "bytemuck"))]
macro_rules! impl_bytemuck_basic {
    ([$($generic:tt)*], $type:ty, item: $item:ty $(,)?) => {};
}

pub(crate) use impl_bytemuck_basic;
