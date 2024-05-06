use thiserror::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::env::var;
use std::io::Write;
use std::path::PathBuf;


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

pub fn get_batl_root() -> Result<PathBuf, UtilityError> {
	// 1. Check BATL_ROOT environment variable
	if let Ok(batl_root) = var("BATL_ROOT") {
		return Ok(PathBuf::from(batl_root));
	}

	// 2. Recursively descend from current directory until .batlrc is found
	let mut current_dir = std::env::current_dir()?;

	loop {
		if current_dir.join(".batlrc").exists() {
			return Ok(current_dir);
		}

		if !current_dir.pop() {
			break;
		}
	}

	// 3. Check for battalion folder in home directory
	if let Ok(home_dir) = var("HOME") {
		let batl_dir = PathBuf::from(home_dir).join("battalion");

		if batl_dir.exists() {
			return Ok(batl_dir);
		}
	}

	Err(UtilityError::ResourceDoesNotExist("Battalion root directory".to_string()))
}

pub fn write_toml<T: serde::Serialize>(path: &PathBuf, data: &T) -> Result<(), UtilityError> {
	let mut file = std::fs::File::create(path)?;

	file.write_all(toml::to_string(data).unwrap().as_bytes())?;

	Ok(())
}

pub fn get_batl_toml_dir() -> Result<PathBuf, UtilityError> {
	let mut current_path = std::env::current_dir()?;

	loop {
		let batl_toml = current_path.join("batl.toml");

		if batl_toml.exists() {
			break Ok(current_path);
		}

		if !current_path.pop() {
			break Err(UtilityError::ResourceDoesNotExist("Workspace config".to_string()));
		}
	}
}
