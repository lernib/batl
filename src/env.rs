use std::env::var as env_var;
use std::path::PathBuf;


pub struct System;

impl System {
	pub fn batl_root() -> Option<PathBuf> {
		// 1. Check BATL_ROOT environment variable
		if let Ok(batl_root) = env_var("BATL_ROOT") {
			return Some(PathBuf::from(batl_root));
		}

		// 2. Recursively descend from current directory until .batlrc is found
		if let Some(current_dir) = std::env::current_dir().ok() {
			let mut current_dir = current_dir;

			loop {
				if current_dir.join(".batlrc").exists() {
					return Some(current_dir);
				}

				if !current_dir.pop() {
					break;
				}
			}
		}

		// 3. Check for battalion folder in home directory
		if let Ok(home_dir) = env_var("HOME") {
			let batl_dir = PathBuf::from(home_dir).join("battalion");

			if batl_dir.exists() {
				return Some(batl_dir);
			}
		}

		None
	}
}
