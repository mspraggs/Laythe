mod stdin;
mod stdout;

use laythe_core::{hooks::GcHooks, module::Module, package::Package, LyResult};
use laythe_env::managed::Managed;
use std::path::PathBuf;

const STDIO_PATH: &str = "std/io/stdio.ly";

pub fn add_stdio(hooks: &GcHooks, io: Managed<Package>) -> LyResult<()> {
  let module = Module::from_path(&hooks, hooks.manage(PathBuf::from(STDIO_PATH)))?;

  Ok(())
}
