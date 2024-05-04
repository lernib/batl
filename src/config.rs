use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/****************************************
* Config Formats
****************************************/
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
	pub environment: EnvConfig,
	pub workspace: Option<HashMap<String, String>>,
	pub repository: Option<RepositoryConfig>,
	pub scripts: Option<HashMap<String, String>>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EnvConfig {
	pub version: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RepositoryConfig {
	pub name: String,
	pub version: String,
	pub build: Option<String>
}
