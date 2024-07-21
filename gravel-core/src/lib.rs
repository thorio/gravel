//! gravel's core library.
//!
//! Contains functionality used internally by gravel, and is generally not
//! required for writing plugins.

use abi_stable::external_types::crossbeam_channel::{RReceiver, RSender};
use abi_stable::{sabi_trait, std_types::RStr};
use engine::QueryEngine;
use gravel_ffi::{ActionKind, ArcDynHit, QueryResult};
use gravel_ffi::{BoxDynFrontendContext, FrontendContext, FrontendMessage, FrontendMessageNe};
use performance::Stopwatch;

pub mod config;
pub mod engine;
pub mod hotkeys;
pub mod paths;
pub mod performance;
pub mod plugin;
pub mod scoring;

pub struct Core {
	engine: QueryEngine,
	frontend_sender: RSender<FrontendMessageNe>,
	receiver: RReceiver<CoreMessage>,
}

pub enum CoreMessage {
	ClearCaches,
	Frontend(FrontendMessage),
}

impl Core {
	pub fn new(
		engine: QueryEngine,
		frontend_sender: RSender<FrontendMessageNe>,
		receiver: RReceiver<CoreMessage>,
	) -> Self {
		Self {
			engine,
			frontend_sender,
			receiver,
		}
	}

	fn run_action(&self, hit: &ArcDynHit, kind: ActionKind) {
		let stopwatch = Stopwatch::start();

		self.engine.run_hit_action(hit, kind);

		log::trace!("hit action took {stopwatch}");

		self.process_messages();
	}

	fn process_messages(&self) {
		if let Ok(message) = self.receiver.try_recv() {
			match message {
				CoreMessage::ClearCaches => self.clear_caches(),
				CoreMessage::Frontend(m) => self.send_frontend(m),
			}

			self.process_messages();
		}
	}

	fn clear_caches(&self) {
		log::debug!("clearing caches");

		self.send_frontend(FrontendMessage::ClearCaches);
		self.engine.clear_caches();
	}

	fn send_frontend(&self, message: FrontendMessage) {
		self.frontend_sender
			.send(FrontendMessageNe::new(message))
			.inspect_err(|e| log::error!("unable to send frontend message: {e}"))
			.ok();
	}
}

impl FrontendContext for Core {
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

impl From<Core> for BoxDynFrontendContext {
	fn from(value: Core) -> Self {
		Self::from_value(value, sabi_trait::TD_Opaque)
	}
}
