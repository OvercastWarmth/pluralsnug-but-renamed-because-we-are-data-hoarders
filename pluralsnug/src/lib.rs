use hocon::HoconLoader;
use serde::Deserialize;
use std::path::Path;

/// A PluralSnug system.
#[derive(Deserialize, Debug)]
pub struct System {
	/// The system's name
	pub name: Option<String>,
}

/// Loads a PluralSnug system.
pub fn load<P: AsRef<Path>>(path: P) -> Result<System, hocon::Error> {
	Ok(HoconLoader::new().load_file(path)?.resolve()?)
}

#[allow(unused_variables)]
pub fn save<P: AsRef<Path>>(system: System, path: P) -> Result<(), hocon::Error> {
	Ok(())
}
