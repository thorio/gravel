use enumflags2::BitFlags;
use std::fmt::Debug;

pub use self::{parsing::ParseError, structs::*};

mod parsing;
mod structs;

struct Hotkey {
	pub modifiers: BitFlags<Modifier>,
	pub key: Key,
	pub callback: Box<dyn Fn() + 'static + Send>,
}

impl Debug for Hotkey {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Hotkey")
			.field("modifiers", &self.modifiers)
			.field("key", &self.key)
			.field("value", &"Box<dyn FnMut>")
			.finish()
	}
}

/// Listens for system-wide hotkeys and calls closures.
///
/// The listener runs in a separate thread to avoid blocking.
#[derive(Default)]
pub struct Listener {
	hotkeys: Vec<Hotkey>,
}

impl Listener {
	/// Registers a hotkey given the modifiers and key.
	pub fn register(&mut self, modifiers: BitFlags<Modifier>, key: Key, f: impl Fn() + 'static + Send) -> &mut Self {
		let hotkey = Hotkey {
			modifiers,
			key,
			callback: Box::from(f),
		};

		self.hotkeys.push(hotkey);

		self
	}

	/// Registers a hotkey given an emacs-like binding.
	///
	/// Examples:
	/// - `A-<Space>` => Alt + Space
	/// - `C-M-s` => Control + Super/Windows + S
	/// - `a` => A
	///
	/// For a complete list of keys and modifiers, see [`Key`], [`Modifier`]
	pub fn register_emacs(&mut self, binding: &str, f: impl Fn() + 'static + Send) -> Result<&mut Self, ParseError> {
		let result = parsing::parse_binding(binding)?;
		self.register(result.modifiers, result.key, f);

		Ok(self)
	}

	/// Spawns the listener with the registered hotkeys.
	pub fn spawn_listener(self) {
		// run the listener on another thread to avoid blocking the current one
		std::thread::spawn(move || {
			log::trace!("starting hotkey listener thread");
			init_hotkeys(self.hotkeys).listen();
		});
	}
}

/// Registers the given hotkeys with a new [`hotkey::Listener`] and returns it.
///
/// If a hotkey cannot be registered, a warning is logged and the hotkey is skipped.
fn init_hotkeys(hotkeys: Vec<Hotkey>) -> hotkey::Listener {
	let mut hk = hotkey::Listener::new();

	for hotkey in hotkeys {
		let modifiers = hotkey.modifiers.iter().fold(0, |r, v| r | convert_modifier(v));
		let key = convert_key(hotkey.key);

		hk.register_hotkey(modifiers, key, move || (hotkey.callback)())
			.inspect_err(|e| log::warn!("failed to register hotkey {:?}: {e}", (hotkey.key, hotkey.modifiers)))
			.ok();
	}

	hk
}

fn convert_modifier(value: Modifier) -> u32 {
	match value {
		Modifier::Alt => hotkey::modifiers::ALT,
		Modifier::Control => hotkey::modifiers::CONTROL,
		Modifier::Shift => hotkey::modifiers::SHIFT,
		Modifier::Super => hotkey::modifiers::SUPER,
	}
}

fn convert_key(value: Key) -> u32 {
	match value {
		Key::A => 'A' as u32,
		Key::B => 'B' as u32,
		Key::C => 'C' as u32,
		Key::D => 'D' as u32,
		Key::E => 'E' as u32,
		Key::F => 'F' as u32,
		Key::G => 'G' as u32,
		Key::H => 'H' as u32,
		Key::I => 'I' as u32,
		Key::J => 'J' as u32,
		Key::K => 'K' as u32,
		Key::L => 'L' as u32,
		Key::M => 'M' as u32,
		Key::N => 'N' as u32,
		Key::O => 'O' as u32,
		Key::P => 'P' as u32,
		Key::Q => 'Q' as u32,
		Key::R => 'R' as u32,
		Key::S => 'S' as u32,
		Key::T => 'T' as u32,
		Key::U => 'U' as u32,
		Key::V => 'V' as u32,
		Key::W => 'W' as u32,
		Key::X => 'X' as u32,
		Key::Y => 'Y' as u32,
		Key::Z => 'Z' as u32,
		Key::Backspace => hotkey::keys::BACKSPACE,
		Key::Tab => hotkey::keys::TAB,
		Key::Enter => hotkey::keys::ENTER,
		Key::CapsLock => hotkey::keys::CAPS_LOCK,
		Key::Escape => hotkey::keys::ESCAPE,
		Key::Space => hotkey::keys::SPACEBAR,
		Key::PageUp => hotkey::keys::PAGE_UP,
		Key::PageDown => hotkey::keys::PAGE_DOWN,
		Key::End => hotkey::keys::END,
		Key::Home => hotkey::keys::HOME,
		Key::Left => hotkey::keys::ARROW_LEFT,
		Key::Right => hotkey::keys::ARROW_RIGHT,
		Key::Up => hotkey::keys::ARROW_UP,
		Key::Down => hotkey::keys::ARROW_DOWN,
		Key::PrintScreen => hotkey::keys::PRINT_SCREEN,
		Key::Insert => hotkey::keys::INSERT,
		Key::Delete => hotkey::keys::DELETE,
	}
}
