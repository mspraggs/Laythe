use crate::{
  env::{MockEnvio, EnvWrapper},
  fs::{MockFsio, FsWrapper},
  stdio::StdioWrapper,
};
use std::fmt;

pub struct IoWrapper {
  io: Box<dyn Io>,
}

impl IoWrapper {
  pub fn new(io: Box<dyn Io>) -> Self {
    Self { io }
  }

  pub fn stdio(&self) -> StdioWrapper {
    self.io.stdio()
  }

  pub fn fsio(&self) -> FsWrapper {
    self.io.fsio()
  }

  pub fn envio(&self) -> EnvWrapper {
    self.io.envio()
  }
}

pub trait Io: fmt::Debug {
  fn stdio(&self) -> StdioWrapper;
  fn fsio(&self) -> FsWrapper;
  fn envio(&self) -> EnvWrapper;
}

#[derive(Debug)]
pub struct MockIo();

impl Io for MockIo {
  fn stdio(&self) -> StdioWrapper {
    todo!()
  }
  fn fsio(&self) -> FsWrapper {
    FsWrapper::new(Box::new(MockFsio()))
  }
  fn envio(&self) -> EnvWrapper {
    EnvWrapper::new(Box::new(MockEnvio()))
  }
}

