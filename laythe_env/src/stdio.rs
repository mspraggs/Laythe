use std::io;
use io::{Read, Write};

pub struct StdioWrapper {
  stdio: Box<dyn Stdio>,
}

impl Default for StdioWrapper {
  fn default() -> Self {
    Self { 
      stdio: Box::new(MockStdio::default())
    }
  }
}

impl StdioWrapper {
  pub fn new(stdio: Box<dyn Stdio>) -> Self {
    Self { stdio }
  }

  pub fn stdout(&mut self) -> &mut dyn Write {
    self.stdio.stdout()
  }

  pub fn stderr(&mut self) -> &mut dyn Write {
    self.stdio.stderr()
  }

  pub fn stdin(&mut self) -> &mut dyn Read {
    self.stdio.stdin()
  }

  pub fn read_line(&self, buffer: &mut String) -> io::Result<usize> {
    self.stdio.read_line(buffer)
  }
}

pub trait Stdio {
  fn stdout(&mut self) -> &mut dyn Write;
  fn stderr(&mut self) -> &mut dyn Write;
  fn stdin(&mut self) -> &mut dyn Read;

  fn read_line(&self, buffer: &mut String) -> io::Result<usize>;
}


pub struct MockStdio {
  write: MockWrite,
  read: MockRead
}

impl Default for MockStdio {
  fn default() -> Self {
    Self {
      write: MockWrite(),
      read: MockRead(),
    }
  }
}

impl Stdio for MockStdio {
  fn stdout(&mut self) -> &mut dyn Write {
    &mut self.write
  }
  fn stderr(&mut self) -> &mut dyn Write {
    &mut self.write
  }
  fn stdin(&mut self) -> &mut dyn Read {
    &mut self.read
  }
  fn read_line(&self, buffer: &mut String) -> io::Result<usize> {
    const LINE: &str = "let x = 10;";
    buffer.push_str(LINE);

    Ok(LINE.len())
  }
}

pub struct MockWrite();

impl Write for MockWrite {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    Ok(buf.len())
  }
  fn flush(&mut self) -> io::Result<()> {
    Ok(())
  }
}

pub struct MockRead();

impl Read for MockRead {
  fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
    Ok(0)
  }
}