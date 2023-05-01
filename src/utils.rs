use crate::config::Config;
use thiserror::Error;
use std::{
  path::PathBuf,
  env::var,
  io::Write
};

#[derive(Error, Debug)]
pub enum UtilityError {
  #[error("IO Error: {0}")]
  IoError(#[from] std::io::Error),
  #[error("Resource does not exist: {0}")]
  ResourceDoesNotExist(String),
  #[error("Invalid config")]
  InvalidConfig
}

pub fn get_batl_root() -> Result<PathBuf, UtilityError> {
  // 1. Check BATL_ROOT environment variable
  if let Ok(batl_root) = var("BATL_ROOT") {
    return Ok(PathBuf::from(batl_root));
  }

  // 2. Recursively descend from current directory until .batlrc is found
  let mut current_dir = std::env::current_dir()?;

  loop {
    if current_dir.join(".batlrc").exists() {
      return Ok(current_dir);
    }

    if !current_dir.pop() {
      break;
    }
  }

  // 3. Check for battalion folder in home directory
  if let Ok(home_dir) = var("HOME") {
    let batl_dir = PathBuf::from(home_dir).join("battalion");

    if batl_dir.exists() {
      return Ok(batl_dir);
    }
  }

  Err(UtilityError::ResourceDoesNotExist("Battalion root directory".to_string()))
}

pub fn write_toml<T: serde::Serialize>(path: &PathBuf, data: &T) -> Result<(), UtilityError> {
  let mut file = std::fs::File::create(path)?;

  file.write_all(toml::to_string(data).unwrap().as_bytes())?;

  Ok(())
}

pub fn write_string(path: &PathBuf, data: &str) -> Result<(), UtilityError> {
  let mut file = std::fs::File::create(path)?;

  file.write_all(data.as_bytes())?;

  Ok(())
}

pub fn get_workspace_config() -> Result<Config, UtilityError> {
  let batl_toml_path = get_batl_toml_dir()?.join("batl.toml");

  let config_str = std::fs::read_to_string(batl_toml_path)?;

  let config: Config = toml::from_str(&config_str).map_err(|_| UtilityError::InvalidConfig)?;

  if config.workspace.is_none() {
    return Err(UtilityError::InvalidConfig);
  }

  Ok(config)
}

pub fn get_repo_config(path: &PathBuf) -> Result<Config, UtilityError> {
  let batl_toml_path = path.join("batl.toml");

  let config_str = std::fs::read_to_string(batl_toml_path)?;

  let config: Config = toml::from_str(&config_str).map_err(|_| UtilityError::InvalidConfig)?;

  if config.repository.is_none() {
    return Err(UtilityError::InvalidConfig);
  }

  Ok(config)
}

pub fn get_batl_toml_dir() -> Result<PathBuf, UtilityError> {
  let mut current_path = std::env::current_dir()?;

  loop {
    let batl_toml = current_path.join("batl.toml");

    if batl_toml.exists() {
      break Ok(current_path);
    }

    if !current_path.pop() {
      break Err(UtilityError::ResourceDoesNotExist("Workspace config".to_string()));
    }
  }
}

pub fn rand8() -> String {
  let random_num: u64 = rand::random();
  format!("{:x}", random_num)[..8].to_string()
}