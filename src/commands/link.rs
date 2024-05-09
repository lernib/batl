use clap::{Subcommand, ValueEnum};
use crate::config::Config;
use crate::env::{Resource, System};
use crate::utils::{
	write_toml,
	UtilityError, BATL_LINK_REGEX, BATL_NAME_REGEX
};
use crate::output::*;
use std::env::current_dir;

#[derive(Subcommand)]
pub enum Commands {
	Ls,
	Stats {
		#[arg(long = "get")]
		get: Option<StatsGet>,
		name: String
	},
	Init {
		#[arg(short = 'n', long = "name")]
		name: Option<String>,
		repo: String
	},
	Delete {
		name: String
	},
	Run {
		name: String,
		#[arg(last = true)]
		args: Vec<String>
	},
	Exec {
		#[arg(short = 'n')]
		name: Option<String>,
		script: String
	}
}

pub fn run(cmd: Commands) -> Result<(), UtilityError> {
	match cmd {
		Commands::Ls => {
			cmd_ls()
		},
		Commands::Stats { name, get } => {
			cmd_stats(name, get)
		},
		Commands::Init { name, repo } => {
			cmd_init(name, repo)
		},
		Commands::Delete { name } => {
			cmd_delete(name)
		},
		Commands::Run { name, args } => {
			cmd_run(name, args)
		},
		Commands::Exec { name, script } => {
			cmd_exec(name, script)
		}
	}
}

fn cmd_ls() -> Result<(), UtilityError> {
	let workspace_config = Config::get_workspace()
		.map_err(|_| UtilityError::InvalidConfig)?
		.ok_or(UtilityError::ResourceDoesNotExist("Workspace Configuration".to_string()))?;

	let links = workspace_config.workspace.unwrap();

	for link in links {
		println!("{}:\t\t{}", link.0, link.1);
	}

	Ok(())
}

#[derive(Clone, ValueEnum)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
enum StatsGet {
	Name,
	Repository
}

fn cmd_stats(name: String, get: Option<StatsGet>) -> Result<(), UtilityError> {
	let workspace_config = Config::get_workspace()
		.map_err(|_| UtilityError::InvalidConfig)?
		.ok_or(UtilityError::ResourceDoesNotExist("Workspace Configuration".to_string()))?;

	let links = workspace_config.workspace.unwrap();

	let link = links.get(&name).ok_or(UtilityError::LinkNotFound)?;

	match get {
		None => {
			println!("Link: {}", name);
			println!("Repository: {}", link);
		},
		Some(StatsGet::Name) => println!("{name}"),
		Some(StatsGet::Repository) => println!("{link}")
	}

	Ok(())
}

fn cmd_init(name: Option<String>, repo: String) -> Result<(), UtilityError> {
	if !BATL_NAME_REGEX.is_match(&repo) {
		return Err(UtilityError::InvalidName(repo));
	}

	// TODO: Make random string
	let name = name.unwrap_or_else(|| unimplemented!());

	if !BATL_LINK_REGEX.is_match(&name) {
		return Err(UtilityError::InvalidName(name));
	}

	let repo_path = System::repository(repo.as_str().into())
		.ok_or(UtilityError::ResourceDoesNotExist("Battalion root".to_string()))?
		.path().to_path_buf();

	let mut workspace_config = Config::get_workspace()
		.map_err(|_| UtilityError::InvalidConfig)?
		.ok_or(UtilityError::ResourceDoesNotExist("Workspace Configuration".to_string()))?;

	let mut links = workspace_config.workspace.unwrap();

	if links.contains_key(&name) {
		return Err(UtilityError::ResourceAlreadyExists(format!("Link {}", name)));
	}

	links.insert(name.clone(), repo.clone());

	workspace_config.workspace = Some(links);

	let workspace_dir = Config::toml_dir(&current_dir()?)
		.ok_or(UtilityError::ResourceDoesNotExist("Workspace Configuration".to_string()))?;

	write_toml(&workspace_dir.join("batl.toml"), &workspace_config)?;

	std::os::unix::fs::symlink(repo_path, workspace_dir.join(&name))?;

	success(&format!("Initialized link {}", name));

	Ok(())
}

fn cmd_delete(name: String) -> Result<(), UtilityError> {
	let mut workspace_config = Config::get_workspace()
		.map_err(|_| UtilityError::InvalidConfig)?
		.ok_or(UtilityError::ResourceDoesNotExist("Workspace Configuration".to_string()))?;

	let mut links = workspace_config.workspace.unwrap();

	if !links.contains_key(&name) {
		return Err(UtilityError::LinkNotFound);
	}

	links.remove(&name);

	workspace_config.workspace = Some(links);

	let workspace_dir = Config::toml_dir(&current_dir()?)
		.ok_or(UtilityError::ResourceDoesNotExist("Workspace Configuration".to_string()))?;

	write_toml(&workspace_dir.join("batl.toml"), &workspace_config)?;

	std::fs::remove_file(workspace_dir.join(&name))?;

	success(&format!("Deleted link {}", name));

	Ok(())
}

fn cmd_run(name: String, args: Vec<String>) -> Result<(), UtilityError> {
	let workspace_config = Config::get_workspace()
		.map_err(|_| UtilityError::InvalidConfig)?
		.ok_or(UtilityError::ResourceDoesNotExist("Workspace Configuration".to_string()))?;

	let links = workspace_config.workspace.unwrap();
	
	links.get(&name).ok_or(UtilityError::LinkNotFound)?;

	info(&format!("Running command for link {}\n", name));

	let workspace_dir = Config::toml_dir(&current_dir()?)
		.ok_or(UtilityError::ResourceDoesNotExist("Workspace Configuration".to_string()))?;

	std::process::Command::new(args.first().unwrap())
		.current_dir(workspace_dir.join(&name))
		.args(args.iter().skip(1))
		.status()?;

	println!("");
	success("Command completed successfully");

	Ok(())
}

fn cmd_exec(name: Option<String>, script: String) -> Result<(), UtilityError> {
	if let Some(name) = &name {
		let workspace_config = Config::get_workspace()
			.map_err(|_| UtilityError::InvalidConfig)?
			.ok_or(UtilityError::ResourceDoesNotExist("Workspace Configuration".to_string()))?;

		let links = workspace_config.workspace.unwrap();
		
		links.get(name).ok_or(UtilityError::LinkNotFound)?;
	}

	let mut workspace_dir = Config::toml_dir(&current_dir()?)
		.ok_or(UtilityError::ResourceDoesNotExist("Workspace Configuration".to_string()))?;

	if let Some(name) = &name {
		workspace_dir.push(name);
	}

	let repository_config = Config::get_repository_from_dir(&workspace_dir)
		.map_err(|_| UtilityError::InvalidConfig)?
		.ok_or(UtilityError::ResourceDoesNotExist("Workspace Configuration".to_string()))?;

	let scripts = match repository_config.scripts {
		Some(scripts) => scripts,
		None => return Err(UtilityError::NoScripts)
	};

	if !scripts.contains_key(&script) {
		return Err(UtilityError::ScriptNotFound(script));
	}

	info(&format!("Running script{}\n", name.map(|s| format!(" for link {}", s)).unwrap_or("".to_string())));

	let status = std::process::Command::new("sh")
		.current_dir(workspace_dir)
		.arg("-c")
		.arg(scripts.get(&script).unwrap())
		.status()?;


	if !status.success() {
		return Err(UtilityError::ScriptError(format!("Exit code {}", status.code().unwrap_or(0))))
	}

	println!("");
	success("Script completed successfully");

	Ok(())
}
