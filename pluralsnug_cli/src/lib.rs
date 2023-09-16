use clap::{Parser, Subcommand};
use std::{env, error::Error, fmt::Display, path::PathBuf};
#[derive(Parser, Debug)]
pub struct Cli {
	#[command(subcommand)]
	target: Target,

	// TODO: Figure out how to bring flags to the back
	#[arg(short, long)]
	config: Option<PathBuf>,
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
	let path = configure(&cli)?;

	match cli.target {
		Target::System { field } => {
			let mut system = pluralsnug::load(&path)?;
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

			pluralsnug::save(system, path)?;
		}
	}

	Ok(())
}

fn configure(cli: &Cli) -> Result<PathBuf, Box<dyn Error>> {
	let path: PathBuf = match &cli.config {
		Some(path) => path.clone(),
		None => match env::var("PLURALSNUG_PATH") {
			Ok(path) => PathBuf::from(path),
			Err(_) => match home::home_dir() {
				Some(dir) => PathBuf::from(dir).join(".pluralsnug/system.json5"),
				None => return Err(Box::new(SystemPathRanOutOfOptionsError)),
			},
		},
	};

	Ok(path)
}

#[derive(Debug)]
struct SystemPathRanOutOfOptionsError;

impl Display for SystemPathRanOutOfOptionsError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"You don't have a home path and we've run out of options.\nSet the PLURALSNUG_PATH environment variable to set where you want your PluralSnug database to be"
		)
	}
}

impl Error for SystemPathRanOutOfOptionsError {}
