use std::env::Args;

mod commands {
	pub mod help;
}
mod errors;

pub fn run(mut args: Args) -> Result<(), &'static str> {
	match args.nth(1).unwrap_or("help".to_string()).as_str() {
		"help" => commands::help::print_help_text(),
		_ => Err("wawa"),
	}
}
