use crate::cli::Cli;
use cli::Target;
use std::{env, error::Error, fmt::Display, path::PathBuf};
use thiserror::Error;

pub mod cli;
pub mod system;

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
	let path = configure(&cli)?;
	let system = pluralsnug::load(&path)?;

	match cli.target {
		Target::System { field } => system::system(system, field)?,
	}
	// TODO: WriteGuard:
	// > make a write guard, basically a struct that holds the value,
	// > and then implements deref and derefmut, if derefmut is used,
	// > assume it was modified and save it to a file when dropped or something
	// pluralsnug::save(system, path)?;
	Ok(())
}

fn configure(cli: &Cli) -> Result<PathBuf, Box<dyn Error>> {
	let path: PathBuf = match &cli.config {
		Some(path) => path.clone(),
		None => match env::var("PLURALSNUG_PATH") {
			Ok(path) => PathBuf::from(path),
			Err(_) => match home::home_dir() {
				Some(dir) => PathBuf::from(dir).join(".pluralsnug/system.json5"),
				None => return Err(Box::new(MissingSystemPathError)),
			},
		},
	};

	Ok(path)
}

#[derive(Debug, Error)]
struct MissingSystemPathError;

impl Display for MissingSystemPathError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"There is no valid system path. Please add one using `-c [FILE]` or set the environment variable `PLURALSNUG_PATH`"
		)
	}
}
