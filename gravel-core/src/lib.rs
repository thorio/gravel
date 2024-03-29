//! gravel's core library.
//! Contains traits and structs needed to write a plugin, as well as
//! core functionality for querying and scoring.

pub mod config;
mod engine;
mod frontend;
pub mod hotkeys;
pub mod paths;
pub mod performance;
pub mod plugin;
mod provider;
pub mod scoring;

pub use engine::{QueryEngine, QueryResult};
pub use frontend::{Frontend, FrontendExitStatus, FrontendMessage};
pub use provider::{Hit, Provider, ProviderResult, SimpleHit};
