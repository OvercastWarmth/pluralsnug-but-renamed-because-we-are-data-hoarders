use std::error::Error;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
	#[command(subcommand)]
	target: Target,
}

#[derive(Subcommand, Debug)]
enum Target {
	System {
		#[command(subcommand)]
		field: SystemField,
	},
}

#[derive(Subcommand, Debug)]
enum SystemField {
	Name {
		#[command(subcommand)]
		action: Option<Action>,
	},
}

#[derive(Subcommand, Debug)]
enum Action {
	Set { value: String },
	Remove,
}

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
	match cli.target {
		Target::System { field } => {
			let mut system = pluralsnug::load("tests/test.conf")?;
			println!("{:?}", system); // TEMP

			match field {
				SystemField::Name { action } => {
					match action {
						Some(Action::Set { value }) => {
							println!("Setting string `name` to \"{}\"", value);
							system.name = Some(value);

							println!("{:?}", system); // TEMP
						}
						Some(Action::Remove) => {
							system.name = None;

							println!("{:?}", system); // TEMP
						}
						None => {
							println!("{}", system.name.unwrap_or(String::from("unset")));
							return Ok(());
						}
					}
				}
			}

			pluralsnug::save(system, "tests/test.conf")?;
		}
	}

	Ok(())
}
