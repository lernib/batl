use clap::Subcommand;
use crate::config::*;
use crate::env::System;
use crate::output::*;
use crate::utils::{UtilityError, BATL_NAME_REGEX, write_toml};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Commands {
	Ls,
	Init {
		name: String
	},
	Delete {
		name: String
	}
}

pub fn run(cmd: Commands) -> Result<(), UtilityError> {
	match cmd {
		Commands::Ls => {
			cmd_ls()
		},
		Commands::Init { name } => {
			cmd_init(name)
		},
		Commands::Delete { name } => {
			cmd_delete(name)
		}
	}
}

fn cmd_ls() -> Result<(), UtilityError> {
	let repo_root = System::batl_root()
		.ok_or(UtilityError::ResourceDoesNotExist("Battalion root".to_string()))?
		.join("repositories");

	let mut to_search: Vec<(String, PathBuf)> = std::fs::read_dir(repo_root)?
		.filter_map(|entry| {
			Some(("".to_string(), entry.ok()?.path()))
		})
		.collect();
	let mut found: Vec<String> = Vec::new();

	while let Some((name, path)) = to_search.pop() {
		if !path.is_dir() {
			continue;
		}

		let filename = path.file_name().unwrap().to_str().unwrap();

		if filename.starts_with('@') {
			let new_name = filename[1..].to_string();
			let new_name = format!("{}{}/", name, new_name);

			to_search.extend(
				std::fs::read_dir(path)?
					.filter_map(|entry| {
						Some((new_name.clone(), entry.ok()?.path()))
					})
			);
		} else {
			found.push(format!("{}{}", name, filename));
		}
	}

	for name in found {
		println!("{}", name);
	}

	Ok(())
}

fn cmd_init(name: String) -> Result<(), UtilityError> {
	if !BATL_NAME_REGEX.is_match(&name) {
		return Err(UtilityError::InvalidName(name));
	}

	let repo_root = System::batl_root()
		.ok_or(UtilityError::ResourceDoesNotExist("Battalion root".to_string()))?
		.join("repositories");

	let mut path = repo_root;
	let parts = name.split('/').collect::<Vec<&str>>();

	for part in parts.iter().take(parts.len() - 1) {
		path = path.join(format!("@{}", part));
	}

	path = path.join(parts.last().unwrap());

	if path.exists() {
		return Err(UtilityError::ResourceAlreadyExists(format!("repository {}", name)));
	}

	std::fs::create_dir_all(&path)?;

	let mut scripts = HashMap::new();
	scripts.insert("build".to_string(), "echo \"No build targets\" && exit 1".to_string());

	let config = Config {
		environment: EnvConfig {
			version: env!("CARGO_PKG_VERSION").to_string(),
		},
		workspace: None,
		repository: Some(RepositoryConfig {
			name: parts.last().unwrap().to_string(),
			version: "0.1.0".to_string(),
			build: None
		}),
		scripts: Some(scripts)
	};

	write_toml(&path.join("batl.toml"), &config)?;

	success("Initialized repository successfully");

	Ok(())
}

fn cmd_delete(name: String) -> Result<(), UtilityError> {
	if !BATL_NAME_REGEX.is_match(&name) {
		return Err(UtilityError::InvalidName(name));
	}

	let repo_root = System::batl_root()
		.ok_or(UtilityError::ResourceDoesNotExist("Battalion root".to_string()))?
		.join("repositories");

	let mut path = repo_root;
	let parts = name.split('/').collect::<Vec<&str>>();

	for part in parts.iter().take(parts.len() - 1) {
		path = path.join(format!("@{}", part));
	}

	path = path.join(parts.last().unwrap());

	if !path.exists() {
		return Err(UtilityError::ResourceDoesNotExist(format!("repository {}", name)));
	}

	std::fs::remove_dir_all(path)?;

	success("Deleted repository successfully");

	Ok(())
}
