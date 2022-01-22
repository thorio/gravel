use crate::{builder, constants::*, scroll::Scroll, structs::*};
use fltk::{enums::*, prelude::*};
use gravel_core::{frontend::*, provider::*, *};
use lazy_static::*;

lazy_static! {
	static ref EMPTY_HIT: HitData = HitData::empty();
}

pub struct DefaultFrontend {
	ui: Ui,
	gravel: Gravel,
	result: QueryResult,
	scroll: Scroll,
}

impl Frontend for DefaultFrontend {
	fn run(&mut self) {
		self.ui.window.platform_show();
		self.run_event_loop();
		self.ui.window.platform_hide();
	}
}

impl DefaultFrontend {
	pub fn new(gravel: Gravel) -> Self {
		let ui = builder::build();

		DefaultFrontend {
			gravel: gravel,
			ui: ui,
			result: QueryResult::empty(),
			scroll: Scroll::new(0, HIT_COUNT),
		}
	}

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
					Message::Cancel => break,
				}
			}
		}
	}

	fn query(&mut self) {
		// check if query has really changed
		if !self.ui.input.changed() {
			return;
		}

		self.result = self.gravel.query(&self.ui.input.value());

		self.ui.input.clear_changed();
		self.update_window();
		self.update_hits();
	}

	fn confirm(&self) {
		if self.result.hits.len() >= 1 {
			let cursor = self.scroll.cursor();
			let hit = &self.result.hits[cursor as usize];
			self.gravel.run_hit_action(Box::new(self), &hit);
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

	fn update_window(&mut self) {
		self.scroll.set_length(self.result.hits.len() as i32);
		let height = builder::get_window_size(self.scroll.view_size());
		self.ui.window.set_size(WINDOW_WIDTH, height);
	}

	fn update_hits(&mut self) {
		// hits offscreen are not updated
		for i in 0..HIT_COUNT {
			let position = self.scroll.scroll() + i;
			let selected = position == self.scroll.cursor();

			let hitdata = if position < self.result.hits.len() as i32 {
				self.result.hits[position as usize].get_data()
			} else {
				&EMPTY_HIT
			};

			let hit = &mut self.ui.hits[i as usize];
			update_hit(hit, hitdata, selected);
		}

		self.update_scrollbar();
	}

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
}

fn update_hit(hit: &mut HitUi, data: &HitData, selected: bool) {
	hit.title.set_label(&data.title);
	hit.subtitle.set_label(&data.subtitle);

	match selected {
		true => hit.group.set_frame(FrameType::FlatBox),
		false => hit.group.set_frame(FrameType::NoBox),
	}
	hit.group.set_damage(true);
}
