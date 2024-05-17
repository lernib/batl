use crate::config::ReadConfigError;
use crate::env::{CreateDependentResourceError, CreateResourceError, DeleteResourceError, GeneralResourceError};
use lazy_static::lazy_static;
use regex::Regex;
use std::io::Write;
use std::path::Path;
use thiserror::Error;


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
	#[error("Script not found: {0}")]
	ScriptNotFound(String),
	#[error("Script error: {0}")]
	ScriptError(String),
	#[error("Resource cannot be collected: {0}")]
	ResourceNotCollected(String),
	#[error("Network Error: {0}")]
	NetworkError(#[from] ureq::Error)
}

impl From<ReadConfigError> for UtilityError {
	fn from(value: ReadConfigError) -> Self {
		match value {
			ReadConfigError::IoError(e) => e.into(),
			ReadConfigError::TomlError(_) => UtilityError::InvalidConfig
		}
	}
}

impl From<GeneralResourceError> for UtilityError {
	fn from(value: GeneralResourceError) -> Self {
		match value {
			GeneralResourceError::DoesNotExist => UtilityError::ResourceDoesNotExist("<>".to_string()),
			GeneralResourceError::Invalid => UtilityError::InvalidConfig,
			GeneralResourceError::IoError(e) => e.into()
		}
	}
}

impl From<CreateResourceError> for UtilityError {
	fn from(value: CreateResourceError) -> Self {
		match value {
			CreateResourceError::AlreadyExists => UtilityError::ResourceAlreadyExists("<>".to_string()),
			CreateResourceError::IoError(e) => e.into(),
			CreateResourceError::NotSetup => UtilityError::ResourceAlreadyExists("Battalion root".to_string())
		}
	}
}

impl From<DeleteResourceError> for UtilityError {
	fn from(value: DeleteResourceError) -> Self {
		match value {
			DeleteResourceError::DoesNotExist => UtilityError::ResourceAlreadyExists("<>".to_string()),
			DeleteResourceError::IoError(e) => e.into()
		}
	}
}

impl From<CreateDependentResourceError> for UtilityError {
	fn from(value: CreateDependentResourceError) -> Self {
		match value {
			CreateDependentResourceError::Creation(e) => e.into(),
			CreateDependentResourceError::IoError(e) => e.into(),
			CreateDependentResourceError::Dependent(e) => e.into()
		}
	}
}

pub fn write_toml<T: serde::Serialize>(path: &Path, data: &T) -> Result<(), std::io::Error> {
	let mut file = std::fs::File::create(path)?;

	file.write_all(toml::to_string(data).unwrap().as_bytes())?;

	Ok(())
}
