use std::sync::mpsc::Receiver;

pub trait Frontend {
	fn run(&mut self, receiver: Receiver<ControlMessage>);
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ControlMessage {
	ShowOrHide,
	Show,
	Hide,
}
