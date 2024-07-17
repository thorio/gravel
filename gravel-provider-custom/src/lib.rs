//! Returns a static set of hits based on configuration, then runs configurable commands when they are selected.
//!
//! Example configuration:
//! ```yml
//! - plugin: custom
//!   config:
//!     hits:
//!       - title: say hello
//!         action: "echo hello from bash"
//!
//!       - title: what do you see?
//!         action: ["ls", "-lAh"]
//!         post_action: nothing
//!
//!       - title: this one is sticky
//!         subtitle: and it won't leave you alone
//!         override_score: 4294967295
//!         action:
//!           shell: zsh
//!           command: "where zsh"
//!         post_action: refresh
//! ```
//!
//! ### Required Fields
//!
//! #### title
//! This text is prominently displayed in the UI.
//!
//! #### action
//! Defines the command run when the hit is selected.  
//! This one has three forms:
//! - System Shell  
//!
//!   This runs the command as one string in the system shell, either `sh` on linux or `cmd` on windows.
//!   ```yml
//!   action: "echo this is run in 'sh' or 'cmd'"
//!   ```
//!   <br>
//!
//! - Shell  
//!
//!   This assumes the executable accepts arguments like `bash -c "command here"`, and therefore won't work well with powershell for example.  
//!   In those cases, use the raw command form below.
//!   ```yml
//!   action:
//!     shell: zsh
//!     command: "echo this is run in 'zsh'; where zsh"
//!   ```
//!   <br>
//!
//! - Raw Command  
//!
//!   Runs the first parameter as an executable, passing the rest as args. It _does not_ use a shell for this, the executable is executed directly.  
//!   The first argument can be an absolute or relative path (to gravel's working directory), or the name of a binary in the PATH.
//!   ```yml
//!   action: ["ls", "-lAh", "/some directory with spaces/abc"]
//!   ```
//!
//! ### Optional Fields
//!
//! #### subtitle
//! This text is displayed next to the title.  
//! It will default to being empty.
//!
//! #### override_score
//! Allows you to skip the scoring process for this hit and assign it a score directly.  
//! Must be an integer between 0 and 4294967295 (32bit unsigned integer).
//! Defaults to `null`, using the normal scoring process.
//!
//! #### secondary_action
//! This is defined identically to the regular action above, but triggered on the secondary action instead.
//!
//! #### wait
//! Either `true` or `false`, this specifies if the provider should wait until the action is completed.  
//! Be careful, setting this to `true` on a long-running command will hang the UI!
//!
//! #### post_action
//! Defines what should be done after the action.  
//! Valid values are:
//! - `nothing`
//! - `hide`, this hides the frontend
//! - `refresh`, this refreshes the frontend, running the current query again

use gravel_ffi::prelude::*;
use nonempty::NonEmpty;
use serde::Deserialize;
use std::{ffi::OsStr, io, process::Command};

struct CustomProvider {
	hits: StaticHitCache,
}

#[gravel_provider("custom")]
impl Provider for CustomProvider {
	fn new(config: &PluginConfigAdapter<'_>) -> Self {
		let hits = config.get::<Config>("").hits;
		log::trace!("initializing custom provider with {} hits", hits.len());

		Self {
			hits: StaticHitCache::new(hits.into_iter().map(into_hit)),
		}
	}

	fn query(&self, _query: &str) -> ProviderResult {
		ProviderResult::from_cached(self.hits.get())
	}
}

fn into_hit(config: HitConfig) -> SimpleHit {
	let subtitle = config.subtitle.unwrap_or_default();
	let wait = config.wait.unwrap_or(true);

	SimpleHit::new(config.title, subtitle, move |h, context| {
		run_action(&config.action, wait, h);
		run_post_action(context, config.post_action);
	})
	.with_secondary(move |hit, context| {
		run_action(&config.secondary_action, wait, hit);
		run_post_action(context, config.post_action);
	})
	.with_score(config.override_score)
}

fn run_action(action: &Action, wait: bool, hit: &SimpleHit) {
	log::debug!("running custom hit '{}'", hit.title());

	let command = match action {
		#[cfg(unix)]
		Action::SystemShell(arg) => command("sh", ["-c", arg]),

		#[cfg(windows)]
		Action::SystemShell(arg) => command("cmd", ["/c", arg]),

		Action::Shell { shell, command: arg } => command(shell, ["-c", arg]),
		Action::Command(args) => command(args.first(), args.iter().skip(1)),
	};

	if let Err(e) = run_command(command, wait) {
		log::error!("unable to run custom hit '{}': {e}", hit.title());
	}
}

fn run_command(mut command: Command, wait: bool) -> io::Result<()> {
	command.spawn().and_then(|mut p| {
		if wait {
			p.wait()?;
		}

		Ok(())
	})
}

fn command<I, P>(executable: impl AsRef<OsStr>, args: I) -> Command
where
	I: IntoIterator<Item = P>,
	P: AsRef<OsStr>,
{
	let mut command = Command::new(executable);
	command.args(args);

	command
}

fn run_post_action(context: RefDynHitActionContext<'_>, action: PostAction) {
	match action {
		PostAction::Nothing => (),
		PostAction::Hide => context.hide_frontend(),
		PostAction::Refresh => context.refresh_frontend(),
	}
}

#[derive(Deserialize, Debug)]
struct Config {
	pub hits: Vec<HitConfig>,
}

#[derive(Deserialize, Debug)]
struct HitConfig {
	pub title: String,
	pub subtitle: Option<String>,
	pub override_score: Option<u32>,
	pub wait: Option<bool>,
	pub action: Action,
	pub secondary_action: Action,

	#[serde(default)]
	pub post_action: PostAction,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
#[serde(expecting = "data did not match any known action format")]
enum Action {
	SystemShell(String),
	Shell { shell: String, command: String },
	Command(NonEmpty<String>),
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum PostAction {
	Nothing,
	Hide,
	Refresh,
}

impl Default for PostAction {
	fn default() -> Self {
		Self::Hide
	}
}
