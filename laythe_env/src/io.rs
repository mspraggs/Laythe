use crate::{
  env::{MockEnvio, EnvWrapper},
  fs::{MockFsio, FsWrapper},
  stdio::{MockStdio, StdioWrapper},
};
use std::fmt;

#[derive(Debug)]
pub struct IoWrapper {
  io: Box<dyn Io>,
}

impl Default for IoWrapper {
  fn default() -> Self {
    Self::new(Box::new(MockIo()))
  }
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

impl Clone for IoWrapper {
  fn clone(&self) -> Self {
    IoWrapper::new(self.io.clone())
  }
}

pub trait Io: fmt::Debug {
  fn clone(&self) -> Box<dyn Io>;

  fn stdio(&self) -> StdioWrapper;
  fn fsio(&self) -> FsWrapper;
  fn envio(&self) -> EnvWrapper;
}

#[derive(Debug)]
pub struct MockIo();

impl Io for MockIo {
  fn stdio(&self) -> StdioWrapper {
    StdioWrapper::new(Box::new(MockStdio::default()))
  }
  fn fsio(&self) -> FsWrapper {
    FsWrapper::new(Box::new(MockFsio()))
  }
  fn envio(&self) -> EnvWrapper {
    EnvWrapper::new(Box::new(MockEnvio()))
  }
  fn clone(&self) -> Box<dyn Io> {
    Box::new(MockIo())
  }
}

