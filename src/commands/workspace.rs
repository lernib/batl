use std::collections::HashMap;
use std::path::PathBuf;

use clap::Subcommand;
use crate::utils::{get_batl_root, write_toml, UtilityError, BATL_NAME_REGEX};
use crate::config::*;
use crate::output::*;

#[derive(Subcommand)]
pub enum Commands {
  Ls,
  Init {
    name: String
  },
  Delete {
    name: String
  },
  Cd {
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
    Commands::Init { name } => {
      cmd_init(name)
    },
    Commands::Delete { name } => {
      cmd_delete(name)
    },
    Commands::Cd { name } => {
      cmd_cd(name)
    },
    Commands::Which { name } => {
      cmd_which(name)
    }
  }
}

fn cmd_ls() -> Result<(), UtilityError> {
  let workspace_root = get_batl_root()?.join("workspaces");

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

fn cmd_init(name: String) -> Result<(), UtilityError> {
  if !BATL_NAME_REGEX.is_match(&name) {
    return Err(UtilityError::InvalidName(name));
  }

  let workspace_root = get_batl_root()?.join("workspaces");

  let parts = name.split('/').collect::<Vec<&str>>();
  let mut path = workspace_root;

  for part in parts.iter().take(parts.len() - 1) {
    path = path.join(format!("@{}", part));
  }
  path = path.join(parts.last().unwrap());

  if path.exists() {
    return Err(UtilityError::ResourceAlreadyExists(format!("Workspace {}", name)));
  }

  std::fs::create_dir_all(path.clone())?;

  let batl_toml_path = path.join("batl.toml");
  let config = Config {
    environment: EnvConfig {
      version: env!("CARGO_PKG_VERSION").to_string(),
    },
    workspace: Some(HashMap::new()),
    repository: None,
    scripts: None
  };

  write_toml(&batl_toml_path, &config)?;

  success(&format!("Workspace {} initialized", name));

  Ok(())
}

fn cmd_delete(name: String) -> Result<(), UtilityError> {
  if !BATL_NAME_REGEX.is_match(&name) {
    return Err(UtilityError::InvalidName(name));
  }

  let workspace_root = get_batl_root()?.join("workspaces");

  let parts = name.split('/').collect::<Vec<&str>>();
  let mut path = workspace_root;

  for part in parts.iter().take(parts.len() - 1) {
    path = path.join(format!("@{}", part));
  }
  path = path.join(parts.last().unwrap());

  if !path.exists() {
    return Err(UtilityError::ResourceDoesNotExist(format!("Workspace {}", name)));
  }

  std::fs::remove_dir_all(path)?;

  success(&format!("Workspace {} deleted", name));

  Ok(())
}

fn cmd_cd(name: String) -> Result<(), UtilityError> {
  if !BATL_NAME_REGEX.is_match(&name) {
    return Err(UtilityError::InvalidName(name));
  }

  let workspace_root = get_batl_root()?.join("workspaces");

  let parts = name.split('/').collect::<Vec<&str>>();
  let mut path = workspace_root;

  for part in parts.iter().take(parts.len() - 1) {
    path = path.join(format!("@{}", part));
  }
  path = path.join(parts.last().unwrap());

  if !path.exists() {
    return Err(UtilityError::ResourceDoesNotExist(format!("Workspace {}", name)));
  }

  std::env::set_current_dir(path)?;

  success(&format!("Workspace {} selected", name));

  Ok(())
}

fn cmd_which(name: String) -> Result<(), UtilityError> {
  if !BATL_NAME_REGEX.is_match(&name) {
    return Err(UtilityError::InvalidName(name));
  }

  let workspace_root = get_batl_root()?.join("workspaces");

  let parts = name.split('/').collect::<Vec<&str>>();
  let mut path = workspace_root;

  for part in parts.iter().take(parts.len() - 1) {
    path = path.join(format!("@{}", part));
  }
  path = path.join(parts.last().unwrap());

  if !path.exists() {
    return Err(UtilityError::ResourceDoesNotExist(format!("Workspace {}", name)));
  }

  println!("{}", path.to_string_lossy());

  Ok(())
}
