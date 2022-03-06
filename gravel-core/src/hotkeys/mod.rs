use enumflags2::BitFlags;
use std::fmt::Debug;
use std::sync::mpsc::Sender;

pub use self::{parsing::ParseError, structs::*};

mod parsing;
mod structs;

#[derive(Copy, Clone, Debug)]
struct Hotkey<T> {
	pub modifiers: BitFlags<Modifier>,
	pub key: Key,
	pub value: T,
}

/// Listens for system-wide hotkeys and sends an arbitrary value through
/// the given [`Sender`].
///
/// The listener runs in a separate thread to avoid blocking.
pub struct Listener<T: 'static + Send + Clone + Debug> {
	hotkeys: Vec<Hotkey<T>>,
}

impl<T: 'static + Send + Clone + Debug> Listener<T> {
	pub fn new() -> Self {
		Self { hotkeys: vec![] }
	}

	/// Registers a hotkey given the modifiers and key.
	pub fn register(&mut self, modifiers: BitFlags<Modifier>, key: Key, value: T) -> &mut Self {
		let hotkey = Hotkey { modifiers, key, value };

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
	pub fn register_emacs(&mut self, binding: &str, value: T) -> Result<&mut Self, ParseError> {
		let result = parsing::parse_binding(binding)?;
		self.register(result.modifiers, result.key, value);

		Ok(self)
	}

	/// Spawns the listener with the registered hotkeys.
	pub fn spawn_listener(&mut self, sender: Sender<T>) -> &mut Self {
		let hotkeys = self.hotkeys.clone();

		// run the listener on another thread to avoid blocking the current one
		std::thread::spawn(move || {
			init_hotkeys(sender, hotkeys).listen();
		});

		self
	}
}

/// Registers the given hotkeys with a new [`hotkey::Listener`] and returns it.
///
/// If a hotkey cannot be registered, a warning is logged and the hotkey is skipped.
fn init_hotkeys<T: 'static + Clone + Debug>(sender: Sender<T>, hotkeys: Vec<Hotkey<T>>) -> hotkey::Listener {
	let mut hk = hotkey::Listener::new();

	for hotkey in hotkeys {
		let sender_clone = sender.clone();
		let value_clone = hotkey.value.clone();
		let modifiers = convert_modifiers(hotkey.modifiers);
		let key = convert_key(hotkey.key);

		let result = hk.register_hotkey(modifiers, key, move || {
			sender_clone.send(value_clone.clone()).unwrap();
		});

		if let Err(_error) = result {
			println!("failed to register hotkey {:?}, skipping", hotkey);
		}
	}

	hk
}

fn convert_modifiers(modifiers: BitFlags<Modifier>) -> u32 {
	let mut result = 0;

	for modifier in modifiers.iter() {
		result = result | convert_modifier(modifier);
	}

	result
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
