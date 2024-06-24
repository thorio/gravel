//! Taken from https://github.com/rodrimati1992/abi_stable_crates/issues/73 and modified for Fn

use abi_stable::StableAbi;
use std::fmt::Debug;

#[repr(C)]
#[derive(StableAbi)]
pub struct RBoxFn<TParam, TResult> {
	caller: extern "C" fn(usize, &TParam) -> TResult,
	remover: extern "C" fn(usize),
	inner: usize,
}

impl<T, TParam, TResult> From<T> for RBoxFn<TParam, TResult>
where
	T: Fn(&TParam) -> TResult,
{
	fn from(inner: T) -> Self {
		extern "C" fn caller<T, TParam, TResult>(ptr: usize, param: &TParam) -> TResult
		where
			T: Fn(&TParam) -> TResult,
		{
			let function = unsafe { &*(ptr as *mut T) };
			(function)(param)
		}

		extern "C" fn dropper<T, TParam, TResult>(ptr: usize)
		where
			T: Fn(&TParam) -> TResult,
		{
			let function = unsafe { Box::from_raw(ptr as *mut T) };
			drop(function);
		}

		Self {
			caller: caller::<T, TParam, TResult>,
			remover: dropper::<T, TParam, TResult>,
			inner: Box::into_raw(Box::new(inner)) as usize,
		}
	}
}

impl<TParam, TResult> Drop for RBoxFn<TParam, TResult> {
	fn drop(&mut self) {
		(self.remover)(self.inner);
	}
}

impl<TParam, TResult> RBoxFn<TParam, TResult> {
	pub fn call(&self, p: &TParam) -> TResult {
		(self.caller)(self.inner, p)
	}
}

impl<TParam, TResult> Debug for RBoxFn<TParam, TResult> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "<RBoxFn:{:p}>", self.inner as *mut ())
	}
}
