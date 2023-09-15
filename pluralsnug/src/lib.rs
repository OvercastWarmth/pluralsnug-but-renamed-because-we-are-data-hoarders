#[derive(Debug)]
pub struct System {
	pub name: Option<String>,
}

impl System {}

// let mut system = pluralsnug::load();
// - Load system
//   - Read hocon file
//   - Parse using serde
// - Print System.name

pub fn load() -> System {
	System {
		name: Some("wawa".to_string())
	}
}