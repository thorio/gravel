use crate::{builder, config::*, native, scroll::Scroll, structs::*};
use fltk::{enums::*, prelude::*};
use gravel_core::{scoring::ScoredHit, *};
use std::sync::mpsc::Receiver;

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
		self.update_window_position();
		self.run_event_loop();
	}
}

impl DefaultFrontend {
	pub fn new(engine: QueryEngine, config: Config) -> Self {
		let ui = builder::build(&config);
		let max_view_size = config.layout.max_hits;

		Self {
			config,
			engine,
			ui,
			result: QueryResult::empty(),
			scroll: Scroll::new(0, max_view_size),
			visible: false,
		}
	}

	/// Runs the FLTK event loop. Blocks until the app exits.
	fn run_event_loop(&mut self) {
		while self.ui.app.wait() {
			let Some(message) = self.ui.receiver.recv() else { continue };

			match message {
				Message::Query => self.query(),
				Message::Confirm => self.confirm(),
				Message::CursorUp => self.cursor_up(),
				Message::CursorDown => self.cursor_down(),
				Message::CursorPageUp => self.cursor_page_up(),
				Message::CursorPageDown => self.cursor_page_down(),
				Message::CursorTop => self.cursor_top(),
				Message::CursorBottom => self.cursor_bottom(),
				Message::ShowWindow => self.show(),
				Message::Cancel | Message::HideWindow => self.hide(),
				Message::ShowOrHideWindow => self.show_or_hide(),
				Message::ShowWithQuery(query) => self.show_with(&query),
				Message::Exit => self.ui.app.quit(),
			}
		}
	}

	/// Registers a recurring timeout that forwards [`FrontendMessage`]s on
	/// the given [`Receiver`] to the frontend's own channel.
	fn handle_frontend_messages(&mut self, receiver: Receiver<FrontendMessage>) {
		let own_sender = self.ui.sender.clone();

		fltk::app::add_timeout3(0.01, move |handle| {
			if let Ok(message) = receiver.try_recv() {
				own_sender.send(message.into());
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
		self.update_window_position();

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
		if !self.result.hits.is_empty() {
			let cursor = self.scroll.cursor();
			let hit = &self.result.hits[cursor as usize];
			self.engine.run_hit_action(&*hit.hit);
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

			let hit_ui = self.ui.hits.get_mut(i as usize).unwrap();
			let hit = self.result.hits.get(position as usize);
			update_hit(hit_ui, hit, selected, self.config.behaviour.show_scores);
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
		let height = builder::get_window_height(&self.config, self.scroll.view_size());
		self.ui.window.set_size(self.config.layout.window_width, height);
	}

	fn update_window_position(&mut self) {
		if !self.config.behaviour.auto_center_window {
			return;
		}

		let width = self.config.layout.window_width;
		let max_height = builder::get_window_height(&self.config, self.config.layout.max_hits);

		let (screen_width, screen_height) = fltk::app::screen_size();

		let pos_x = (screen_width as i32 - width) / 2;
		let pos_y = (screen_height as i32 - max_height) / 2;

		self.ui.window.set_pos(pos_x, pos_y);
	}
}

/// Writes the given [`HitData`] to the given [`HitUi`].
///
/// `selected` highlights the hit.
fn update_hit(hit_ui: &mut HitUi, hit: Option<&ScoredHit>, selected: bool, show_score: bool) {
	let title = hit.map_or("", |h| h.hit.get_title());
	let subtitle = hit.map_or("", |h| h.hit.get_subtitle());

	hit_ui.title.set_label(title);

	if show_score {
		let format = format!("[{}] {}", hit.map_or(0, |h| h.score), subtitle);
		hit_ui.subtitle.set_label(&format);
	} else {
		hit_ui.subtitle.set_label(subtitle);
	}

	let frame_type = match selected {
		true => FrameType::FlatBox,
		false => FrameType::NoBox,
	};

	hit_ui.group.set_frame(frame_type);
}
