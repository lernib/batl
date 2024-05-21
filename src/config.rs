use semver::Version;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::env::current_dir;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use thiserror::Error;

/****************************************
* Config Formats
****************************************/
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Config {
	pub environment: EnvConfig,
	pub workspace: Option<HashMap<String, String>>,
	pub repository: Option<RepositoryConfig>,
	pub scripts: Option<HashMap<String, String>>,
	pub dependencies: Option<HashMap<String, String>>,
	pub restrict: Option<HashMap<Restrictor, RestrictConfig>>
}

impl Config {
	pub fn toml_dir(latest_child: &Path) -> Option<PathBuf> {
		let mut current_path = latest_child.to_path_buf();

		loop {
			let batl_toml = current_path.join("batl.toml");
	
			if batl_toml.exists() {
				break Some(current_path);
			}
	
			if !current_path.pop() {
				break None;
			}
		}
	}

	pub fn read(path: &Path) -> Result<Self, ReadConfigError> {
		let config_str = std::fs::read_to_string(path)?;
		Ok(toml::from_str(&config_str)?)
	}

	pub fn get_path_on_condition_from_dir(path: &Path, f: impl Fn(&Self) -> bool) -> Result<Option<PathBuf>, ReadConfigError> {
		let mut search_dir = Self::toml_dir(path);

		while let Some(config_dir) = search_dir {
			let config = Self::read(&config_dir.join("batl.toml"))?;
		
			if !f(&config) {
				search_dir = config_dir
					.parent()
					.and_then(|p| Self::toml_dir(p));
			} else {
				return Ok(Some(config_dir.join("batl.toml")));
			}
		}

		Ok(None)
	}

	pub fn get_on_condition_from_dir(path: &Path, f: impl Fn(&Self) -> bool) -> Result<Option<Config>, ReadConfigError> {
		let mut search_dir = Self::toml_dir(path);

		while let Some(config_dir) = search_dir {
			let config = Self::read(&config_dir.join("batl.toml"))?;
		
			if !f(&config) {
				search_dir = config_dir
					.parent()
					.and_then(|p| Self::toml_dir(p));
			} else {
				return Ok(Some(config));
			}
		}

		Ok(None)
	}

	pub fn get_path_on_condition(f: impl Fn(&Self) -> bool) -> Result<Option<PathBuf>, ReadConfigError> {
		Self::get_path_on_condition_from_dir(&current_dir()?, f)
	}

	pub fn get_on_condition(f: impl Fn(&Self) -> bool) -> Result<Option<Config>, ReadConfigError> {
		Self::get_on_condition_from_dir(&current_dir()?, f)
	}

	pub fn get_workspace() -> Result<Option<Config>, ReadConfigError> {
		Self::get_on_condition(|conf| conf.is_workspace())
	}

	pub fn get_repository() -> Result<Option<Config>, ReadConfigError> {
		Self::get_on_condition(|conf| conf.is_repository())
	}

	pub fn get_workspace_from_dir(path: &Path) -> Result<Option<Config>, ReadConfigError> {
		Self::get_on_condition_from_dir(path, |conf| conf.is_workspace())
	}

	pub fn get_repository_from_dir(path: &Path) -> Result<Option<Config>, ReadConfigError> {
		Self::get_on_condition_from_dir(path, |conf| conf.is_repository())
	}

	pub fn is_workspace(&self) -> bool {
		self.workspace.is_some()
	}

	pub fn is_repository(&self) -> bool {
		self.repository.is_some()
	}

	pub fn path(&self) -> Option<PathBuf> {
		Self::get_path_on_condition(|conf| conf == self).ok().flatten()
	}
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct EnvConfig {
	pub version: Version
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct RepositoryConfig {
	pub name: String,
	pub version: Version,
	pub build: Option<String>,
	pub git: Option<RepositoryGitConfig>
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct RepositoryGitConfig {
	pub url: String,
	pub path: String
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Restrictor {
	Windows,
	Linux,
	Unix,
	MacOs
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct RestrictConfig {
	pub include: Option<RestrictRequirement>,
	pub dependencies: Option<HashMap<String, String>>
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RestrictRequirement {
	Deny,
	Allow,
	Require
}

#[derive(Debug, Error)]
pub enum ReadConfigError {
	#[error("{0}")]
	IoError(#[from] std::io::Error),
	#[error("{0}")]
	TomlError(#[from] toml::de::Error)
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct BatlRc {
	pub api: BatlRcApi
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct BatlRcApi {
	pub credentials: String
}

impl Default for BatlRcApi {
	fn default() -> Self {
		Self {
			credentials: "YOUR-KEY-GOES-HERE".to_string()
		}
	}
}
