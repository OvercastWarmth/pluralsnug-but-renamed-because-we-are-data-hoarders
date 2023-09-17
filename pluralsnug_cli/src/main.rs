use clap::Parser;
use pluralsnug_cli::{cli::Cli, run};
use std::process;

fn main() {
	let cli = Cli::parse();

	match run(cli) {
		Err(error) => {
			println!("Error: {}", error);
			process::exit(247);
		}
		_ => (),
	}
}
