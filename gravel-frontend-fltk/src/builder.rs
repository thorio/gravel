use crate::config::Config;
use crate::scrollbar::Scrollbar;
use crate::structs::{Event, HitUi, Ui};
use fltk::enums::{Align, Event as FltkEvent, FrameType, Key};
use fltk::{app, app::Sender, frame::Frame, group::Group, input::Input, prelude::*, window::Window};

const WINDOW_TITLE: &str = "gravel";
const WM_CLASS: &str = "gravel";

/// Get the window's target size given the number of hits displayed.
pub fn get_window_height(config: &Config, hit_count: i32) -> i32 {
	let padding = match hit_count {
		0 => 0,
		_ => config.layout.padding,
	};

	config.layout.window_min_height + config.layout.hit_height * hit_count + padding
}

/// Constructs the UI.
pub fn build(config: &Config) -> Ui {
	let app = app::App::default().with_scheme(app::Scheme::Gtk);
	app::set_visible_focus(false);

	let (sender, receiver) = app::channel::<Event>();

	let mut window = build_window(config);
	let mut input = build_input(config);

	let do_auto_hide = config.behaviour.auto_hide;
	let sender_clone = sender.clone();
	window.handle(move |_window, event| on_window_event(event, &sender_clone, do_auto_hide));

	let sender_clone = sender.clone();
	input.handle(move |_input, event| on_input_event(event, &sender_clone));

	let hits = (0..config.layout.max_hits).map(|i| build_hit(i, config)).collect();

	let scrollbar = build_scrollbar(config);

	window.end();
	window.show();

	if config.behaviour.start_hidden {
		// HACK: hiding the window right after it's created doesn't work on linux
		// and causes high cpu usage on windows, so wait a bit and then hide it.
		let sender_clone = sender.clone();
		app::add_timeout3(0.05, move |_handle| sender_clone.send(Event::HideWindow));
	}

	Ui {
		window,
		app,
		input,
		scrollbar,
		hits,
		receiver,
		sender,
	}
}

fn build_window(config: &Config) -> Window {
	let mut window = Window::default()
		.with_size(config.layout.window_width, get_window_height(config, 0))
		.with_label(WINDOW_TITLE);

	window.set_color(config.colors.background);
	window.set_border(false);
	window.set_xclass(WM_CLASS);

	window
}

fn build_input(config: &Config) -> Input {
	let mut input = Input::default()
		.with_pos(config.layout.padding, config.layout.padding)
		.with_size(config.layout.query_width, config.layout.query_height);

	input.set_text_size(config.layout.query_font_size);
	input.set_frame(FrameType::FlatBox);
	input.set_color(config.colors.background);
	input.set_text_color(config.colors.query_text);
	input.set_selection_color(config.colors.query_highlight);
	input.set_cursor_color(config.colors.query_cursor);

	input
}

fn build_scrollbar(config: &Config) -> Scrollbar {
	Scrollbar::default()
		.with_pos(config.layout.scrollbar_x, config.layout.scrollbar_y)
		.with_size(config.layout.scrollbar_width, config.layout.scrollbar_height)
		.with_padding(config.layout.scrollbar_padding)
		.with_colors(config.colors.background, config.colors.scrollbar)
}

fn build_hit(i: i32, config: &Config) -> HitUi {
	let y = config.layout.hit_start_y + config.layout.hit_height * i;

	let mut group = Group::default()
		.with_pos(config.layout.padding, y)
		.with_size(config.layout.hit_width, config.layout.hit_height);

	group.set_color(config.colors.hit_highlight);
	group.set_frame(FrameType::FlatBox);

	let mut title = Frame::default()
		.with_pos(config.layout.padding, y)
		.with_size(config.layout.hit_width, config.layout.hit_title_height)
		.with_align(Align::BottomLeft | Align::Inside | Align::Clip);

	title.set_label_size(config.layout.hit_title_font_size);
	title.set_label_color(config.colors.hit_title);

	let mut subtitle = Frame::default()
		.with_pos(config.layout.padding, y + config.layout.hit_title_height)
		.with_size(config.layout.hit_width, config.layout.hit_subtitle_height)
		.with_align(Align::TopLeft | Align::Inside | Align::Clip);

	subtitle.set_label_size(config.layout.hit_subtitle_font_size);
	subtitle.set_label_color(config.colors.hit_subtitle);

	group.show();
	group.end();

	HitUi { group, title, subtitle }
}

fn on_window_event(event: FltkEvent, sender: &Sender<Event>, do_auto_hide: bool) -> bool {
	let message = match event {
		FltkEvent::Unfocus if do_auto_hide => Event::HideWindow,
		_ => return false,
	};

	sender.send(message);

	true
}

fn on_input_event(event: FltkEvent, sender: &Sender<Event>) -> bool {
	match event {
		FltkEvent::KeyDown | FltkEvent::Paste => on_input_keydown(app::event_key(), sender),
		_ => return false,
	};

	true
}

fn on_input_keydown(key: Key, sender: &Sender<Event>) {
	let message = match key {
		Key::Escape => Event::Cancel,
		Key::Enter | Key::KPEnter => Event::Confirm,
		Key::Up => Event::CursorUp,
		Key::Down => Event::CursorDown,
		Key::PageUp => Event::CursorPageUp,
		Key::PageDown => Event::CursorPageDown,
		Key::Home if ctrl_down() => Event::CursorTop,
		Key::End if ctrl_down() => Event::CursorBottom,
		_ => Event::Query,
	};

	sender.send(message);
}

fn ctrl_down() -> bool {
	app::event_key_down(Key::ControlL) || app::event_key_down(Key::ControlR)
}
