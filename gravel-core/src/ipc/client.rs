use super::connection::{Connection, Error};
use super::{ClientMessage, ServerMessage};
use ipmb::Options;
use ipmb::{label, RecvError};
use std::time::Duration;

pub type ClientConnection = Connection<ClientMessage, ServerMessage>;

pub fn connect(name: impl Into<String>) -> Result<ClientConnection, Error> {
	let name = name.into();
	log::debug!("opening ipc client connection with name '{name}'");

	let opt = Options::new(name, label!("client"), "");
	Ok(Connection::open(opt, "server")?)
}

pub fn ping(conn: &mut ClientConnection) -> Result<bool, Error> {
	let up = match query(conn, ClientMessage::Ping) {
		Ok(ServerMessage::Ok) => true,
		Err(Error::Recv(RecvError::Timeout)) => false,
		Err(err) => Err(err)?,
	};

	Ok(up)
}

pub fn query(conn: &mut ClientConnection, msg: ClientMessage) -> Result<ServerMessage, Error> {
	conn.send(msg)?;
	Ok(conn.recv(Duration::from_millis(100))?)
}
