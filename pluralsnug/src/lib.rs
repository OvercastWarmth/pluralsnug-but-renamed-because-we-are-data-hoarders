use serde::Deserialize;
use std::{error::Error, fs, path::Path};

/// A PluralSnug system.
#[derive(Deserialize, Debug)]
pub struct System {
	/// The system's name
	pub name: Option<String>,
}

/// Loads a PluralSnug system.
pub fn load<P: AsRef<Path>>(path: P) -> Result<System, Box<dyn Error>> {
	Ok(json5::from_str(fs::read_to_string(path)?.as_str())?)
}

/// Saves a PluralSnug system.
///
/// Currently not functioning
#[allow(unused_variables)]
pub fn save<P: AsRef<Path>>(system: System, path: P) -> Result<(), Box<dyn Error>> {
	Ok(())
}
