use fltk::enums::Color;
use gravel_core::config::PluginConfigAdapter;
use serde::Deserialize;

pub const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

/// Reads the [`DeserializedLayoutConfig`] from the adapter and transforms
/// it to the final [`Config`].
pub fn get_config(adapter: &PluginConfigAdapter) -> Config {
	let config = adapter.get::<Config>(DEFAULT_CONFIG);

	if config.behaviour.start_hidden && config.behaviour.exit_on_hide {
		log::warn!("frontend is configured to both start hidden and hide on exit, that doesn't make sense");
	}

	config
}

#[derive(Deserialize, Debug)]
pub struct Behaviour {
	pub start_hidden: bool,
	pub auto_hide: bool,
	pub exit_on_hide: bool,
	pub window_hide_debounce: Option<u64>,
	pub auto_center_window: bool,
	pub show_scores: bool,
}

#[derive(Deserialize, Debug)]
#[serde(from = "deserialize::Layout")]
pub struct Layout {
	pub max_hits: i32,
	pub hit_width: i32,
	pub hit_height: i32,
	pub query_width: i32,
	pub query_height: i32,
	pub scrollbar_x: i32,
	pub scrollbar_y: i32,
	pub scrollbar_height: i32,
	pub hit_start_y: i32,
	pub window_min_height: i32,
	pub hit_title_height: i32,
	pub hit_title_font_size: i32,
	pub hit_subtitle_height: i32,
	pub hit_subtitle_font_size: i32,
	pub query_font_size: i32,
	pub scrollbar_width: i32,
	pub scrollbar_padding: i32,
	pub padding: i32,
	pub window_width: i32,
}

#[derive(Deserialize, Debug)]
pub struct Config {
	pub layout: Layout,
	pub colors: Colors,
	pub behaviour: Behaviour,
}

#[derive(Deserialize, Debug)]
pub struct Colors {
	#[serde(deserialize_with = "deserialize::color")]
	pub background: Color,
	#[serde(deserialize_with = "deserialize::color")]
	pub accent: Color,
	#[serde(deserialize_with = "deserialize::color")]
	pub text: Color,
}

pub mod deserialize {
	use fltk::enums::Color;
	use serde::{Deserialize, Deserializer};

	pub fn color<'de, D: Deserializer<'de>>(de: D) -> Result<Color, D::Error> {
		u32::deserialize(de).map(Color::from_hex)
	}

	#[derive(Deserialize, Debug)]
	pub struct Layout {
		pub scale: f32,
		pub max_hits: i32,
		pub hit_title_height: i32,
		pub hit_title_font_size: i32,
		pub hit_subtitle_height: i32,
		pub hit_subtitle_font_size: i32,
		pub query_font_size: i32,
		pub scrollbar_width: i32,
		pub scrollbar_padding: i32,
		pub padding: i32,
		pub window_width: i32,
	}

	impl From<Layout> for super::Layout {
		fn from(value: Layout) -> Self {
			let hit_title_height = (value.hit_title_height as f32 * value.scale) as i32;
			let hit_title_font_size = (value.hit_title_font_size as f32 * value.scale) as i32;
			let hit_subtitle_height = (value.hit_subtitle_height as f32 * value.scale) as i32;
			let hit_subtitle_font_size = (value.hit_subtitle_font_size as f32 * value.scale) as i32;
			let query_font_size = (value.query_font_size as f32 * value.scale) as i32;
			let scrollbar_width = (value.scrollbar_width as f32 * value.scale) as i32;
			let scrollbar_padding = (value.scrollbar_padding as f32 * value.scale) as i32;
			let padding = (value.padding as f32 * value.scale) as i32;
			let window_width = (value.window_width as f32 * value.scale) as i32;
			let max_hits = value.max_hits;

			let hit_width = window_width - padding * 2;
			let hit_height = hit_title_height + hit_subtitle_height;

			let query_width = hit_width;
			let query_height = hit_height;

			let scrollbar_x = window_width - padding - scrollbar_width;
			let scrollbar_y = padding * 2 + query_height;
			let scrollbar_height = hit_height * max_hits;

			let hit_start_y = padding * 2 + query_height;
			let window_min_height = query_height + padding * 2;

			super::Layout {
				max_hits,
				hit_width,
				hit_height,
				query_width,
				query_height,
				scrollbar_x,
				scrollbar_y,
				scrollbar_height,
				hit_start_y,
				window_min_height,
				hit_title_height,
				hit_title_font_size,
				hit_subtitle_height,
				hit_subtitle_font_size,
				query_font_size,
				scrollbar_width,
				scrollbar_padding,
				padding,
				window_width,
			}
		}
	}
}
