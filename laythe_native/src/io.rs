use laythe_env::{
  io::Io,
  stdio::StdioWrapper,
  fs::FsWrapper,
  env::EnvWrapper,
};
use crate::{fs::NativeFsio, stdio::NativeStdio, env::NativeEnvio};

#[derive(Debug, Default)]
pub struct NativeIo();

impl Io for NativeIo {
  fn stdio(&self) -> StdioWrapper {
    StdioWrapper::new(Box::new(NativeStdio::default()))
  }

  fn fsio(&self) -> FsWrapper {
    FsWrapper::new(Box::new(NativeFsio()))
  }

  fn envio(&self) -> EnvWrapper {
    EnvWrapper::new(Box::new(NativeEnvio()))
  }

  fn clone(&self) -> Box<dyn Io> {
    Box::new(NativeIo())
  }
}