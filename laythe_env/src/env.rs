use std::{io, path::PathBuf};

pub struct EnvWrapper {
  env: Box<dyn Envio>,
}

impl EnvWrapper {
  pub fn new(env: Box<dyn Envio>) -> Self {
    Self { env }
  }

  fn current_dir(&self) -> io::Result<PathBuf> {
    self.env.current_dir()
  }

  fn args(&self) -> Vec<String> {
    self.env.args()
  }
}

pub trait Envio {
  fn current_dir(&self) -> io::Result<PathBuf>;
  fn args(&self) -> Vec<String>;
}

pub struct MockEnvio();

impl Envio for MockEnvio {
  fn current_dir(&self) -> io::Result<PathBuf> {
    Ok(PathBuf::new())
  }

  fn args(&self) -> Vec<String> {
    vec![]
  }
}
