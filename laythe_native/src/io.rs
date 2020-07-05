use laythe_env::{
  io::Io,
  stdio::StdioWrapper,
  fs::FsWrapper,
  env::EnvWrapper,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct NativeIo();

impl Io for NativeIo {
  fn stdio(&self) -> StdioWrapper {
    StdioWrapper()
  }

  fn fsio(&self) -> FsWrapper {
    NativeFsIo()
  }

  fn envio(&self) -> EnvWrapper {
    NativeEnvIo()
  }
}