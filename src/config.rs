use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub name: String,
  pub languages: Vec<String>,
  pub links: HashMap<String, String>
}