//! gravel's core library.
//!
//! Contains functionality used internally by gravel, and is generally not
//! required for writing plugins.

use abi_stable::external_types::crossbeam_channel::{RReceiver, RSender};
use abi_stable::sabi_trait;
use abi_stable::std_types::RString;
use abi_stable::traits::{IntoReprC, IntoReprRust};
use engine::QueryEngine;
use gravel_ffi::{ActionKind, ArcDynHit};
use gravel_ffi::{BoxDynFrontendContext, FrontendContext, FrontendMessage, FrontendMessageNe};
use performance::Stopwatch;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

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
	Frontend(FrontendMessage),
	Query(u32, String),
	ClearCaches,
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

	pub fn run(&self) {
		loop {
			self.receive_message();
		}
	}

	fn receive_message(&self) -> Option<()> {
		const ONE_MILLI: Duration = Duration::from_millis(1);

		match self.receiver.recv_timeout(ONE_MILLI).ok()? {
			CoreMessage::Frontend(m) => self.send_frontend(m),
			CoreMessage::Query(t, q) => self.query(t, q),
			CoreMessage::ClearCaches => self.clear_caches(),
		}

		None
	}

	fn run_action(&self, hit: &ArcDynHit, kind: ActionKind) {
		let stopwatch = Stopwatch::start();

		self.engine.run_hit_action(hit, kind);

		log::trace!("hit action took {stopwatch}");
	}

	fn query(&self, token: u32, query: String) {
		let result = self.engine.query(query.into_c().as_rstr());

		self.send_frontend(FrontendMessage::QueryResult(token, result));
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

pub struct FrontendCtx {
	sender: RSender<CoreMessage>,
	token_counter: AtomicU32,
}

impl FrontendCtx {
	pub fn new(sender: RSender<CoreMessage>) -> Self {
		Self {
			sender,
			token_counter: AtomicU32::default(),
		}
	}

	fn send(&self, message: CoreMessage) {
		self.sender
			.send(message)
			.inspect_err(|e| log::error!("unable to send core message: {e}"))
			.ok();
	}

	fn get_token(&self) -> u32 {
		self.token_counter.fetch_add(0, Ordering::Relaxed)
	}
}

impl FrontendContext for FrontendCtx {
	fn query(&self, query: RString) -> u32 {
		let token = self.get_token();
		self.send(CoreMessage::Query(token, query.into_rust()));

		token
	}

	fn run_hit_action(&self, hit: &ArcDynHit, kind: ActionKind) {
		todo!();
	}
}

impl From<FrontendCtx> for BoxDynFrontendContext {
	fn from(value: FrontendCtx) -> Self {
		Self::from_value(value, sabi_trait::TD_Opaque)
	}
}
