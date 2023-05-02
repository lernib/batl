use clap::Subcommand;
use crate::utils::{get_workspace_config, get_batl_toml_dir, write_toml, UtilityError, BATL_LINK_REGEX, BATL_NAME_REGEX, get_batl_root};
use crate::output::*;

#[derive(Subcommand)]
pub enum Commands {
  Ls,
  Stats {
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
  }
}

pub fn run(cmd: Commands) -> Result<(), UtilityError> {
  match cmd {
    Commands::Ls => {
      cmd_ls()
    },
    Commands::Stats { name } => {
      cmd_stats(name)
    },
    Commands::Init { name, repo } => {
      cmd_init(name, repo)
    },
    Commands::Delete { name } => {
      cmd_delete(name)
    },
    Commands::Run { name, args } => {
      cmd_run(name, args)
    }
  }
}

fn cmd_ls() -> Result<(), UtilityError> {
  let workspace_config = get_workspace_config()?;

  let links = workspace_config.workspace.unwrap();

  for link in links {
    println!("{}:\t\t{}", link.0, link.1);
  }

  Ok(())
}

fn cmd_stats(name: String) -> Result<(), UtilityError> {
  let workspace_config = get_workspace_config()?;

  let links = workspace_config.workspace.unwrap();

  let link = links.get(&name).ok_or(UtilityError::LinkNotFound)?;

  println!("Link: {}", name);
  println!("Repository: {}", link);

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

  let parts = repo.split("/").collect::<Vec<&str>>();
  let mut repo_root = get_batl_root()?.join("repositories");

  for part in parts.iter().take(parts.len() - 1) {
    repo_root.push(format!("@{}", part));
  }

  repo_root.push(parts.last().unwrap());

  let mut workspace_config = get_workspace_config()?;

  let mut links = workspace_config.workspace.unwrap();

  if links.contains_key(&name) {
    return Err(UtilityError::ResourceAlreadyExists(format!("Link {}", name)));
  }

  links.insert(name.clone(), repo.clone());

  workspace_config.workspace = Some(links);

  write_toml(&get_batl_toml_dir()?.join("batl.toml"), &workspace_config)?;

  std::os::unix::fs::symlink(repo_root, get_batl_toml_dir()?.join(&name))?;

  success(&format!("Initialized link {}", name));

  Ok(())
}

fn cmd_delete(name: String) -> Result<(), UtilityError> {
  let mut workspace_config = get_workspace_config()?;

  let mut links = workspace_config.workspace.unwrap();

  if !links.contains_key(&name) {
    return Err(UtilityError::LinkNotFound);
  }

  links.remove(&name);

  workspace_config.workspace = Some(links);

  write_toml(&get_batl_toml_dir()?.join("batl.toml"), &workspace_config)?;

  std::fs::remove_file(get_batl_toml_dir()?.join(&name))?;

  success(&format!("Deleted link {}", name));

  Ok(())
}

fn cmd_run(name: String, args: Vec<String>) -> Result<(), UtilityError> {
  let workspace_config = get_workspace_config()?;

  let links = workspace_config.workspace.unwrap();
  
  links.get(&name).ok_or(UtilityError::LinkNotFound)?;

  info(&format!("Running command for link {}\n", name));

  std::process::Command::new(args.first().unwrap())
    .current_dir(get_batl_toml_dir()?.join(&name))
    .args(args.iter().skip(1))
    .status()?;

  println!("");
  success("Command completed successfully");

  Ok(())
}