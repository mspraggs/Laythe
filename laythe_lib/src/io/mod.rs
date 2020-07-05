mod stdio;

use laythe_core::{hooks::GcHooks, package::Package, LyResult};
use laythe_env::managed::Managed;
use stdio::add_stdio;

const IO_PACKAGE_NAME: &str = "io";

pub fn add_io(hooks: &GcHooks, mut std: Managed<Package>) -> LyResult<()> {
  let package = hooks.manage(Package::new(hooks.manage_str(IO_PACKAGE_NAME.to_string())));

  add_stdio(hooks, package)?;

  std.add_package(hooks, package)?;

  Ok(())
}
