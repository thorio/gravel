use std::sync::Mutex;

pub struct Clipboard {
	inner: Option<Mutex<arboard::Clipboard>>,
}

impl Clipboard {
	#[expect(clippy::new_without_default)]
	pub fn new() -> Self {
		Self {
			inner: create_clipboard().map(Mutex::new),
		}
	}

	pub fn set_text(&self, content: &str) {
		log::trace!("setting clipboard to {content}");

		let Some(mutex) = &self.inner else {
			log::trace!("clipboard not initialized, canceling operation");
			return;
		};

		let Ok(mut clipboard) = mutex.lock() else {
			log::error!("clipboard mutex poisened, canceling operation");
			return;
		};

		clipboard
			.set_text(content)
			.inspect_err(|e| log::error!("unable to set clipboard: {e}"))
			.ok();
	}
}

fn create_clipboard() -> Option<arboard::Clipboard> {
	log::trace!("spawning clipboard instance");

	arboard::Clipboard::new()
		.inspect_err(|e| log::error!("unable to initialize clipboard: {e}"))
		.ok()
}
