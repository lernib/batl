use crate::utils;
use std::{path::PathBuf, collections::HashMap};
use regex::Regex;
use lazy_static::lazy_static;
use crate::config::*;

lazy_static! {
  static ref BATL_NAME_REGEX: Regex = Regex::new(r"^[a-z][a-z0-9\-_]*(/[a-z][a-z0-9\-_]*)+$").unwrap();
}

/****************************************
* ls
****************************************/
pub fn ls(all: bool) {
  if all {
    let batl_root = utils::get_batl_root().unwrap();

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
              search_dirs.push((path, format!("{}/{}", prefix, name.replace("@", ""))));
            } else {
              repos.push(format!("{}/{}", prefix, name));
            }
          }
        }
      }
    }

    repos.sort();

    for repo in repos {
      println!("{}", repo);
    }
  } else {
    let config = utils::get_workspace_config().unwrap();

    for (code, name) in config.links.iter() {
      println!("{}: {}", code, name);
    }
  }
}

/****************************************
* init
****************************************/
pub fn init(workspace: bool, name: String) {
  if !BATL_NAME_REGEX.is_match(&name) {
    println!("Invalid name: {}", name);
    return;
  }

  let batl_root = utils::get_batl_root().unwrap();

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

  std::fs::create_dir_all(&path).unwrap();

  let mut batl_toml = path.clone();
  batl_toml.push("batl.toml");

  let config = Config {
    name: name.clone(),
    languages: vec![],
    links: HashMap::new()
  };

  utils::write_toml(&batl_toml, &config).unwrap();

  println!("Initialized {} {}", if workspace { "workspace" } else { "repository" }, name);
}

/****************************************
* link
****************************************/
pub fn link(name: String) {
  if !BATL_NAME_REGEX.is_match(&name) {
    println!("Invalid name: {}", name);
    return;
  }

  let mut config = utils::get_workspace_config().unwrap();

  let repo_base = utils::get_batl_root().unwrap().join("repositories");

  let mut path = repo_base.clone();
  let parts: Vec<&str> = name.split("/").collect();
  
  for part in parts.iter().take(parts.len() - 1) {
    path = path.join(format!("@{}", part));
  }
  path = path.join(parts.last().unwrap());

  if !path.exists() {
    println!("Repository does not exist: {}", name);
    std::process::exit(1);
  }

  let repo_code = utils::rand8();

  let workspace_dir = utils::get_batl_toml_dir().unwrap();
  config.links.insert(format!("r{}", repo_code), name.clone());

  utils::write_toml(&workspace_dir.join("batl.toml"), &config).unwrap();

  std::os::unix::fs::symlink(path, workspace_dir.join(format!("r{}", repo_code))).unwrap();

  println!("Linked {} to {}", name, format!("r{}", repo_code));
}