#![allow(clippy::module_name_repetitions)]

use thiserror::Error;


#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ReadConfigError {
	#[error("{0}")]
	IoError(#[from] std::io::Error),
	#[error("{0}")]
	TomlError(#[from] toml::de::Error)
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CreateResourceError {
	#[error("IO Error: {0}")]
	IoError(#[from] std::io::Error),
	#[error("Battalion not set up")]
	NotSetup,
	#[error("Resource already exists")]
	AlreadyExists
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CreateDependentResourceError {
	#[error("IO Error: {0}")]
	IoError(#[from] std::io::Error),
	#[error("Error while creating resource: {0}")]
	Creation(#[from] CreateResourceError),
	#[error("Error while getting dependents: {0}")]
	Dependent(#[from] GeneralResourceError)
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum GeneralResourceError {
	#[error("IO Error: {0}")]
	IoError(#[from] std::io::Error),
	#[error("Resource does not exist")]
	DoesNotExist,
	#[error("Resource invalid/corrupted")]
	Invalid
}

impl From<ReadConfigError> for GeneralResourceError {
	#[inline]
	fn from(value: ReadConfigError) -> Self {
		match value {
			ReadConfigError::IoError(e) if {
				e.kind() == std::io::ErrorKind::NotFound
			} => Self::DoesNotExist,
			ReadConfigError::IoError(e) => e.into(),
			ReadConfigError::TomlError(_) => Self::Invalid
		}
	}
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum DeleteResourceError {
	#[error("IO Error: {0}")]
	IoError(#[from] std::io::Error),
	#[error("Resource does not exist")]
	DoesNotExist
}
