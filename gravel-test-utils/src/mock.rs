use abi_stable::sabi_trait;
use gravel_ffi::{HitActionContext, RefDynHitActionContext};
use mockall::mock;

mock! {
	pub HitActionContext {}
	impl HitActionContext for HitActionContext {
		fn hide_frontend(&self);
		fn refresh_frontend(&self);
		fn exit(&self);
		fn restart(&self);
	}
}

impl<'a> From<&'a MockHitActionContext> for RefDynHitActionContext<'a> {
	fn from(value: &'a MockHitActionContext) -> Self {
		Self::from_ptr(value, sabi_trait::TD_Opaque)
	}
}
