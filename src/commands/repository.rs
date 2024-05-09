use clap::Subcommand;
use console::Term;
use git2::{FetchOptions, RemoteCallbacks, Progress};
use semver::Version;
use crate::config::*;
use crate::env::{Resource, System};
use crate::output::*;
use crate::utils::{UtilityError, BATL_NAME_REGEX, write_toml};
use git2::build::RepoBuilder;
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Commands {
	Ls,
	Init {
		name: String
	},
	Delete {
		name: String
	},
	Clone {
		url: String,
		#[arg(short = 'o')]
		name: String
	},
	Scaffold
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
		},
		Commands::Clone { url, name } => {
			cmd_clone(url, name)
		},
		Commands::Scaffold => {
			cmd_scaffold()
		}
	}
}

fn cmd_ls() -> Result<(), UtilityError> {
	let repo_root = System::repository_root()
		.ok_or(UtilityError::ResourceDoesNotExist("Repository root".to_string()))?;

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

	let path = System::repository(name.as_str().into())
		.ok_or(UtilityError::ResourceDoesNotExist("Battalion root".to_string()))?
		.path().to_path_buf();

	if path.exists() {
		return Err(UtilityError::ResourceAlreadyExists(format!("repository {}", name)));
	}

	std::fs::create_dir_all(&path)?;

	let mut scripts = HashMap::new();
	scripts.insert("build".to_string(), "echo \"No build targets\" && exit 1".to_string());

	let config = Config {
		environment: EnvConfig {
			version: Version::parse(env!("CARGO_PKG_VERSION")).unwrap(),
		},
		workspace: None,
		repository: Some(RepositoryConfig {
			name,
			version: Version::new(0, 1, 0),
			build: None,
			git: None
		}),
		scripts: Some(scripts),
		dependencies: None
	};

	write_toml(&path.join("batl.toml"), &config)?;

	success("Initialized repository successfully");

	Ok(())
}

fn cmd_delete(name: String) -> Result<(), UtilityError> {
	if !BATL_NAME_REGEX.is_match(&name) {
		return Err(UtilityError::InvalidName(name));
	}

	let path = System::repository(name.as_str().into())
		.ok_or(UtilityError::ResourceDoesNotExist("Battalion root".to_string()))?
		.path().to_path_buf();

	if !path.exists() {
		return Err(UtilityError::ResourceDoesNotExist(format!("repository {}", name)));
	}

	std::fs::remove_dir_all(path)?;

	success("Deleted repository successfully");

	Ok(())
}

fn cmd_clone(url: String, name: String) -> Result<(), UtilityError> {
	if !BATL_NAME_REGEX.is_match(&name) {
		return Err(UtilityError::InvalidName(name));
	}

	let path = System::repository(name.as_str().into())
		.ok_or(UtilityError::ResourceDoesNotExist("Battalion root".to_string()))?
		.path().to_path_buf();

	if path.exists() {
		return Err(UtilityError::ResourceAlreadyExists(format!("repository {}", name)));
	}
	
	std::fs::create_dir_all(&path)?;

	let mut scripts = HashMap::new();
	scripts.insert("build".to_string(), "echo \"No build targets\" && exit 1".to_string());

	let config = Config {
		environment: EnvConfig {
			version: Version::parse(env!("CARGO_PKG_VERSION")).unwrap(),
		},
		workspace: None,
		repository: Some(RepositoryConfig {
			name,
			version: Version::new(0, 1, 0),
			build: None,
			git: Some(RepositoryGitConfig {
				url,
				path: "git".to_string()
			})
		}),
		scripts: Some(scripts),
		dependencies: None
	};

	write_toml(&path.join("batl.toml"), &config)?;

	success("Initialized repository clone successfully");

	Ok(())
}

fn cmd_scaffold() -> Result<(), UtilityError> {
	let config = Config::get_repository()
		.map_err(|_| UtilityError::InvalidConfig)?
		.ok_or(UtilityError::ResourceDoesNotExist("Repository Configuration".to_string()))?;


	let repository = config.repository.clone().expect("Nonsensical repository config without a repository");

	if let Some(git) = repository.git {
		let git_path = config.path()
			.expect("Nonsensical no path for config file")
			.parent()
			.expect("Nonsensical config path has no parent")
			.join(git.path);

		let mut fetch_callbacks = RemoteCallbacks::new();
		fetch_callbacks.transfer_progress(transfer_progress);

		let mut fetch_options = FetchOptions::new();
		fetch_options.remote_callbacks(fetch_callbacks);

		let result = RepoBuilder::new()
			.fetch_options(fetch_options)
			.clone(&git.url, &git_path);

		println!("");

		if let Err(err) = result {
			println!("{}", err);

			return Err(UtilityError::ResourceNotCollected("Git remote".to_string()));
		}

		success("Successfully scaffolded repository");
	}

	Ok(())
}

fn transfer_progress(progress: Progress<'_>) -> bool {
	let percentage = progress.received_objects() as f64 / progress.total_objects() as f64;

	let mut term = Term::stdout();

	term.clear_line().unwrap();
	term.write_fmt(format_args!("Cloning repository... {:.2}%", percentage * 100.0)).unwrap();
	term.flush().unwrap();



	true
}
