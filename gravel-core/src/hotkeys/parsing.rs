use crate::hotkeys::{Key, Modifier};
use enumflags2::BitFlags;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct ParsedBinding {
	pub modifiers: BitFlags<Modifier>,
	pub key: Key,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseError {
	#[error("'{0}' is not a valid modifier")]
	InvalidModifier(String),
	#[error("'{0}' is not a valid key")]
	InvalidKey(String),
	#[error("'{0}' is a valid modifier, but is used in place of a key")]
	ModifierUsedAsKey(String),
	#[error("binding is empty")]
	Empty,
}

/// Parse an emacs-like keybinding. Does not support cords.
pub fn parse_binding(binding: &str) -> Result<ParsedBinding, ParseError> {
	if binding.is_empty() {
		return Err(ParseError::Empty);
	}

	let parts = binding.split('-').collect::<Vec<&str>>();

	let key = convert_key(parts.last().expect("vec always contains at least one item"))?;
	let mut modifiers = BitFlags::empty();

	for part in &parts[0..parts.len() - 1] {
		modifiers |= convert_modifier(part)?;
	}

	Ok(ParsedBinding { modifiers, key })
}

fn convert_modifier(value: &str) -> Result<Modifier, ParseError> {
	let modifier = match value {
		"A" => Modifier::Alt,
		"C" => Modifier::Control,
		"S" => Modifier::Shift,
		"M" => Modifier::Super,
		_ => return Err(ParseError::InvalidModifier(value.to_owned())),
	};

	Ok(modifier)
}

fn convert_key(value: &str) -> Result<Key, ParseError> {
	if convert_modifier(value).is_ok() {
		return Err(ParseError::ModifierUsedAsKey(value.to_owned()));
	}

	let key = match value {
		"a" => Key::A,
		"b" => Key::B,
		"c" => Key::C,
		"d" => Key::D,
		"e" => Key::E,
		"f" => Key::F,
		"g" => Key::G,
		"h" => Key::H,
		"i" => Key::I,
		"j" => Key::J,
		"k" => Key::K,
		"l" => Key::L,
		"m" => Key::M,
		"n" => Key::N,
		"o" => Key::O,
		"p" => Key::P,
		"q" => Key::Q,
		"r" => Key::R,
		"s" => Key::S,
		"t" => Key::T,
		"u" => Key::U,
		"v" => Key::V,
		"w" => Key::W,
		"x" => Key::X,
		"y" => Key::Y,
		"z" => Key::Z,
		_ => match value.to_lowercase().as_str() {
			"<backspace>" => Key::Backspace,
			"<tab>" => Key::Tab,
			"<enter>" => Key::Enter,
			"<caps_lock>" => Key::CapsLock,
			"<escape>" => Key::Escape,
			"<space>" => Key::Space,
			"<page_up>" => Key::PageUp,
			"<page_down>" => Key::PageDown,
			"<end>" => Key::End,
			"<home>" => Key::Home,
			"<left>" => Key::Left,
			"<right>" => Key::Right,
			"<up>" => Key::Up,
			"<down>" => Key::Down,
			"<print_screen>" => Key::PrintScreen,
			"<insert>" => Key::Insert,
			"<delete>" => Key::Delete,
			_ => return Err(ParseError::InvalidKey(value.to_owned())),
		},
	};

	Ok(key)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::hotkeys::{Key, Modifier};
	use enumflags2::BitFlags;
	use rstest::rstest;

	#[rstest]
	#[case("q", ParsedBinding {modifiers: BitFlags::empty(), key: Key::Q})]
	#[case("C-a", ParsedBinding {modifiers: Modifier::Control.into(), key: Key::A})]
	#[case("C-A-S-s", ParsedBinding {modifiers: Modifier::Control | Modifier::Alt | Modifier::Shift, key: Key::S})]
	#[case("M-S-<PRINT_screen>", ParsedBinding {modifiers: Modifier::Super | Modifier::Shift, key: Key::PrintScreen})]
	fn should_parse(#[case] binding: &str, #[case] expected: ParsedBinding) {
		let actual = parse_binding(binding);
		assert_eq!(actual, Ok(expected));
	}

	#[rstest]
	#[case("not- working", ParseError::InvalidKey(String::from(" working")))]
	#[case("Z", ParseError::InvalidKey(String::from("Z")))]
	#[case("c-d", ParseError::InvalidModifier(String::from("c")))]
	#[case("C-S", ParseError::ModifierUsedAsKey(String::from("S")))]
	#[case("", ParseError::Empty)]
	fn should_err(#[case] binding: &str, #[case] expected: ParseError) {
		let actual = parse_binding(binding);
		assert_eq!(actual, Err(expected));
	}
}
