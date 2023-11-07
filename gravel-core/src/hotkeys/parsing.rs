use crate::hotkeys::{Key, Modifier};
use enumflags2::BitFlags;

#[derive(Debug, PartialEq, Eq)]
pub struct ParsedBinding {
	pub modifiers: BitFlags<Modifier>,
	pub key: Key,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
	InvalidModifier,
	InvalidKey,
}

/// Parse an emacs-like keybinding. Does not support cords.
pub fn parse_binding(binding: &str) -> Result<ParsedBinding, ParseError> {
	let parts = binding.split('-').collect::<Vec<&str>>();

	let key = convert_key(parts.last().unwrap())?;
	let mut modifiers = BitFlags::empty();

	for part in &parts[0..parts.len() - 1] {
		modifiers |= convert_modifier(part)?;
	}

	Ok(ParsedBinding { modifiers, key })
}

fn convert_modifier(value: &str) -> Result<Modifier, ParseError> {
	match value {
		"A" => Ok(Modifier::Alt),
		"C" => Ok(Modifier::Control),
		"S" => Ok(Modifier::Shift),
		"M" => Ok(Modifier::Super),
		_ => Err(ParseError::InvalidModifier),
	}
}

fn convert_key(value: &str) -> Result<Key, ParseError> {
	match value.to_lowercase().as_str() {
		"a" => Ok(Key::A),
		"b" => Ok(Key::B),
		"c" => Ok(Key::C),
		"d" => Ok(Key::D),
		"e" => Ok(Key::E),
		"f" => Ok(Key::F),
		"g" => Ok(Key::G),
		"h" => Ok(Key::H),
		"i" => Ok(Key::I),
		"j" => Ok(Key::J),
		"k" => Ok(Key::K),
		"l" => Ok(Key::L),
		"m" => Ok(Key::M),
		"n" => Ok(Key::N),
		"o" => Ok(Key::O),
		"p" => Ok(Key::P),
		"q" => Ok(Key::Q),
		"r" => Ok(Key::R),
		"s" => Ok(Key::S),
		"t" => Ok(Key::T),
		"u" => Ok(Key::U),
		"v" => Ok(Key::V),
		"w" => Ok(Key::W),
		"x" => Ok(Key::X),
		"y" => Ok(Key::Y),
		"z" => Ok(Key::Z),
		"<backspace>" => Ok(Key::Backspace),
		"<tab>" => Ok(Key::Tab),
		"<enter>" => Ok(Key::Enter),
		"<caps_lock>" => Ok(Key::CapsLock),
		"<escape>" => Ok(Key::Escape),
		"<space>" => Ok(Key::Space),
		"<page_up>" => Ok(Key::PageUp),
		"<page_down>" => Ok(Key::PageDown),
		"<end>" => Ok(Key::End),
		"<home>" => Ok(Key::Home),
		"<left>" => Ok(Key::Left),
		"<right>" => Ok(Key::Right),
		"<up>" => Ok(Key::Up),
		"<down>" => Ok(Key::Down),
		"<print_screen>" => Ok(Key::PrintScreen),
		"<insert>" => Ok(Key::Insert),
		"<delete>" => Ok(Key::Delete),
		_ => Err(ParseError::InvalidKey),
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::hotkeys::{Key, Modifier};
	use enumflags2::BitFlags;

	#[test]
	fn should_parse_single_key() {
		check_binding("q", BitFlags::empty(), Key::Q);
	}

	#[test]
	fn should_parse_single_modifier() {
		check_binding("C-a", (Modifier::Control).into(), Key::A);
	}

	#[test]
	fn should_parse_multi_modifier() {
		check_binding("C-A-S-s", Modifier::Control | Modifier::Alt | Modifier::Shift, Key::S);
	}

	#[test]
	#[should_panic]
	fn should_fail() {
		check_binding("garbage in - garbage out", BitFlags::empty(), Key::A);
	}

	fn check_binding(binding: &str, modifiers: BitFlags<Modifier>, key: Key) {
		let expected = ParsedBinding { modifiers, key };

		let actual = parse_binding(binding).unwrap();
		assert_eq!(actual, expected);
	}
}
