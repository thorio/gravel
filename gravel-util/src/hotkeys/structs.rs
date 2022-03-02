use enumflags2::bitflags;

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Modifier {
	Alt,
	Control,
	Shift,
	Super,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Key {
	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H,
	I,
	J,
	K,
	L,
	M,
	N,
	O,
	P,
	Q,
	R,
	S,
	T,
	U,
	V,
	W,
	X,
	Y,
	Z,
	Backspace,
	Tab,
	Enter,
	CapsLock,
	Escape,
	Space,
	PageUp,
	PageDown,
	End,
	Home,
	Left,
	Right,
	Up,
	Down,
	PrintScreen,
	Insert,
	Delete,
}
