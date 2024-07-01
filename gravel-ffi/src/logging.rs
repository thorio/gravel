use abi_stable::{sabi_trait, std_types::RBox};
use log::{Log, Metadata, Record};
use log_types::{RLevelFilter, RMetadata, RRecord};

pub type BoxDynLogTarget = LogTarget_TO<'static, RBox<()>>;

#[sabi_trait]
pub trait LogTarget: Send + Sync {
	fn enabled(&self, metadata: RMetadata<'_>) -> bool;
	fn log(&self, record: RRecord<'_>);
	fn flush(&self);
	fn max_level(&self) -> RLevelFilter;
}

pub struct StaticLogTarget;

impl StaticLogTarget {
	pub fn get() -> BoxDynLogTarget {
		BoxDynLogTarget::from_value(Self {}, sabi_trait::TD_Opaque)
	}
}

impl LogTarget for StaticLogTarget {
	fn enabled(&self, metadata: RMetadata<'_>) -> bool where {
		log::logger().enabled(&metadata.into())
	}

	fn log(&self, record: RRecord<'_>) {
		record.log(log::logger());
	}

	fn flush(&self) {
		log::logger().flush();
	}

	fn max_level(&self) -> RLevelFilter {
		log::max_level().into()
	}
}

pub struct ForwardLogger {
	target: BoxDynLogTarget,
}

impl ForwardLogger {
	pub fn register(target: BoxDynLogTarget) {
		log::set_max_level(target.max_level().into());
		log::set_boxed_logger(Box::new(Self { target })).expect("logger must only be registered once per plugin");
	}
}

impl Log for ForwardLogger {
	fn enabled(&self, metadata: &Metadata<'_>) -> bool {
		self.target.enabled(metadata.into())
	}

	fn log(&self, record: &Record<'_>) {
		self.target.log(record.into());
	}

	fn flush(&self) {
		self.target.flush();
	}
}

/// Contains FFI-safe wrappers and associated conversions for `log` types.
mod log_types {
	use abi_stable::std_types::{ROption, RStr, RString};
	use abi_stable::traits::{IntoReprC, IntoReprRust};
	use abi_stable::StableAbi;
	use log::{Level, LevelFilter, Log, Metadata, Record, RecordBuilder};
	use std::fmt::Arguments;

	/// FFI-safe representation of `log::Metadata`
	#[repr(C)]
	#[derive(StableAbi)]
	pub struct RMetadata<'a> {
		level: RLevel,
		target: RStr<'a>,
	}

	impl<'a> From<&Metadata<'a>> for RMetadata<'a> {
		fn from(value: &Metadata<'a>) -> Self {
			Self {
				level: value.level().into(),
				target: value.target().into_c(),
			}
		}
	}

	impl<'a> From<RMetadata<'a>> for Metadata<'a> {
		fn from(value: RMetadata<'a>) -> Self {
			Metadata::builder()
				.level(value.level.into())
				.target(value.target.as_str())
				.build()
		}
	}

	/// FFI-safe representation of `log::LevelFilter`
	#[repr(u8)]
	#[derive(StableAbi, Clone, Copy)]
	pub enum RLevelFilter {
		Off,
		Error,
		Warn,
		Info,
		Debug,
		Trace,
	}

	impl From<LevelFilter> for RLevelFilter {
		fn from(value: LevelFilter) -> Self {
			match value {
				LevelFilter::Off => Self::Off,
				LevelFilter::Error => Self::Error,
				LevelFilter::Warn => Self::Warn,
				LevelFilter::Info => Self::Info,
				LevelFilter::Debug => Self::Debug,
				LevelFilter::Trace => Self::Trace,
			}
		}
	}

	impl From<RLevelFilter> for LevelFilter {
		fn from(value: RLevelFilter) -> Self {
			match value {
				RLevelFilter::Off => Self::Off,
				RLevelFilter::Error => Self::Error,
				RLevelFilter::Warn => Self::Warn,
				RLevelFilter::Info => Self::Info,
				RLevelFilter::Debug => Self::Debug,
				RLevelFilter::Trace => Self::Trace,
			}
		}
	}

	/// FFI-safe representation of `log::Level`
	#[repr(u8)]
	#[derive(StableAbi, Clone, Copy)]
	pub enum RLevel {
		Error,
		Warn,
		Info,
		Debug,
		Trace,
	}

	impl From<Level> for RLevel {
		fn from(value: Level) -> Self {
			match value {
				Level::Error => Self::Error,
				Level::Warn => Self::Warn,
				Level::Info => Self::Info,
				Level::Debug => Self::Debug,
				Level::Trace => Self::Trace,
			}
		}
	}

	impl From<RLevel> for Level {
		fn from(value: RLevel) -> Self {
			match value {
				RLevel::Error => Self::Error,
				RLevel::Warn => Self::Warn,
				RLevel::Info => Self::Info,
				RLevel::Debug => Self::Debug,
				RLevel::Trace => Self::Trace,
			}
		}
	}

	/// FFI-safe representation of `log::Record`
	#[repr(C)]
	#[derive(StableAbi)]
	pub struct RRecord<'a> {
		// you can do this without allocations using DynTrait,
		// but I found it's a few hundred nanoseconds slower overall
		args: RString,
		metadata: RMetadata<'a>,
		module_path: ROption<RStr<'a>>,
		file: ROption<RStr<'a>>,
		line: ROption<u32>,
	}

	impl<'a> From<&'a Record<'a>> for RRecord<'a> {
		fn from(value: &'a Record<'a>) -> Self {
			Self {
				args: value.args().to_string().into_c(),
				metadata: value.metadata().into(),
				module_path: value.module_path().map(IntoReprC::into_c).into_c(),
				file: value.file().map(IntoReprC::into_c).into_c(),
				line: value.line().into_c(),
			}
		}
	}

	impl<'a> RRecord<'a> {
		pub fn log(self, logger: &dyn Log) {
			fn inner<'a>(logger: &dyn Log, args: Arguments<'a>, mut builder: RecordBuilder<'a>) {
				logger.log(&builder.args(args).build());
			}

			let mut builder = Record::builder();

			builder
				.metadata(self.metadata.into())
				.module_path(self.module_path.map(IntoReprRust::into_rust).into_rust())
				.file(self.file.map(IntoReprRust::into_rust).into_rust())
				.line(self.line.into_rust());

			// we have do do this song and dance to make format_args happy
			inner(logger, format_args!("{}", self.args), builder);
		}
	}
}
