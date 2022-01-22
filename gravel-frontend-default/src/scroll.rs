use std::cmp;

pub struct Scroll {
	length: i32,
	max_view_size: i32,
	cursor: i32,
	scroll: i32,
}

impl Scroll {
	pub fn new(length: i32, max_view_size: i32) -> Self {
		Scroll {
			length: length,
			max_view_size: max_view_size,
			cursor: 0,
			scroll: 0,
		}
	}

	pub fn set_length(&mut self, length: i32) {
		self.length = length;
		self.cursor = 0;
		self.scroll = 0;
	}

	pub fn cursor_up(&mut self) {
		if self.cursor <= 0 {
			self.bottom();
		} else {
			self.cursor = self.cursor - 1;
			self.scroll = cmp::min(self.scroll, self.cursor);
		}
	}

	pub fn cursor_down(&mut self) {
		if self.cursor >= self.length - 1 {
			self.top();
		} else {
			self.cursor = self.cursor + 1;
			self.scroll = cmp::max(self.scroll, self.cursor - self.view_size() + 1);
		}
	}

	pub fn page_up(&mut self) {
		if self.scroll - self.view_size() <= 0 {
			self.top();
		} else {
			self.scroll = self.scroll() - self.view_size();
			self.cursor = self.scroll;
		}
	}

	pub fn page_down(&mut self) {
		if self.scroll + self.view_size() * 2 >= self.length {
			self.bottom();
		} else {
			self.scroll = self.scroll() + self.view_size();
			self.cursor = self.scroll + self.view_size() - 1;
		}
	}

	pub fn top(&mut self) {
		self.cursor = 0;
		self.scroll = 0;
	}

	pub fn bottom(&mut self) {
		self.cursor = self.length - 1;
		self.scroll = self.length - self.view_size();
	}

	// getters
	pub fn view_size(&self) -> i32 {
		cmp::min(self.length, self.max_view_size)
	}

	pub fn cursor(&self) -> i32 {
		self.cursor
	}

	pub fn scroll(&self) -> i32 {
		self.scroll
	}

	pub fn length(&self) -> i32 {
		self.length
	}
}
