use clap::Parser;
use pluralsnug_cli::{run, Cli};

// pluralsnug system name  set    "string"
//            target field action value

fn main() {
	let cli = Cli::parse();

	match run(cli) {
		Err(error) => {
			println!("Error: {}", error)
		}
		_ => (),
	}
}
