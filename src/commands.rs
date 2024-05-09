use crate::config::Config;
use crate::env::System;
use crate::output::success;
use crate::utils::{UtilityError, write_toml};
use std::{collections::HashMap, env, path::PathBuf};

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

pub fn cmd_add(name: String) -> Result<(), UtilityError> {
	let config_path = Config::get_path_on_condition(|_| true)
		.map_err(|_| UtilityError::InvalidConfig)?
		.ok_or(UtilityError::ResourceDoesNotExist("Batallion config".to_string()))?;

	let mut config = Config::read(&config_path)
		.map_err(|_| UtilityError::InvalidConfig)?;

	if let Some(mut deps) = config.dependencies {
		deps.insert(name.clone(), "latest".to_string());

		config.dependencies = Some(deps);
	} else {
		let mut deps = HashMap::new();
		deps.insert(name.clone(), "latest".to_string());

		config.dependencies = Some(deps);
	}

	write_toml(&config_path, &config)?;

	success(&format!("Added dependency {}", name));

	Ok(())
}

pub fn cmd_remove(name: String) -> Result<(), UtilityError> {
	let config_path = Config::get_path_on_condition(|_| true)
		.map_err(|_| UtilityError::InvalidConfig)?
		.ok_or(UtilityError::ResourceDoesNotExist("Batallion config".to_string()))?;

	let mut config = Config::read(&config_path)
		.map_err(|_| UtilityError::InvalidConfig)?;

	if let Some(mut deps) = config.dependencies {
		if deps.remove(&name).is_none() {
			return Err(UtilityError::ResourceDoesNotExist("Dependency".to_string()))
		}

		config.dependencies = Some(deps);
	} else {
		return Err(UtilityError::ResourceDoesNotExist("Dependency".to_string()));
	}

	write_toml(&config_path, &config)?;

	success(&format!("Removed dependency {}", name));

	Ok(())
}

pub fn cmd_upgrade() -> Result<(), UtilityError> {
	let batl_root = System::batl_root()
		.ok_or(UtilityError::ResourceDoesNotExist("Battalion root".to_string()))?;

	if !batl_root.join("gen").exists() {
		let gen_ = batl_root.join("gen");

		std::fs::create_dir(&gen_)?;
		std::fs::create_dir(&gen_.join("archives"))?;
		std::fs::create_dir(&gen_.join("archives/repositories"))?;
		std::fs::create_dir(&gen_.join("archives/workspaces"))?;

		success("Added gen folder");
	}

	Ok(())
}
