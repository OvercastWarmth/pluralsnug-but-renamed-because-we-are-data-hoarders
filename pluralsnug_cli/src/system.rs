use pluralsnug::System;

use crate::cli::SystemField;
use std::error::Error;

pub fn system(system: System, field: SystemField) -> Result<(), Box<dyn Error>> {
	match field {
		SystemField::Name { action } => name::name(system, action)?,
	};

	Ok(())
}

mod name {
	use crate::cli::Action;
	use pluralsnug::System;
	use std::error::Error;

	pub fn name(mut system: System, action: Option<Action>) -> Result<(), Box<dyn Error>> {
		match action {
			Some(Action::Set { value }) => {
				println!("Setting string `name` to \"{}\"", value);
				system.name = Some(value);
				Ok(())
			}
			Some(Action::Remove) => {
				system.name = None;
				Ok(())
			}
			None => {
				// TODO figure out how on earth to represent the lack of a name
				println!("{}", system.name.unwrap_or(String::from("")));
				Ok(())
			}
		}
	}
}
