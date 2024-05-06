use thiserror::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::Write;
use std::path::Path;


lazy_static! {
	pub static ref BATL_NAME_REGEX: Regex = Regex::new(r"^[a-z][a-z0-9\-_]*(/[a-z][a-z0-9\-_]*)+$").unwrap();
	pub static ref BATL_LINK_REGEX: Regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_\-]*$").unwrap();
}

#[derive(Error, Debug)]
pub enum UtilityError {
	#[error("IO Error: {0}")]
	IoError(#[from] std::io::Error),
	#[error("Resource does not exist: {0}")]
	ResourceDoesNotExist(String),
	#[error("Resource already exists: {0}")]
	ResourceAlreadyExists(String),
	#[error("Invalid config")]
	InvalidConfig,
	#[error("Link not found")]
	LinkNotFound,
	#[error("Invalid name: {0}")]
	InvalidName(String),
	#[error("Already setup")]
	AlreadySetup,
	#[error("No scripts found")]
	NoScripts,
	#[error("Script not found: {0}")]
	ScriptNotFound(String),
	#[error("Script error: {0}")]
	ScriptError(String)
}

pub fn write_toml<T: serde::Serialize>(path: &Path, data: &T) -> Result<(), UtilityError> {
	let mut file = std::fs::File::create(path)?;

	file.write_all(toml::to_string(data).unwrap().as_bytes())?;

	Ok(())
}
