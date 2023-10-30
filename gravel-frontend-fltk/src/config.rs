use fltk::enums::Color;
use gravel_core::config::PluginConfigAdapter;
use serde::Deserialize;

pub const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

/// Reads the [`DeserializedLayoutConfig`] from the adapter and transforms
/// it to the final [`Config`].
pub fn get_config(config: &PluginConfigAdapter) -> Config {
	let base = config.get::<DeserializedConfig>(DEFAULT_CONFIG);

	Config {
		layout: get_layout_config(&base.layout),
		colors: get_color_config(&base.colors),
		behaviour: base.behaviour,
	}
}

/// Performs scaling and computes additional values.
fn get_layout_config(layout: &DeserializedLayoutConfig) -> LayoutConfig {
	let hit_title_height = (layout.hit_title_height as f32 * layout.scale) as i32;
	let hit_title_font_size = (layout.hit_title_font_size as f32 * layout.scale) as i32;
	let hit_subtitle_height = (layout.hit_subtitle_height as f32 * layout.scale) as i32;
	let hit_subtitle_font_size = (layout.hit_subtitle_font_size as f32 * layout.scale) as i32;
	let query_font_size = (layout.query_font_size as f32 * layout.scale) as i32;
	let scrollbar_width = (layout.scrollbar_width as f32 * layout.scale) as i32;
	let scrollbar_padding = (layout.scrollbar_padding as f32 * layout.scale) as i32;
	let padding = (layout.padding as f32 * layout.scale) as i32;
	let window_width = (layout.window_width as f32 * layout.scale) as i32;
	let max_hits = layout.max_hits;

	let hit_width = window_width - padding * 2;
	let hit_height = hit_title_height + hit_subtitle_height;

	let query_width = hit_width;
	let query_height = hit_height;

	let scrollbar_x = window_width - padding - scrollbar_width;
	let scrollbar_y = padding * 2 + query_height;
	let scrollbar_height = hit_height * max_hits;

	let hit_start_y = padding * 2 + query_height;
	let window_min_height = query_height + padding * 2;

	LayoutConfig {
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

/// Converts the hex colors into the [`Color`] enum.
fn get_color_config(colors: &DeserializedColorConfig) -> ColorConfig {
	ColorConfig {
		background: Color::from_hex(colors.background),
		accent: Color::from_hex(colors.accent),
		text: Color::from_hex(colors.text),
	}
}

#[derive(Debug)]
pub struct Config {
	pub layout: LayoutConfig,
	pub colors: ColorConfig,
	pub behaviour: BehaviourConfig,
}

#[derive(Debug)]
pub struct ColorConfig {
	pub background: Color,
	pub accent: Color,
	pub text: Color,
}

#[derive(Deserialize, Debug)]
pub struct LayoutConfig {
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
pub struct BehaviourConfig {
	pub auto_hide: bool,
	pub show_scores: bool,
	pub auto_center_window: bool,
	pub window_hide_debounce: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct DeserializedConfig {
	pub layout: DeserializedLayoutConfig,
	pub colors: DeserializedColorConfig,
	pub behaviour: BehaviourConfig,
}

#[derive(Deserialize, Debug)]
pub struct DeserializedLayoutConfig {
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

#[derive(Deserialize, Debug)]
pub struct DeserializedColorConfig {
	pub background: u32,
	pub accent: u32,
	pub text: u32,
}
