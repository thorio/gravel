use std::sync::mpsc::Receiver;

pub trait Frontend {
	fn run(&mut self, receiver: Receiver<FrontendMessage>);
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FrontendMessage {
	ShowOrHide,
	Show,
	Hide,
}
