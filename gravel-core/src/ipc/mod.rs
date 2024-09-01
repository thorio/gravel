use crate::config;
use crate::env;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use type_uuid::TypeUuid;

pub mod client;
pub mod connection;
pub mod server;

#[derive(Deserialize, Serialize, Debug, TypeUuid)]
#[uuid = "5ba5c971-b816-4e59-b236-d9ab963fc3af"]
pub enum ClientMessage {
	Ping,
	Show,
	Hide,
}

#[derive(Deserialize, Serialize, Debug, TypeUuid)]
#[uuid = "b99156d9-0718-4697-a954-ec1d188cb636"]
pub enum ServerMessage {
	Ok,
}

pub fn get_name(config: &config::Ipc) -> Cow<str> {
	if config.append_display {
		let display = env::display().unwrap_or_default();
		return Cow::Owned(format!("{}-{}", config.name, display.to_string_lossy()));
	}

	Cow::Borrowed(&config.name)
}
