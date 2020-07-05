use std::io;
use io::{Read, Write};

pub struct StdioWrapper {
  stdio: Box<dyn Stdio>,
}

impl StdioWrapper {
  pub fn new(stdio: Box<dyn Stdio>) -> Self {
    Self { stdio }
  }

  pub fn stdout(&self) -> &dyn Write {
    self.stdio.stdout()
  }

  pub fn stderr(&self) -> &dyn Write {
    self.stdio.stderr()
  }

  pub fn stdin(&self) -> &dyn Read {
    self.stdio.stdin()
  }

  pub fn read_line(&self, buffer: &mut String) -> io::Result<usize> {
    self.stdio.read_line(buffer)
  }
}

pub trait Stdio {
  fn stdout(&self) -> &dyn Write;
  fn stderr(&self) -> &dyn Write;
  fn stdin(&self) -> &dyn Read;

  fn read_line(&self, buffer: &mut String) -> io::Result<usize>;
}


pub struct MockStdio();

impl Stdio for MockStdio {
  fn stdout(&self) -> &dyn Write {
      todo!()
  }
  fn stderr(&self) -> &dyn Write {
      todo!()
  }
  fn stdin(&self) -> &dyn Read {
      todo!()
  }
  fn read_line(&self, buffer: &mut String) -> io::Result<usize> {
      todo!()
  }
}

pub struct  MockWrite();