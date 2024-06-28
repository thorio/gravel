//! Taken from https://github.com/rodrimati1992/abi_stable_crates/issues/73 and modified

use abi_stable::StableAbi;
use std::fmt::{Debug, Formatter, Result};

#[repr(C)]
#[derive(StableAbi)]
pub struct RBoxFn<P1, P2, R> {
	caller: extern "C" fn(usize, &P1, &P2) -> R,
	dropper: extern "C" fn(usize),
	inner: usize,
}

impl<F, P1, P2, R> From<F> for RBoxFn<P1, P2, R>
where
	F: Fn(&P1, &P2) -> R,
{
	fn from(inner: F) -> Self {
		extern "C" fn caller<F, P1, P2, R>(ptr: usize, p1: &P1, p2: &P2) -> R
		where
			F: Fn(&P1, &P2) -> R,
		{
			let function = unsafe { &*(ptr as *mut F) };
			(function)(p1, p2)
		}

		extern "C" fn dropper<F, P1, P2, R>(ptr: usize)
		where
			F: Fn(&P1, &P2) -> R,
		{
			let function = unsafe { Box::from_raw(ptr as *mut F) };
			drop(function);
		}

		Self {
			caller: caller::<F, P1, P2, R>,
			dropper: dropper::<F, P1, P2, R>,
			inner: Box::into_raw(Box::new(inner)) as usize,
		}
	}
}

impl<P1, P2, R> Drop for RBoxFn<P1, P2, R> {
	fn drop(&mut self) {
		(self.dropper)(self.inner);
	}
}

impl<P1, P2, R> RBoxFn<P1, P2, R> {
	pub fn call(&self, p1: &P1, p2: &P2) -> R {
		(self.caller)(self.inner, p1, p2)
	}
}

impl<P1, P2, R> Debug for RBoxFn<P1, P2, R> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "<RBoxFn:{:p}>", self.inner as *mut ())
	}
}
