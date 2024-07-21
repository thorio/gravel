use abi_stable::{sabi_trait, std_types::RStr};
use gravel_core::engine::QueryEngine;
use gravel_ffi::{ActionKind, ArcDynHit, BoxDynFrontendContext, FrontendContext, QueryResult};

pub struct Runner {
	engine: QueryEngine,
}

impl Runner {
	pub fn new(engine: QueryEngine) -> Self {
		Self { engine }
	}

	fn run_action(&self, hit: &ArcDynHit, kind: ActionKind) {
		self.engine.run_hit_action(hit, kind);
	}
}

impl FrontendContext for Runner {
	fn query(&self, query: RStr<'_>) -> QueryResult {
		self.engine.query(query)
	}

	fn run_hit_action(&self, hit: &ArcDynHit) {
		self.run_action(hit, ActionKind::Primary);
	}

	fn run_secondary_hit_action(&self, hit: &ArcDynHit) {
		self.run_action(hit, ActionKind::Secondary);
	}
}

impl From<Runner> for BoxDynFrontendContext {
	fn from(value: Runner) -> Self {
		Self::from_value(value, sabi_trait::TD_Opaque)
	}
}
