use clap::Subcommand;
use crate::env::{Repository, Resource, ResourceName, System, Workspace};
use crate::output::*;
use crate::utils::{UtilityError, BATL_NAME_REGEX};
use std::path::PathBuf;


#[derive(Subcommand)]
pub enum Commands {
	Ls,
	Init {
		name: String,
		#[arg(long = "ref")]
		ref_: bool
	},
	Delete {
		name: String
	},
	Which {
		name: String
	}
}

pub fn run(cmd: Commands) -> Result<(), UtilityError> {
	match cmd {
		Commands::Ls => {
			cmd_ls()
		},
		Commands::Init { name, ref_ } => {
			cmd_init(name, ref_)
		},
		Commands::Delete { name } => {
			cmd_delete(name)
		},
		Commands::Which { name } => {
			cmd_which(name)
		}
	}
}

fn cmd_ls() -> Result<(), UtilityError> {
	let workspace_root = System::workspace_root()
		.ok_or(UtilityError::ResourceDoesNotExist("Workspace root".to_string()))?;

	let mut to_search: Vec<(String, PathBuf)> = std::fs::read_dir(workspace_root)?
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

fn cmd_init(name: String, ref_: bool) -> Result<(), UtilityError> {
	if !BATL_NAME_REGEX.is_match(&name) {
		return Err(UtilityError::InvalidName(name));
	}

	let name: ResourceName = name.into();

	if ref_ {
		let mut repository = Repository::load(name.clone())?
			.ok_or(UtilityError::ResourceDoesNotExist("Repository".to_string()))?;

		Workspace::create_from_repository(&mut repository)?;

		success(&format!("Repository {} workspace created", name));
	} else {
		Workspace::create(name.clone())?;

		success(&format!("Workspace {} initialized", name.clone()));
	}

	Ok(())
}

fn cmd_delete(name: String) -> Result<(), UtilityError> {
	if !BATL_NAME_REGEX.is_match(&name) {
		return Err(UtilityError::InvalidName(name));
	}

	let workspace = Workspace::load(name.as_str().into())?
		.ok_or(UtilityError::ResourceDoesNotExist("Workspace".into()))?;

	workspace.destroy()?;

	success(&format!("Workspace {} deleted", name));

	Ok(())
}

fn cmd_which(name: String) -> Result<(), UtilityError> {
	if !BATL_NAME_REGEX.is_match(&name) {
		return Err(UtilityError::InvalidName(name));
	}

	let workspace = Workspace::load(name.into())?
		.ok_or(UtilityError::ResourceDoesNotExist("Workspace".into()))?;

	println!("{}", workspace.path().to_string_lossy());

	Ok(())
}
