use crate::{constants::*, scrollbar::Scrollbar, structs::*};
use fltk::{
	app, app::Sender, enums::*, frame::Frame, group::Group, input::Input, prelude::*,
	window::Window,
};

pub fn get_window_size(hit_count: i32) -> i32 {
	// when there are no results, we don't need padding between the input and the result list
	let padding = PADDING * if hit_count == 0 { 2 } else { 3 };

	HIT_HEIGHT * hit_count + QUERY_HEIGHT + padding
}

pub fn build() -> Ui {
	let app = app::App::default().with_scheme(app::Scheme::Gtk);
	app::set_visible_focus(false);

	let (sender, receiver) = app::channel::<Message>();

	let mut window = build_window();
	let mut input = build_input();

	input.handle(move |input, event| input_event(input, event, &sender));

	let mut hits = Vec::new();
	for i in 0..HIT_COUNT {
		hits.push(build_hit(i));
	}

	let scollbar = build_scrollbar();

	window.end();
	window.show();

	Ui {
		window: window,
		app: app,
		input: input,
		scrollbar: scollbar,
		hits: hits,
		receiver: receiver,
	}
}

fn build_window() -> Window {
	let mut window = Window::default()
		.with_size(WINDOW_WIDTH, get_window_size(HIT_COUNT))
		.center_screen()
		.with_label("Gravel");
	window.set_color(COLOR_BACKGROUND);
	// window.set_border(false);

	// change the height after the window has been centered
	window.set_size(WINDOW_WIDTH, get_window_size(0));

	window
}

fn build_input() -> Input {
	let mut input = Input::default()
		.with_pos(PADDING, PADDING)
		.with_size(QUERY_WIDTH, QUERY_HEIGHT);
	input.set_text_size(QUERY_FONT_SIZE);
	input.set_frame(FrameType::FlatBox);
	input.set_color(COLOR_BACKGROUND);
	input.set_text_color(COLOR_TEXT);
	input.set_selection_color(COLOR_ACCENT);
	// TODO fix cursor color
	// input.set_cursor_color(COLOR_TEXT);

	input
}

fn build_scrollbar() -> Scrollbar {
	Scrollbar::default()
		.with_pos(WINDOW_WIDTH - PADDING - SCROLLBAR_WIDTH, PADDING * 2 + QUERY_HEIGHT)
		.with_size(SCROLLBAR_WIDTH, HIT_HEIGHT * HIT_COUNT)
		.with_padding(SCROLLBAR_PADDING)
		.with_colors(COLOR_BACKGROUND, COLOR_ACCENT)
}

fn build_hit(i: i32) -> HitUi {
	let x = PADDING;
	let y = PADDING * 2 + QUERY_HEIGHT + HIT_HEIGHT * i;

	let mut group = Group::default()
		.with_pos(x, y)
		.with_size(HIT_WIDTH, HIT_HEIGHT);
	group.set_color(COLOR_ACCENT);
	group.set_frame(FrameType::FlatBox);

	let mut title = Frame::default()
		.with_pos(x, y)
		.with_size(HIT_WIDTH - HIT_HEIGHT, HIT_TITLE_HEIGHT)
		.with_align(Align::BottomLeft | Align::Inside | Align::Wrap);
	title.set_label_size(HIT_TITLE_FONT_SIZE);
	title.set_label_color(COLOR_TEXT);

	let mut subtitle = Frame::default()
		.with_pos(x, y + HIT_TITLE_HEIGHT)
		.with_size(HIT_WIDTH - HIT_HEIGHT, HIT_SUBTITLE_HEIGHT)
		.with_align(Align::TopLeft | Align::Inside | Align::Wrap);
	subtitle.set_label_size(HIT_SUBTITLE_FONT_SIZE);
	subtitle.set_label_color(COLOR_TEXT);

	group.show();
	group.end();

	HitUi {
		group: group,
		title: title,
		subtitle: subtitle,
	}
}

fn input_event(input: &mut Input, event: Event, sender: &Sender<Message>) -> bool {
	match event {
		Event::KeyDown => input_keydown(input, app::event_key(), sender),
		_ => false,
	}
}

fn input_keydown(_input: &mut Input, key: Key, sender: &Sender<Message>) -> bool {
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
