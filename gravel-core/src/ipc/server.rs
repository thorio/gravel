use super::connection::{Connection, Error};
use super::{ClientMessage, ServerMessage};
use crate::CoreMessage;
use abi_stable::external_types::crossbeam_channel::RSender;
use gravel_ffi::FrontendMessage;
use ipmb::RecvError;
use ipmb::{label, Options};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::{sync::Arc, time::Duration};

pub struct Server {
	conn: Connection<ServerMessage, ClientMessage>,
	sender: RSender<CoreMessage>,
	stop_signal: Arc<AtomicBool>,
}

impl Server {
	pub fn start(name: impl Into<String>, sender: RSender<CoreMessage>) -> Result<Self, Error> {
		let name = name.into();
		log::debug!("starting ipc server with name '{name}'");

		let opt = Options::new(name, label!("server"), "");
		let conn = Connection::open(opt, "client")?;
		let stop_signal = Arc::from(AtomicBool::new(false));

		Ok(Self {
			conn,
			sender,
			stop_signal,
		})
	}

	pub fn spawn(mut self) -> ServerHandle {
		let signal_clone = self.stop_signal.clone();

		let handle = thread::spawn(move || {
			log::trace!("ipc server listening");
			self.listen();
			log::trace!("stopping ipc server");
		});

		ServerHandle::new(signal_clone, handle)
	}

	fn listen(&mut self) {
		loop {
			if self.should_stop() {
				return;
			}

			self.try_recv()
				.inspect_err(|e| log::error!("ipc receive error: {e}"))
				.ok();
		}
	}

	fn try_recv(&mut self) -> Result<(), Error> {
		const ONE_MILLI: Duration = Duration::from_millis(1);

		let query = match self.conn.recv(ONE_MILLI) {
			Ok(query) => query,
			Err(RecvError::Timeout) => return Ok(()),
			Err(err) => Err(err)?,
		};

		let res = self.handle_message(query);
		self.conn.send(res)?;

		Ok(())
	}

	fn handle_message(&self, msg: ClientMessage) -> ServerMessage {
		match msg {
			ClientMessage::Ping => ServerMessage::Ok,
			ClientMessage::Show => self.send_frontend(FrontendMessage::Show),
			ClientMessage::Hide => self.send_frontend(FrontendMessage::Hide),
		}
	}

	fn send_frontend(&self, msg: FrontendMessage) -> ServerMessage {
		self.send_core(CoreMessage::Frontend(msg))
	}

	fn send_core(&self, msg: CoreMessage) -> ServerMessage {
		self.sender
			.send(msg)
			.inspect_err(|e| log::error!("unable to send core message: {e}"))
			.ok();

		ServerMessage::Ok
	}

	fn should_stop(&self) -> bool {
		self.stop_signal.load(Ordering::Relaxed)
	}
}

pub struct ServerHandle {
	signal: Arc<AtomicBool>,
	handle: JoinHandle<()>,
}

impl ServerHandle {
	fn new(signal: Arc<AtomicBool>, handle: JoinHandle<()>) -> Self {
		Self { signal, handle }
	}

	#[allow(clippy::missing_panics_doc)]
	pub fn quit(self) {
		self.signal.fetch_or(true, Ordering::Relaxed);
		self.handle.join().expect("server thread must never panic");
	}
}
