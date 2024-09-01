use ipmb::{EndpointReceiver, EndpointSender, Message, MessageBox, Options, Selector};
use ipmb::{JoinError, RecvError, SendError};
use serde::{de::DeserializeOwned, Serialize};
use std::{borrow::Cow, time::Duration};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("send: {0}")]
	Send(#[from] SendError),
	#[error("recv: {0}")]
	Recv(#[from] RecvError),
	#[error("join: {0}")]
	Join(#[from] JoinError),
}

pub struct Connection<S: Serialize + MessageBox, R: DeserializeOwned + MessageBox> {
	send: EndpointSender<S>,
	recv: EndpointReceiver<R>,
	target: Cow<'static, str>,
}

impl<S: Serialize + MessageBox, R: DeserializeOwned + MessageBox> Connection<S, R> {
	pub fn open(opt: Options, target: impl Into<Cow<'static, str>>) -> Result<Self, JoinError> {
		let (send, recv) = ipmb::join(opt, None)?;

		Ok(Self {
			send,
			recv,
			target: target.into(),
		})
	}

	pub fn send(&mut self, msg: S) -> Result<(), SendError> {
		let selector = Selector::unicast(&self.target);
		let message = Message::new(selector, msg);

		self.send.send(message)
	}

	pub fn recv(&mut self, timeout: Duration) -> Result<R, RecvError> {
		self.recv.recv(Some(timeout)).map(|m| m.payload)
	}
}
