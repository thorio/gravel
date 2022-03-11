use crate::{builder, config::*, native, scroll::Scroll, structs::*};
use fltk::{enums::*, prelude::*};
use gravel_core::*;
use lazy_static::*;
use std::sync::mpsc::Receiver;

lazy_static! {
	static ref EMPTY_HIT: HitData = HitData::empty();
}

pub struct DefaultFrontend {
	config: Config,
	ui: Ui,
	engine: QueryEngine,
	result: QueryResult,
	scroll: Scroll,
	visible: bool,
}

impl Frontend for DefaultFrontend {
	fn run(&mut self, receiver: Receiver<FrontendMessage>) {
		self.handle_frontend_messages(receiver);
		self.run_event_loop();
	}
}

impl DefaultFrontend {
	pub fn new(engine: QueryEngine, config: Config) -> Self {
		let ui = builder::build(&config);
		let max_view_size = config.layout.max_hits;

		DefaultFrontend {
			config: config,
			engine: engine,
			ui: ui,
			result: QueryResult::empty(),
			scroll: Scroll::new(0, max_view_size),
			visible: false,
		}
	}

	/// Runs the FLTK event loop. Blocks until the app exits.
	fn run_event_loop(&mut self) {
		while self.ui.app.wait() {
			if let Some(message) = self.ui.receiver.recv() {
				match message {
					Message::Query => self.query(),
					Message::Confirm => self.confirm(),
					Message::CursorUp => self.cursor_up(),
					Message::CursorDown => self.cursor_down(),
					Message::CursorPageUp => self.cursor_page_up(),
					Message::CursorPageDown => self.cursor_page_down(),
					Message::CursorTop => self.cursor_top(),
					Message::CursorBottom => self.cursor_bottom(),
					Message::Cancel => self.hide(),
					Message::ShowWindow => self.show(),
					Message::HideWindow => self.hide(),
					Message::ShowOrHideWindow => self.show_or_hide(),
					Message::ShowWithQuery(query) => self.show_with(&query),
					Message::Exit => break,
				}
			}
		}
	}

	/// Registers a recurring timeout that forwards [`FrontendMessage`]s on
	/// the given [`Receiver`] to the frontend's own channel.
	fn handle_frontend_messages(&mut self, receiver: Receiver<FrontendMessage>) {
		let own_sender = self.ui.sender.clone();

		fltk::app::add_timeout3(0.01, move |handle| {
			if let Ok(message) = receiver.try_recv() {
				own_sender.send(convert_message(message));
			}

			fltk::app::repeat_timeout3(0.01, handle);
		});
	}

	fn show_or_hide(&mut self) {
		if self.visible {
			self.hide();
		} else {
			self.show();
		}
	}

	fn hide(&mut self) {
		self.ui.window.platform_hide();
		self.visible = false;
	}

	fn show(&mut self) {
		// select the entire previous query so it is overwritten when the user starts typing
		self.input_select_all();

		self.ui.window.platform_show();
		self.visible = true;

		// pull the window into the foreground so it isn't stuck behind other windows
		native::activate_window(&self.ui.window);
	}

	/// Shows the window and populates the input with the given query.
	fn show_with(&mut self, query: &str) {
		self.show();
		self.ui.input.set_value(query);
		self.query_force();
	}

	fn input_select_all(&mut self) {
		let length = self.ui.input.value().chars().count() as i32;

		if length > 0 {
			self.ui.input.set_position(0).unwrap();
			self.ui.input.set_mark(length).unwrap();
		}
	}

	/// Queries the [`QueryEngine`] if the input has changed.
	fn query(&mut self) {
		if self.ui.input.changed() {
			self.query_force();
		}
	}

	/// Queries the [`QueryEngine`].
	fn query_force(&mut self) {
		self.result = self.engine.query(&self.ui.input.value());
		self.ui.input.clear_changed();

		self.update_window_height();
		self.update_hits();
	}

	/// Runs the action of the selected hit.
	fn confirm(&self) {
		if self.result.hits.len() >= 1 {
			let cursor = self.scroll.cursor();
			let hit = &self.result.hits[cursor as usize];
			self.engine.run_hit_action(&hit);
		}
	}

	fn cursor_up(&mut self) {
		self.scroll.cursor_up();
		self.update_hits();
	}

	fn cursor_down(&mut self) {
		self.scroll.cursor_down();
		self.update_hits();
	}

	fn cursor_page_up(&mut self) {
		self.scroll.page_up();
		self.update_hits();
	}

	fn cursor_page_down(&mut self) {
		self.scroll.page_down();
		self.update_hits();
	}

	fn cursor_top(&mut self) {
		self.scroll.top();
		self.update_hits();
	}

	fn cursor_bottom(&mut self) {
		self.scroll.bottom();
		self.update_hits();
	}

	/// Writes the hit data to the UI elements.
	fn update_hits(&mut self) {
		for i in 0..self.config.layout.max_hits {
			let position = self.scroll.scroll() + i;
			let selected = position == self.scroll.cursor();

			let hitdata = if position < self.result.hits.len() as i32 {
				self.result.hits[position as usize].get_data()
			} else {
				// hits offscreen must be set to empty, otherwise any residual data might still be visible.
				&EMPTY_HIT
			};

			let hit = &mut self.ui.hits[i as usize];
			update_hit(hit, hitdata, selected);
		}

		self.update_scrollbar();
	}

	/// Writes scroll data to the scrollbar.
	fn update_scrollbar(&mut self) {
		// only show the scrollbar if there is something to scroll
		if self.scroll.view_size() >= self.result.hits.len() as i32 {
			self.ui.scrollbar.hide();
		} else {
			self.ui.scrollbar.show();
		}

		let pos = self.scroll.scroll() as f32 / self.scroll.length() as f32;
		let size = self.scroll.view_size() as f32 / self.scroll.length() as f32;
		self.ui.scrollbar.set_slider_position(pos);
		self.ui.scrollbar.set_slider_size(size);
	}

	/// Sets the new size on its [`Scroll`] and updates the window's height.
	fn update_window_height(&mut self) {
		self.scroll.set_length(self.result.hits.len() as i32);
		let height = builder::get_window_size(&self.config, self.scroll.view_size());
		self.ui.window.set_size(self.config.layout.window_width, height);
	}
}

fn convert_message(message: FrontendMessage) -> Message {
	match message {
		FrontendMessage::ShowOrHide => Message::ShowOrHideWindow,
		FrontendMessage::Show => Message::ShowWindow,
		FrontendMessage::Hide => Message::HideWindow,
		FrontendMessage::ShowWithQuery(query) => Message::ShowWithQuery(query),
		FrontendMessage::Exit => Message::Exit,
	}
}

/// Writes the given [`HitData`] to the given [`HitUi`].
///
/// `selected` highlights the hit.
fn update_hit(hit: &mut HitUi, data: &HitData, selected: bool) {
	hit.title.set_label(&data.title);
	hit.subtitle.set_label(&data.subtitle);

	match selected {
		true => hit.group.set_frame(FrameType::FlatBox),
		false => hit.group.set_frame(FrameType::NoBox),
	}
	hit.group.set_damage(true);
}
