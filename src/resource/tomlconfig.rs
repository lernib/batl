#![allow(clippy::exhaustive_structs)]
#![allow(clippy::exhaustive_enums)]

use crate::error::ReadConfigError;
use crate::resource::Name;
use crate::version::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};


pub type EnvironmentLatest = Environment0_2_2;
pub type RepositoryLatest = Repository0_2_2;
pub type WorkspaceLatest = Workspace0_2_2;
pub type ScriptsLatest = Scripts0_2_2;
pub type DependenciesLatest = Dependencies0_2_2;
pub type RestrictLatest = Restrict0_2_2;
pub type RestrictorLatest = Restrictor0_2_2;

#[derive(Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Environment0_2_2 {
	pub version: Version0_2_2
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Repository0_2_2 {
	pub name: Name,
	pub version: semver::Version,
	pub git: Option<RepositoryGit0_2_2>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Workspace0_2_2 {
	pub name: Name,
	pub version: semver::Version
}

pub type Links0_2_2 = Links0_2_1;
pub type RepositoryGit0_2_2 = RepositoryGit0_2_1;
pub type Scripts0_2_2 = Scripts0_2_1;
pub type Dependencies0_2_2 = Dependencies0_2_1;
pub type Restrict0_2_2 = HashMap<Restrictor0_2_2, RestrictorSettings0_2_2>;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Restrictor0_2_2 {
	Windows,
	Linux,
	Unix,
	MacOs
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct RestrictorSettings0_2_2 {
	pub include: Option<RestrictRequirement0_2_2>,
	pub dependencies: Option<Dependencies0_2_2>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RestrictRequirement0_2_2 {
	Deny,
	Allow,
	Require
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Repository0_2_1 {
	pub name: Name,
	pub version: semver::Version,
	pub build: Option<String>,
	pub git: Option<RepositoryGit0_2_1>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Workspace0_2_1 {
	pub name: Name,
	pub version: semver::Version,
	pub build: Option<String>
}

pub type Links0_2_1 = HashMap<String, Name>;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct RepositoryGit0_2_1 {
	pub url: String,
	pub path: String
}

pub type Scripts0_2_1 = HashMap<String, String>;
pub type Dependencies0_2_1 = HashMap<Name, String>;


/// Writes a toml struct to a path
/// 
/// # Errors
/// 
/// Propogates any IO errors received while writing the file.
#[inline]
pub fn write_toml<T: serde::Serialize>(path: &Path, data: &T) -> Result<(), std::io::Error> {
	let mut file = std::fs::File::create(path)?;

	file.write_all(toml::to_string(data).unwrap_or_default().as_bytes())?;

	Ok(())
}

/// Reads a toml file from a path
/// 
/// # Errors
/// 
/// Propogates any toml and IO errors to the caller
#[inline]
pub fn read_toml<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, ReadConfigError> {
	let config_str = std::fs::read_to_string(path)?;
	Ok(toml::from_str(&config_str)?)
}

/// Returns `None` if a hashmap is empty
#[inline]
#[must_use]
pub fn hashmap_to_option_hashmap<K, V, S>(map: HashMap<K, V, S>) -> Option<HashMap<K, V, S>> {
	if map.is_empty() {
		None
	} else {
		Some(map)
	}
}

pub trait TomlConfig: serde::de::DeserializeOwned {
	/// Starts at the path given, then searches all
	/// parents for a batl.toml, returning the first
	/// directory with one.
	#[inline]
	#[must_use]
	fn locate_possible(current: &Path) -> Option<PathBuf> {
		let mut current_path = current.to_path_buf();

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

	/// Starts at the given path, then searches all
	/// paths for a batl.toml *that satisfies parsing*.
	/// Returns the path of the batl.toml, including
	/// the filename.
	/// 
	/// Returns `None` if it can't be found.
	#[inline]
	#[must_use]
	fn locate(current: &Path) -> Option<PathBuf> {
		let mut search_dir = Self::locate_possible(current);

		while let Some(config_dir) = search_dir {
			let config: Result<Self, _> = read_toml(&config_dir.join("batl.toml"));

			match config {
				Ok(_) => {
					return Some(config_dir.join("batl.toml"))
				},
				Err(_) => {
					search_dir = config_dir
						.parent()
						.and_then(|p| Self::locate_possible(p));
				}
			}
		}

		None
	}

	/// Starts at the given path, then searches all
	/// paths for a batl.toml *that satisfies parsing*.
	/// Returns the path of the batl.toml, including
	/// the filename.
	/// 
	/// Returns `None` if it can't be found.
	#[inline]
	#[must_use]
	fn load(current: &Path) -> Option<Self> {
		let mut search_dir = Self::locate_possible(current);

		while let Some(config_dir) = search_dir {
			let config: Result<Self, _> = read_toml(&config_dir.join("batl.toml"));

			match config {
				Ok(config_out) => {
					return Some(config_out)
				},
				Err(_) => {
					search_dir = config_dir
						.parent()
						.and_then(|p| Self::locate_possible(p));
				}
			}
		}

		None
	}
}
