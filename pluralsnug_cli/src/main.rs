use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli {
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

// pluralsnug system name  set    "string"
//            target field action value

fn main() {
	let cli = Cli::parse();

	match cli.target {
		Target::System { field } => {
			// TODO: Read from database
			let mut system = pluralsnug::load();
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
						}
					}
				}
			}

			// TODO: Save to database
			// pluralsnug::System::Save(system);
		}
	}
}