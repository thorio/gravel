use fltk::{enums::*, frame::Frame, prelude::*};

/// Custom Scrollbar implementation.
///
/// For display only, it is not interactive.
pub struct Scrollbar {
	frame: Frame,
	slider: Frame,
	slider_position: f32,
	slider_size: f32,
	pos: (i32, i32),
	size: (i32, i32),
	padding: i32,
}

impl Scrollbar {
	/// Constructs a default instance.
	pub fn default() -> Self {
		let mut frame = Frame::default();
		frame.set_frame(FrameType::FlatBox);
		frame.set_color(Color::from_hex(0xffffff));

		let mut slider = Frame::default();
		slider.set_frame(FrameType::FlatBox);
		slider.set_color(Color::from_hex(0xd77b0f));

		Scrollbar {
			frame,
			slider,
			slider_position: 0.0,
			slider_size: 0.5,
			pos: (0, 0),
			size: (0, 0),
			padding: 2,
		}
	}

	pub fn with_pos(mut self, x: i32, y: i32) -> Self {
		self.pos = (x, y);
		self.apply();
		self
	}

	pub fn with_size(mut self, w: i32, h: i32) -> Self {
		self.size = (w, h);
		self.apply();
		self
	}

	pub fn with_padding(mut self, padding: i32) -> Self {
		self.padding = padding;
		self.apply();
		self
	}

	pub fn with_colors(mut self, frame: Color, slider: Color) -> Self {
		self.frame.set_color(frame);
		self.slider.set_color(slider);
		self
	}

	pub fn set_slider_position(&mut self, position: f32) {
		self.slider_position = position;
		self.apply();
	}

	pub fn set_slider_size(&mut self, size: f32) {
		self.slider_size = size;
		self.apply();
	}

	pub fn hide(&mut self) {
		self.frame.hide();
		self.slider.hide();
	}

	pub fn show(&mut self) {
		self.frame.show();
		self.slider.show();
	}

	fn apply(&mut self) {
		self.frame.set_pos(self.pos.0, self.pos.1);
		self.frame.set_size(self.size.0, self.size.1);

		let slider_x = self.pos.0 + self.padding;
		let slider_y = self.pos.1 + (self.size.1 as f32 * self.slider_position) as i32;
		let slider_w = self.size.0 - self.padding * 2;
		let slider_h = (self.size.1 as f32 * self.slider_size) as i32;
		self.slider.set_pos(slider_x, slider_y);
		self.slider.set_size(slider_w, slider_h);
	}
}
