use crate::{config::*, scrollbar::Scrollbar, structs::*};
use fltk::{app, app::Sender, enums::*, frame::Frame, group::Group, input::Input, prelude::*, window::Window};

/// Get the target window's size given the number of hits displayed.
pub fn get_window_size(config: &Config, hit_count: i32) -> i32 {
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

	let (sender, receiver) = app::channel::<Message>();

	let mut window = build_window(config);
	let mut input = build_input(config);

	let mut sender_clone = sender.clone();
	window.handle(move |_window, event| window_event(event, &sender_clone));

	sender_clone = sender.clone();
	input.handle(move |_input, event| input_event(event, &sender_clone));

	let mut hits = Vec::new();
	for i in 0..config.layout.max_hits {
		hits.push(build_hit(i, config));
	}

	let scollbar = build_scrollbar(config);

	window.end();
	window.show();
	window.platform_hide();

	Ui {
		window,
		app,
		input,
		scrollbar: scollbar,
		hits,
		receiver,
		sender,
	}
}

fn build_window(config: &Config) -> Window {
	let mut window = Window::default()
		.with_size(
			config.layout.window_width,
			get_window_size(config, config.layout.max_hits),
		)
		.center_screen()
		.with_label("Gravel");
	window.set_color(config.colors.background);
	window.set_border(false);

	// change the height after the window has been centered
	window.set_size(config.layout.window_width, get_window_size(config, 0));

	window
}

fn build_input(config: &Config) -> Input {
	let mut input = Input::default()
		.with_pos(config.layout.padding, config.layout.padding)
		.with_size(config.layout.query_width, config.layout.query_height);
	input.set_text_size(config.layout.query_font_size);
	input.set_frame(FrameType::FlatBox);
	input.set_color(config.colors.background);
	input.set_text_color(config.colors.text);
	input.set_selection_color(config.colors.accent);
	// TODO fix cursor color
	// input.set_cursor_color(COLOR_TEXT);

	input
}

fn build_scrollbar(config: &Config) -> Scrollbar {
	Scrollbar::default()
		.with_pos(config.layout.scrollbar_x, config.layout.scrollbar_y)
		.with_size(config.layout.scrollbar_width, config.layout.scrollbar_height)
		.with_padding(config.layout.scrollbar_padding)
		.with_colors(config.colors.background, config.colors.accent)
}

fn build_hit(i: i32, config: &Config) -> HitUi {
	let y = config.layout.hit_start_y + config.layout.hit_height * i;

	let mut group = Group::default()
		.with_pos(config.layout.padding, y)
		.with_size(config.layout.hit_width, config.layout.hit_height);
	group.set_color(config.colors.accent);
	group.set_frame(FrameType::FlatBox);

	let mut title = Frame::default()
		.with_pos(config.layout.padding, y)
		.with_size(config.layout.hit_width, config.layout.hit_title_height)
		.with_align(Align::BottomLeft | Align::Inside | Align::Clip);
	title.set_label_size(config.layout.hit_title_font_size);
	title.set_label_color(config.colors.text);

	let mut subtitle = Frame::default()
		.with_pos(config.layout.padding, y + config.layout.hit_title_height)
		.with_size(config.layout.hit_width, config.layout.hit_subtitle_height)
		.with_align(Align::TopLeft | Align::Inside | Align::Clip);
	subtitle.set_label_size(config.layout.hit_subtitle_font_size);
	subtitle.set_label_color(config.colors.text);

	group.show();
	group.end();

	HitUi { group, title, subtitle }
}

/// Handles events on the window.
fn window_event(event: Event, sender: &Sender<Message>) -> bool {
	match event {
		Event::Unfocus => window_unfocus(sender),
		_ => false,
	}
}

fn window_unfocus(sender: &Sender<Message>) -> bool {
	sender.send(Message::HideWindow);

	true
}

/// Handles events on the input.
fn input_event(event: Event, sender: &Sender<Message>) -> bool {
	match event {
		Event::KeyDown => input_keydown(app::event_key(), sender),
		_ => false,
	}
}

fn input_keydown(key: Key, sender: &Sender<Message>) -> bool {
	let message = match key {
		Key::Escape => Message::Cancel,
		Key::Enter | Key::KPEnter => Message::Confirm,
		Key::Up => Message::CursorUp,
		Key::Down => Message::CursorDown,
		Key::PageUp => Message::CursorPageUp,
		Key::PageDown => Message::CursorPageDown,
		Key::Home if ctrl_down() => Message::CursorTop,
		Key::End if ctrl_down() => Message::CursorBottom,
		_ => Message::Query,
	};

	sender.send(message);

	true
}

fn ctrl_down() -> bool {
	app::event_key_down(Key::ControlL) || app::event_key_down(Key::ControlR)
}
