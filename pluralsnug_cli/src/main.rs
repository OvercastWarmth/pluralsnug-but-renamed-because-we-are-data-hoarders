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
		action: StringAction,
	},
}

#[derive(Subcommand, Debug)]
enum StringAction {
	Set { value: String },
}

// pluralsnug system name  set    "string"
//            target field action value

fn main() {
	let cli = Cli::parse();

	match cli.target {
		Target::System { field } => match field {
			SystemField::Name { action } => match action {
				StringAction::Set { value } => {
					// Load system from config
					// let mut system = pluralsnug::System::Load()
					println!("Set string `name` to \"{}\"", value)
					// system.name = value
					// Save system to config
					// pluralsnug::System::Save(system)

					// Load system -> system.changename().save()
					// system.readname()
					// system.removename()

					// system.name = "something"
					// system.name
					// system.name = ""
				}
			},
		},
	}
}
