//! gravel's core library.
//! Contains traits and structs needed to write a plugin, as well as
//! core functionality for querying and scoring.

pub mod config;
mod engine;
mod frontend;
pub mod hotkeys;
pub mod paths;
pub mod plugin;
mod provider;
pub mod scoring;

pub use engine::QueryEngine;
pub use frontend::{Frontend, FrontendMessage};
pub use provider::{Hit, HitData, Provider, QueryResult, SimpleHit};
