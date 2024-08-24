//! gravel's core library.
//!
//! Contains functionality used internally by gravel, and is generally not
//! required for writing plugins.

use abi_stable::external_types::crossbeam_channel::{RReceiver, RSender};
use abi_stable::sabi_trait;
use abi_stable::std_types::{ROption, RString};
use abi_stable::traits::{IntoReprC, IntoReprRust};
use engine::QueryEngine;
use gravel_ffi::{clone_hit_arc, ActionKind, ArcDynHit};
use gravel_ffi::{BoxDynFrontendContext, FrontendContext, FrontendMessage, FrontendMessageNe};
use std::sync::atomic::{AtomicU32, Ordering};
use std::{thread, time::Duration};

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
	RunAction(ArcDynHit, ActionKind),
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

	pub fn spawn(self) {
		thread::spawn(move || self.run());
	}

	pub fn run(&self) {
		loop {
			self.receive_message();
		}
	}

	fn receive_message(&self) -> Option<()> {
		const ONE_MILLI: Duration = Duration::from_millis(1);

		match self.receiver.recv_timeout(ONE_MILLI).ok()? {
			CoreMessage::Frontend(msg) => self.send_frontend(msg),
			CoreMessage::Query(token, query) => self.query(token, &query),
			CoreMessage::RunAction(hit, kind) => self.run_action(&hit, kind),
			CoreMessage::ClearCaches => self.clear_caches(),
		}

		None
	}

	fn run_action(&self, hit: &ArcDynHit, kind: ActionKind) {
		timed!("hit action took", {
			self.engine.run_hit_action(hit, kind);
		});
	}

	fn query(&self, token: u32, query: &str) {
		let result = timed!(("full query {token} took"), { self.engine.query(query.into_c()) });

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
	receiver: RReceiver<FrontendMessageNe>,
	sender: RSender<CoreMessage>,
	token_counter: AtomicU32,
}

impl FrontendCtx {
	pub fn new(receiver: RReceiver<FrontendMessageNe>, sender: RSender<CoreMessage>) -> Self {
		Self {
			receiver,
			sender,
			token_counter: Default::default(),
		}
	}

	fn send(&self, message: CoreMessage) {
		self.sender
			.send(message)
			.inspect_err(|e| log::error!("unable to send core message: {e}"))
			.ok();
	}

	fn new_token(&self) -> u32 {
		self.token_counter.fetch_add(1, Ordering::Relaxed)
	}
}

impl FrontendContext for FrontendCtx {
	fn recv_raw(&self) -> ROption<FrontendMessageNe> {
		self.receiver.try_recv().ok().into_c()
	}

	fn query(&self, query: RString) -> u32 {
		let token = self.new_token();
		self.send(CoreMessage::Query(token, query.into_rust()));
		token
	}

	fn run_hit_action(&self, hit: &ArcDynHit, kind: ActionKind) {
		self.send(CoreMessage::RunAction(clone_hit_arc(hit), kind));
	}
}

impl From<FrontendCtx> for BoxDynFrontendContext {
	fn from(value: FrontendCtx) -> Self {
		Self::from_value(value, sabi_trait::TD_Opaque)
	}
}
