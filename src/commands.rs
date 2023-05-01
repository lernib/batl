use crate::utils;
use crate::config::*;
use std::{path::PathBuf, collections::HashMap};
use regex::Regex;
use lazy_static::lazy_static;
use dialoguer::Confirm;
use thiserror::Error;

lazy_static! {
  static ref BATL_NAME_REGEX: Regex = Regex::new(r"^[a-z][a-z0-9\-_]*(/[a-z][a-z0-9\-_]*)+$").unwrap();
  static ref BATL_LINK_REGEX: Regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_\-]*$").unwrap();
}

/****************************************
* BatlError
****************************************/
#[derive(Error, Debug)]
pub enum BatlError {
  #[error("{0}")]
  UtilityError(#[from] utils::UtilityError),
  #[error("Invalid battalion name: {0}")]
  InvalidBattalionName(String)
}

pub type CmdResult<T> = Result<T, BatlError>;

/****************************************
* ls
****************************************/
pub fn ls(all: bool) -> CmdResult<()> {
  if all {
    let batl_root = utils::get_batl_root()?;

    let repo_root = batl_root.join("repositories");

    let mut repos: Vec<String> = vec![];
    let mut search_dirs: Vec<(PathBuf, String)> = vec![
      (repo_root, "".to_string())
    ];

    while let Some((dir, prefix)) = search_dirs.pop() {
      if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries {
          if let Ok(entry) = entry {
            let path = entry.path();

            if !path.is_dir() {
              continue;
            }

            let name = path.file_name().unwrap().to_str().unwrap().to_string();

            if name.starts_with("@") {
              search_dirs.push((path, format!("{}{}{}", prefix, {
                if prefix.len() > 0 {
                  "/"
                } else {
                  ""
                }
              }, name.replace("@", ""))));
            } else {
              repos.push(format!("{}{}{}", prefix, {
                if prefix.len() > 0 {
                  "/"
                } else {
                  ""
                }
              }, name));
            }
          }
        }
      }
    }

    repos.sort();

    for repo in repos {
      println!("{}", repo);
    }

    Ok(())
  } else {
    let config = utils::get_workspace_config()?;

    for (code, name) in config.workspace.unwrap().iter() {
      println!("{}: {}", code, name);
    }

    Ok(())
  }
}

/****************************************
* init
****************************************/
pub fn init(workspace: bool, name: String) -> CmdResult<()> {
  if !BATL_NAME_REGEX.is_match(&name) {
    return Err(BatlError::InvalidBattalionName(name));
  }

  let batl_root = utils::get_batl_root()?;

  let mut batl_base: PathBuf = batl_root.clone();
  if workspace {
    batl_base = batl_base.join("workspaces");
  } else {
    batl_base = batl_base.join("repositories");
  }

  let mut path = batl_base.clone();
  let parts: Vec<&str> = name.split("/").collect();
  
  for part in parts.iter().take(parts.len() - 1) {
    path = path.join(format!("@{}", part));
  }
  path = path.join(parts.last().unwrap());

  if path.exists() {
    if workspace {
      println!("Workspace already exists: {}", name);
    } else {
      println!("Repository already exists: {}", name);
    }

    std::process::exit(1);
  }

  std::fs::create_dir_all(&path)
    .map_err(|e| BatlError::UtilityError(utils::UtilityError::IoError(e)))?;

  let mut batl_toml = path.clone();
  batl_toml.push("batl.toml");

  let config: Config;
  if workspace {
    config = Config {
      environment: EnvConfig {
        min_version: env!("CARGO_PKG_VERSION").to_string()
      },
      workspace: Some(HashMap::new()),
      repository: None
    }
  } else {
    config = Config {
      environment: EnvConfig {
        min_version: env!("CARGO_PKG_VERSION").to_string()
      },
      workspace: None,
      repository: Some(RepositoryConfig {
        name: parts.last().unwrap().to_string(),
        version: "0.0.1".to_string()
      })
    }
  }

  utils::write_toml(&batl_toml, &config)?;

  println!("Initialized {} {}", if workspace { "workspace" } else { "repository" }, name);

  Ok(())
}

/****************************************
* purge
****************************************/
pub fn purge(workspace: bool, name: String) -> CmdResult<()> {
  if !BATL_NAME_REGEX.is_match(&name) {
    return Err(BatlError::InvalidBattalionName(name));
  }

  let batl_root = utils::get_batl_root()?;

  let mut batl_base: PathBuf = batl_root.clone();
  if workspace {
    batl_base = batl_base.join("workspaces");
  } else {
    batl_base = batl_base.join("repositories");
  }

  let mut path = batl_base.clone();
  let parts: Vec<&str> = name.split("/").collect();

  for part in parts.iter().take(parts.len() - 1) {
    path = path.join(format!("@{}", part));
  }
  path = path.join(parts.last().unwrap());

  if !path.exists() {
    if workspace {
      println!("Workspace does not exist: {}", name);
    } else {
      println!("Repository does not exist: {}", name);
    }

    std::process::exit(1);
  }

  if !Confirm::new().default(false).with_prompt(format!("Are you sure you want to purge {} {}? (there's no undo command!)", if workspace { "workspace" } else { "repository" }, name)).interact()
    .map_err(|e| BatlError::UtilityError(utils::UtilityError::IoError(e)))? {
    println!("Aborted");
    std::process::exit(1);
  }

  std::fs::remove_dir_all(&path)
    .map_err(|e| BatlError::UtilityError(utils::UtilityError::IoError(e)))?;

  println!("Purged {} {}", if workspace { "workspace" } else { "repository" }, name);

  Ok(())
}

/****************************************
* link
****************************************/
pub fn link(name: Option<String>, repo: String) -> CmdResult<()> {
  if !BATL_NAME_REGEX.is_match(&repo) {
    return Err(BatlError::InvalidBattalionName(repo));
  }

  let mut config = utils::get_workspace_config()?;

  let repo_base = utils::get_batl_root()?.join("repositories");

  let mut path = repo_base.clone();
  let parts: Vec<&str> = repo.split("/").collect();
  
  for part in parts.iter().take(parts.len() - 1) {
    path = path.join(format!("@{}", part));
  }
  path = path.join(parts.last().unwrap());

  if !path.exists() {
    println!("Repository does not exist: {}", repo);
    std::process::exit(1);
  }

  let mut repo_code: String;

  if let Some(name) = name {
    if !BATL_LINK_REGEX.is_match(&name) {
      println!("Invalid link name: {}", name);
      std::process::exit(1);
    }

    if config.clone().workspace.unwrap().contains_key(&name) {
      println!("Link already exists: {}", name);
      std::process::exit(1);
    }

    repo_code = name.clone();
  } else {
    loop {
      repo_code = format!("r{}", utils::rand8());

      if !config.clone().workspace.unwrap().contains_key(&repo_code) {
        break;
      }
    }
  }

  let workspace_dir = utils::get_batl_toml_dir()?;
  config.workspace = {
    let mut workspace = config.workspace.unwrap();
    workspace.insert(repo_code.clone(), repo.clone());
    Some(workspace)
  };

  utils::write_toml(&workspace_dir.join("batl.toml"), &config)?;

  std::os::unix::fs::symlink(path, workspace_dir.join(&repo_code))
    .map_err(|e| BatlError::UtilityError(utils::UtilityError::IoError(e)))?;

  println!("Linked {} to {}", repo, repo_code);

  Ok(())
}

/****************************************
* unlink
****************************************/
pub fn unlink(name: String) -> CmdResult<()> {
  let mut config = utils::get_workspace_config()?;

  match config.clone().workspace.unwrap().get(&name) {
    None => {
      println!("Link does not exist: {}", name);
      std::process::exit(1);
    },
    Some(..) => {}
  }

  let workspace_dir = utils::get_batl_toml_dir()?;
  let repo_base = workspace_dir.join(&name);

  std::fs::remove_file(&repo_base)
    .map_err(|e| BatlError::UtilityError(utils::UtilityError::IoError(e)))?;

  config.workspace = {
    let mut workspace = config.workspace.unwrap();
    workspace.remove(&name);
    Some(workspace)
  };

  utils::write_toml(&workspace_dir.join("batl.toml"), &config)?;

  println!("Unlinked {}", name);

  Ok(())
}

/****************************************
* run
****************************************/
pub fn run(repo: String, cmd: Vec<String>) -> CmdResult<()> {
  let config = utils::get_workspace_config()?;

  match config.workspace.unwrap().get(&repo) {
    None => {
      println!("Repository not linked: {}", repo);
      std::process::exit(1);
    },
    Some(..) => {}
  }

  let repo_base = utils::get_batl_toml_dir()?.join(repo.clone());

  let cmd_first = cmd.first().unwrap();
  let cmd_rest = &cmd[1..];

  std::process::Command::new(cmd_first)
    .current_dir(repo_base)
    .args(cmd_rest)
    .spawn()
    .unwrap()
    .wait()
    .unwrap();

  println!("Ran {} in {}", cmd_first, repo);

  Ok(())
}

/****************************************
* alias_rename
****************************************/
pub fn alias_rename(old: String, new: String) -> CmdResult<()> {
  let mut config = utils::get_workspace_config()?;

  match config.clone().workspace.unwrap().get(&old) {
    None => {
      println!("Repository not linked: {}", old);
      std::process::exit(1);
    },
    Some(..) => {}
  }

  match config.clone().workspace.unwrap().get(&new) {
    Some(..) => {
      println!("Alias already linked: {}", new);
      std::process::exit(1);
    },
    None => {}
  }

  let old_name = config.clone().workspace.unwrap().remove(&old).unwrap();
  config.workspace = {
    let mut map = config.workspace.unwrap();

    map.insert(new.clone(), old_name);
    Some(map)
  };

  let workspace_dir = utils::get_batl_toml_dir()?;
  utils::write_toml(&workspace_dir.join("batl.toml"), &config)?;

  std::fs::rename(workspace_dir.join(&old), workspace_dir.join(&new))
    .map_err(|e| BatlError::UtilityError(utils::UtilityError::IoError(e)))?;

  println!("Renamed {} to {}", old, new);

  Ok(())
}