/// A PluralSnug system.
#[derive(Debug)]
pub struct System {
	/// The system's name
	pub name: Option<String>,
}

impl System {}

// let mut system = pluralsnug::load();
// - Load system
//   - Read hocon file
//   - Parse using serde
// - Print System.name

/// Loads a PluralSnug system.
///
/// Currently unimplemented, provides a placeholder instead.
pub fn load() -> System {
	System {
		name: Some("wawa".to_string()),
	}
}
