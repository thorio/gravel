//! Defines constants for the UI sizing and layout.

use fltk::enums::Color;
use std::*;

// sizing
pub const HIT_COUNT: i32 = 6;
pub const HIT_HEIGHT: i32 = HIT_TITLE_HEIGHT + HIT_SUBTITLE_HEIGHT;
pub const HIT_WIDTH: i32 = WINDOW_WIDTH - PADDING * 2;

pub const HIT_TITLE_HEIGHT: i32 = 30;
pub const HIT_TITLE_FONT_SIZE: i32 = 20;

pub const HIT_SUBTITLE_HEIGHT: i32 = 20;
pub const HIT_SUBTITLE_FONT_SIZE: i32 = 12;

pub const QUERY_WIDTH: i32 = HIT_WIDTH;
pub const QUERY_HEIGHT: i32 = HIT_HEIGHT;
pub const QUERY_FONT_SIZE: i32 = 25;

pub const SCROLLBAR_WIDTH: i32 = 10;
pub const SCROLLBAR_PADDING: i32 = 3;

pub const PADDING: i32 = 8;

pub const WINDOW_WIDTH: i32 = 800;

pub const COLOR_BACKGROUND: Color = Color::from_hex(0x202020);
pub const COLOR_ACCENT: Color = Color::from_hex(0xd77b0f);
pub const COLOR_TEXT: Color = Color::from_hex(0xffffff);
