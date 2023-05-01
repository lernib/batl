use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
  pub environment: EnvConfig,
  pub workspace: Option<HashMap<String, String>>,
  pub repository: Option<RepositoryConfig>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EnvConfig {
  pub version: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RepositoryConfig {
  pub name: String,
  pub version: String,
  pub build: String
}