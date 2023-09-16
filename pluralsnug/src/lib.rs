use serde::{Deserialize, Serialize};
use std::{
	error::Error,
	fs,
	path::{Path, PathBuf},
};

/// A PluralSnug system.
#[derive(Deserialize, Debug, Serialize)]
pub struct System {
	/// The system's name
	pub name: Option<String>,
}

/// Loads a PluralSnug system.
pub fn load(path: &PathBuf) -> Result<System, Box<dyn Error>> {
	let content = match fs::read_to_string(&path) {
		Ok(content) => content,
		Err(error) => Err(format!(
			"Could not load PluralSnug system at {:?}\nOriginal Error: {}",
			path, error
		))?,
	};

	let system = match json5::from_str(content.as_str()) {
		Ok(system) => system,
		Err(error) => Err(format!(
			"Could not load PluralSnug system: JSON5 is invalid\nOriginal Error: {}",
			error
		))?,
	};

	Ok(system)
}

/// Saves a PluralSnug system.
pub fn save<P: AsRef<Path>>(system: System, path: P) -> Result<(), Box<dyn Error>> {
	fs::write(path, json5::to_string(&system)?)?;
	Ok(())
}
