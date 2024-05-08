use std::convert::Infallible;
use std::env::var as env_var;
use std::path::{Path, PathBuf};
use std::str::FromStr;


pub struct System;

impl System {
	pub fn batl_root() -> Option<PathBuf> {
		// 1. Check BATL_ROOT environment variable
		if let Ok(batl_root) = env_var("BATL_ROOT") {
			return Some(PathBuf::from(batl_root));
		}

		// 2. Recursively descend from current directory until .batlrc is found
		if let Some(current_dir) = std::env::current_dir().ok() {
			let mut current_dir = current_dir;

			loop {
				if current_dir.join(".batlrc").exists() {
					return Some(current_dir);
				}

				if !current_dir.pop() {
					break;
				}
			}
		}

		// 3. Check for battalion folder in home directory
		if let Ok(home_dir) = env_var("HOME") {
			let batl_dir = PathBuf::from(home_dir).join("battalion");

			if batl_dir.exists() {
				return Some(batl_dir);
			}
		}

		None
	}

	pub fn workspace_root() -> Option<PathBuf> {
		Self::batl_root().map(|p| p.join("workspaces"))
	}

	pub fn repository_root() -> Option<PathBuf> {
		Self::batl_root().map(|p| p.join("repositories"))
	}

	pub fn repository(name: ResourceName) -> Option<Repository> {
		let mut path = Self::repository_root()?;

		let parts = name.components();

		for part in parts.iter().take(parts.len() - 1) {
			path = path.join(format!("@{}", part));
		}
		path = path.join(parts.last().unwrap());

		Some(Repository::new(path))
	}

	pub fn workspace(name: ResourceName) -> Option<Workspace> {
		let mut path = Self::workspace_root()?;

		let parts = name.components();

		for part in parts.iter().take(parts.len() - 1) {
			path = path.join(format!("@{}", part));
		}
		path = path.join(parts.last().unwrap());

		Some(Workspace::new(path))
	}
}

#[derive(Debug, Clone)]
pub struct ResourceName(Vec<String>);

impl ResourceName {
	fn new(components: Vec<String>) -> Self {
		Self(components)
	}

	fn components(&self) -> &Vec<String> {
		&self.0
	}
}

impl FromStr for ResourceName {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self::new(s.split('/').map(ToString::to_string).collect()))
	}
}

impl From<String> for ResourceName {
	fn from(value: String) -> Self {
		Self::from_str(&value).unwrap()
	}
}

impl From<&str> for ResourceName {
	fn from(value: &str) -> Self {
		Self::from_str(value).unwrap()
	}
}

pub trait Resource {
	fn path(&self) -> &Path;
}

pub struct Repository {
	path: PathBuf
}

impl Repository {
	pub(crate) fn new(path: PathBuf) -> Self {
		Self { path }
	}
}

impl Resource for Repository {
	fn path(&self) -> &Path {
		&self.path
	}
}

pub struct Workspace {
	path: PathBuf
}

impl Workspace {
	pub(crate) fn new(path: PathBuf) -> Self {
		Self { path }
	}
}

impl Resource for Workspace {
	fn path(&self) -> &Path {
		&self.path
	}
}
