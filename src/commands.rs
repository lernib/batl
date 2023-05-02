use crate::utils::{UtilityError, get_batl_root};
use std::{path::PathBuf, env};

pub mod workspace;
pub mod link;
pub mod repository;


pub fn cmd_setup() -> Result<(), UtilityError> {
  if !get_batl_root().is_err() {
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