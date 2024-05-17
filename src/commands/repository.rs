use clap::Subcommand;
use console::Term;
use crate::config::*;
use crate::env::{CreateRepositoryOptions, Repository, Resource, System};
use crate::output::*;
use crate::utils::{UtilityError, BATL_NAME_REGEX};use envfile::EnvFile;
use git2::{FetchOptions, RemoteCallbacks, Progress};
use git2::build::RepoBuilder;
use std::env::current_dir;
use std::io::Write;
use std::path::PathBuf;


#[derive(Subcommand)]
pub enum Commands {
	Ls {
		filter: Option<String>
	},
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
	Scaffold,
	Env {
		#[arg(short = 'n')]
		name: Option<String>,
		var: String
	},
	Archive {
		name: String
	},
	Publish {
		name: String
	},
	Which {
		name: String
	},
	Exec {
		#[arg(short = 'n')]
		name: Option<String>,
		script: String
	}
}

pub fn run(cmd: Commands) -> Result<(), UtilityError> {
	match cmd {
		Commands::Ls { filter } => {
			cmd_ls(filter)
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
		},
		Commands::Env { name, var } => {
			cmd_env(name, var)
		},
		Commands::Archive { name } => {
			cmd_archive(name)
		},
		Commands::Publish { name } => {
			cmd_publish(name)
		},
		Commands::Which { name } => {
			cmd_which(name)
		},
		Commands::Exec { name, script } => {
			cmd_exec(name, script)
		}
	}
}

fn cmd_ls(filter: Option<String>) -> Result<(), UtilityError> {
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
		if let Some(filter_str) = &filter {
			if !name.starts_with(filter_str) {
				continue;
			}
		}

		println!("{}", name);
	}

	Ok(())
}

fn cmd_init(name: String) -> Result<(), UtilityError> {
	if !BATL_NAME_REGEX.is_match(&name) {
		return Err(UtilityError::InvalidName(name));
	}

	Repository::create(name.into(), Default::default())?;

	success("Initialized repository successfully");

	Ok(())
}

fn cmd_delete(name: String) -> Result<(), UtilityError> {
	if !BATL_NAME_REGEX.is_match(&name) {
		return Err(UtilityError::InvalidName(name));
	}

	Repository::load(name.into())?
		.ok_or(UtilityError::ResourceDoesNotExist("Repository".to_string()))?
		.destroy()?;

	success("Deleted repository successfully");

	Ok(())
}

fn cmd_clone(url: String, name: String) -> Result<(), UtilityError> {
	if !BATL_NAME_REGEX.is_match(&name) {
		return Err(UtilityError::InvalidName(name));
	}

	Repository::create(name.into(), CreateRepositoryOptions {
		git: Some(RepositoryGitConfig {
			url,
			path: "git".to_string()
		}),
		..Default::default()
	})?;

	success("Initialized repository clone successfully");

	Ok(())
}

fn cmd_scaffold() -> Result<(), UtilityError> {
	let repository = Repository::locate_then_load(&current_dir()?)?
		.ok_or(UtilityError::ResourceDoesNotExist("Repository".to_string()))?;

	let config = repository.config();

	if let Some(git) = config.git {
		let git_path = repository.path().join(git.path);

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

fn cmd_env(name: Option<String>, var: String) -> Result<(), UtilityError> {
	let mut workspace_dir = Config::toml_dir(&current_dir()?)
		.ok_or(UtilityError::ResourceDoesNotExist("Workspace Configuration".to_string()))?;

	if let Some(name) = &name {
		workspace_dir.push(name);
	}

	let env_file = EnvFile::new(&workspace_dir.join("batl.env"))
		.map_err(|_| UtilityError::ResourceDoesNotExist("Environment variables".to_string()))?;

	if let Some(val) = env_file.get(&var) {
		println!("{}", val);
	}

	Ok(())
}

fn cmd_archive(name: String) -> Result<(), UtilityError> {
	let repository = Repository::load(name.as_str().into())?
		.ok_or(UtilityError::ResourceDoesNotExist("Repository".into()))?;

	repository.archive_gen()?;

	Ok(())
}

fn cmd_publish(name: String) -> Result<(), UtilityError> {
	let repository = Repository::load(name.as_str().into())?
		.ok_or(UtilityError::ResourceDoesNotExist("Repository".into()))?;

	let archive = repository.archive()
		.ok_or(UtilityError::ResourceDoesNotExist("Archive".into()))?;

	let resp = ureq::post("https://api.batl.circetools.net/pkg/upload")
		.query("id", &repository.name().to_string())
		.send(archive.to_file())?;

	if resp.status() == 200 {
		success(&format!("Published repository {}", name))
	} else {
		error(&format!("Failed to send repository: status code {}", resp.status()))
	}

	Ok(())
}

fn cmd_which(name: String) -> Result<(), UtilityError> {
	if !BATL_NAME_REGEX.is_match(&name) {
		return Err(UtilityError::InvalidName(name));
	}

	let workspace = Repository::load(name.into())?
		.ok_or(UtilityError::ResourceDoesNotExist("Workspace".into()))?;

	println!("{}", workspace.path().to_string_lossy());

	Ok(())
}

fn cmd_exec(name: Option<String>, script: String) -> Result<(), UtilityError> {
	let repository = match &name {
		Some(val) => {
			Repository::load(val.as_str().into())?
		},
		None => Repository::locate_then_load(&current_dir()?)?
	}.ok_or(UtilityError::ResourceDoesNotExist("Repository".to_string()))?;

	let command = repository.script(&script)
		.ok_or(UtilityError::ScriptNotFound(script))?;

	info(&format!("Running script{}\n", name.map(|s| format!(" for link {}", s)).unwrap_or("".to_string())));

	let status = std::process::Command::new("sh")
		.current_dir(repository.path())
		.arg("-c")
		.arg(command)
		.status()?;


	if !status.success() {
		return Err(UtilityError::ScriptError(format!("Exit code {}", status.code().unwrap_or(0))))
	}

	println!("");
	success("Script completed successfully");

	Ok(())
}
