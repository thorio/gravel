use std::cmp;

/// Handles scrolling logic.
///
/// The view is always an integer offset from the top, thus
/// never displaying a partial item.
pub struct Scroll {
	length: i32,
	max_view_size: i32,
	cursor: i32,
	scroll: i32,
}

impl Scroll {
	/// Creates a new instance.
	/// - length: number of items in the list
	/// - max_view_size: number of items that can be displayed at once
	pub fn new(length: i32, max_view_size: i32) -> Self {
		Scroll {
			length: length,
			max_view_size: max_view_size,
			cursor: 0,
			scroll: 0,
		}
	}

	/// Move the cursor up by one item.
	pub fn cursor_up(&mut self) {
		if self.cursor <= 0 {
			self.bottom();
		} else {
			self.cursor = self.cursor - 1;
			self.scroll = cmp::min(self.scroll, self.cursor);
		}
	}

	/// Move the cursor down by one item.
	pub fn cursor_down(&mut self) {
		if self.cursor >= self.length - 1 {
			self.top();
		} else {
			self.cursor = self.cursor + 1;
			self.scroll = cmp::max(self.scroll, self.cursor - self.view_size() + 1);
		}
	}

	/// Move the view up one page.
	pub fn page_up(&mut self) {
		if self.scroll - self.view_size() <= 0 {
			self.top();
		} else {
			self.scroll = self.scroll() - self.view_size();
			self.cursor = self.scroll;
		}
	}

	/// Move the view down one page.
	pub fn page_down(&mut self) {
		if self.scroll + self.view_size() * 2 >= self.length {
			self.bottom();
		} else {
			self.scroll = self.scroll() + self.view_size();
			self.cursor = self.scroll + self.view_size() - 1;
		}
	}

	/// Move the view and cursor to the top.
	pub fn top(&mut self) {
		self.cursor = 0;
		self.scroll = 0;
	}

	/// Move the view and cursor to the bottom.
	pub fn bottom(&mut self) {
		self.cursor = self.length - 1;
		self.scroll = self.length - self.view_size();
	}

	/// Set the number of items in the list.
	pub fn set_length(&mut self, length: i32) {
		self.length = length;
		self.cursor = 0;
		self.scroll = 0;
	}

	/// Gets the number of items that fit inside the view.
	pub fn view_size(&self) -> i32 {
		cmp::min(self.length, self.max_view_size)
	}

	/// Gets the cursor position.
	pub fn cursor(&self) -> i32 {
		self.cursor
	}

	/// Gets the views offset from the top of the list.
	pub fn scroll(&self) -> i32 {
		self.scroll
	}

	/// Gets the length of the list.
	pub fn length(&self) -> i32 {
		self.length
	}
}
