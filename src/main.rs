use std::process;

use pluralsnug::run;

fn main() {
	let args = std::env::args();

	if let Err(e) = run(args) {
		println!("Error: {e}");
		process::exit(1);
	}
}
