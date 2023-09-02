use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct CommandNotFoundError {
	command: String,
}

impl Display for CommandNotFoundError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "Command not found", self.command)
	}
}
impl Error for CommandNotFoundError {}
