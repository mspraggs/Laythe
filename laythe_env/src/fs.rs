use crate::SlIoError;
use std::{
  io,
  path::{Path, PathBuf},
};

pub struct FsWrapper {
  fs: Box<dyn Fsio>,
}

impl FsWrapper {
  pub fn new(fs: Box<dyn Fsio>) -> Self {
    Self { fs }
  }

  fn read_file(&self, path: &Path) -> Result<String, SlIoError> {
    self.fs.read_file(path)
  }

  fn read_directory(&self, path: &Path) -> io::Result<SlDirEntry> {
    self.fs.read_directory(path)
  }

  fn canonicalize(&self, path: &Path) -> Result<PathBuf, SlIoError> {
    self.fs.canonicalize(path)
  }

  fn relative_path(&self, base: &PathBuf, import: &Path) -> Result<PathBuf, SlIoError> {
    self.fs.relative_path(base, import)
  }
}

pub struct SlDirEntry();

pub trait Fsio {
  fn read_file(&self, path: &Path) -> Result<String, SlIoError>;
  fn read_directory(&self, path: &Path) -> io::Result<SlDirEntry>;
  fn canonicalize(&self, path: &Path) -> Result<PathBuf, SlIoError>;
  fn relative_path(&self, base: &PathBuf, import: &Path) -> Result<PathBuf, SlIoError>;
}

pub struct MockFsio();

impl Fsio for MockFsio {
  fn read_file(&self, path: &Path) -> Result<String, SlIoError> {
    Ok("let x = 10;".to_string())
  }
  fn read_directory(&self, path: &Path) -> io::Result<SlDirEntry> {
    Ok(SlDirEntry())
  }
  fn canonicalize(&self, path: &Path) -> Result<PathBuf, SlIoError> {
    Ok(path.to_path_buf())
  }
  fn relative_path(&self, base: &PathBuf, import: &Path) -> Result<PathBuf, SlIoError> {
    Ok(import.to_path_buf())
  }
}


