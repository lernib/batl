use crate::env::System;
use crate::utils::UtilityError;
use std::{path::PathBuf, env};

pub mod workspace;
pub mod link;
pub mod repository;

pub fn cmd_setup() -> Result<(), UtilityError> {
	if System::batl_root().is_some() {
		return Err(UtilityError::AlreadySetup);
	}

	let batl_root = PathBuf::from(env::var("HOME").map_err(
		|_| UtilityError::ResourceDoesNotExist("Home directory".to_string())
	)?).join("battalion");

	std::fs::create_dir_all(batl_root.join("workspaces"))?;
	std::fs::create_dir_all(batl_root.join("repositories"))?;
	std::fs::File::create(batl_root.join(".batlrc"))?;

	println!("Battalion root directory created at {}", batl_root.display());

	Ok(())  
}
