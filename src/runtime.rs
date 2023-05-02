use rlua::{Lua, Result, StdLib};
use std::path::PathBuf;

pub struct Runtime {
  pub lua: Lua
}

impl Runtime {
  pub fn new() -> Result<Self> {
    let lua = Lua::new_with(StdLib::empty());
    let mut runtime = Runtime { lua };
    runtime.init()?;
    Ok(runtime)
  }

  fn init(&mut self) -> Result<()> {
    self.lua.context(|lua_ctx| {
      let globals = lua_ctx.globals();

      globals.set("shell", lua_ctx.create_function(|_, (cmd,): (String,)| {
        std::process::Command::new("bash")
          .arg("-c")
          .arg(cmd)
          .status()
          .expect("failed to execute process");

        Ok(())
      })?)?;

      Ok(())
    })
  }

  pub fn exec_file(&mut self, path: &PathBuf) -> Result<()> {
    let contents = std::fs::read_to_string(path).or_else(|_| {
      Err(rlua::Error::RuntimeError("Failed to read file".to_string()))
    })?;

    self.lua.context(|lua_ctx| {
      lua_ctx.load(&contents).exec()
    })
  }
}