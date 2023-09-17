use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
	#[command(subcommand)]
	pub target: Target,

	// TODO: Figure out how to bring flags to the back
	#[arg(short, long)]
	pub config: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Target {
	System {
		#[command(subcommand)]
		field: SystemField,
	},
}

#[derive(Subcommand, Debug)]
pub enum SystemField {
	Name {
		#[command(subcommand)]
		action: Option<Action>,
	},
}

#[derive(Subcommand, Debug)]
pub enum Action {
	Set { value: String },
	Remove,
}
