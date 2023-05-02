use std::path::PathBuf;

use clap::Subcommand;
use crate::utils::{get_batl_root, UtilityError};

#[derive(Subcommand)]
pub enum Commands {
  Ls
}

pub fn run(cmd: Commands) -> Result<(), UtilityError> {
  match cmd {
    Commands::Ls => {
      cmd_ls()
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