use laythe_env::{io::{IoWrapper, Io}, fs::FsWrapper, env::EnvWrapper, stdio::{StdioWrapper, Stdio}};
use laythe_vm::vm::{ExecuteResult, Vm};
use laythe_native::{
  fs::NativeFsio,
  env::NativeEnvio,
};
use std::fs::File;
use std::io::prelude::*;
use std::{
  cell::{Ref, RefCell, RefMut},
  path::{Path, PathBuf},
};
use std::str;
use std::{io::Result, rc::Rc};

pub fn fixture_path_inner(fixture_path: &str, test_file_path: &str) -> Option<PathBuf> {
  let test_path = Path::new(test_file_path);

  test_path
    .parent()
    .and_then(|path| path.parent())
    .and_then(|path| path.parent())
    .and_then(|path| Some(path.join("fixture").join(fixture_path)))
}

pub fn assert_files_exit(
  paths: &[&str],
  test_file_path: &str,
  io: IoWrapper,
  result: ExecuteResult,
) -> Result<()> {
  for path in paths {
    let mut vm = Vm::new(io.clone());

    let test_path = fixture_path_inner(path, test_file_path).expect("No parent directory");
    let debug_path = test_path.to_str().map(|s| s.to_string());

    let mut file = match File::open(test_path.clone()) {
      Ok(file) => file,
      Err(err) => {
        println!("Could not find {}", test_path.to_str().unwrap());
        return Err(err);
      }
    };
    let mut source = String::new();
    file.read_to_string(&mut source)?;

    assert_eq!(
      vm.run(test_path, &source),
      result,
      "Failing file {:?}",
      debug_path
    );
  }

  Ok(())
}

// this seems like a compiler bug? this is used in language.rs
#[allow(dead_code)]
pub fn assert_file_exit_and_stdio(
  path: &str,
  file_path: &str,
  stdout: Option<Vec<&str>>,
  stderr: Option<Vec<&str>>,
  result: ExecuteResult,
) -> Result<()> {
  let mut mock_console = MockedConsoleIo::new(MockedStdIo::default());
  let io = IoWrapper::new(mock_console.clone());

  assert_files_exit(&[path], file_path, io.clone(), result)?;

  mock_console.stdio.finish();
  if let Some(stdout) = stdout {
    mock_console
      .stdio
      .stdout
      .borrow()
      .iter()
      .zip(stdout.iter())
      .for_each(|(actual, expected)| {
        assert_eq!(actual, expected);
      });

    assert_eq!(
      mock_console
      .stdio
      .stdout.borrow().len(),
      stdout.len(),
      "Different standard out lines were collected than expected"
    );
  }

  if let Some(errout) = stderr {
    mock_console
      .stdio
      .stderr
      .borrow()
      .iter()
      .zip(errout.iter())
      .for_each(|(actual, expected)| {
        assert_eq!(actual, expected);
      });

    assert_eq!(
      mock_console
      .stdio
      .stderr.borrow().len(),
      errout.len(),
      "Different error out lines were collected than expected"
    );
  }

  Ok(())
}
#[derive(Debug, Default)]
pub struct MockedConsoleIo {
  pub stdio: MockedStdIo,
}

impl MockedConsoleIo {
  pub fn new(stdio: MockedStdIo) -> Self {
    Self { stdio }
  }
}

impl Io for MockedConsoleIo {
  fn stdio(&self) -> StdioWrapper {
    StdioWrapper::new(Box::new(self.stdio.clone()))
  }

  fn fsio(&self) -> FsWrapper {
    FsWrapper::new(Box::new(NativeFsio()))
  }

  fn envio(&self) -> EnvWrapper {
    EnvWrapper::new(Box::new(NativeEnvio()))
  }

  fn clone(&self) -> Box<dyn Io> {
    Box::new(MockedConsoleIo::new(self.stdio.clone()))
  }
}


#[derive(Clone, Debug)]
pub struct StdioCapture(Rc<RefCell<Vec<String>>>);

impl Default for StdioCapture {
  fn default() -> Self {
    Self(Rc::new(RefCell::new(vec![])))
  }
}

impl StdioCapture {
  fn borrow(&self) -> Ref<Vec<String>> {
    self.0.borrow()
  }

  fn borrow_mut(&self) -> RefMut<Vec<String>> {
    self.0.borrow_mut()
  }
}

#[derive(Clone, Debug)]
pub struct MockedStdIo {
  pub stdout: StdioCapture,
  pub stderr: StdioCapture,
  pub read_lines: Rc<RefCell<Vec<String>>>,
}

impl Default for MockedStdIo {
  fn default() -> Self {
    Self {
      stdout: StdioCapture::default(),
      stderr: StdioCapture::default(),
      read_lines: Rc::new(RefCell::new(vec![])),
    }
  }
}

impl MockedStdIo {
  pub fn finish(&mut self) {
    if let Some(last) = self.stdout.borrow().last() {
      if last == "" {
        self.stdout.borrow_mut().pop();
      }
    }

    if let Some(last) = self.stderr.borrow().last() {
      if last == "" {
        self.stderr.borrow_mut().pop();
      }
    }
  }
}

impl Stdio for MockedStdIo {
  fn stdout(&mut self) -> &mut dyn Write {
    &mut self.stdout
  }
  fn stderr(&mut self) -> &mut dyn Write {
    &mut self.stderr
  }
  fn stdin(&self) -> &dyn Read {
    todo!()
  }
  fn read_line(&self, _buffer: &mut String) -> Result<usize> {
    Ok(0)
  }
}

impl Write for StdioCapture {
  fn write(&mut self, buf: &[u8]) -> Result<usize> {
    let string = String::from(str::from_utf8(buf).unwrap());
    let mut segments = string.split('\n');

    match segments.next() {
      Some(first) => {
        let mut capture = self.borrow_mut();
        
        match capture.last_mut() {
          Some(last) => last.push_str(first),
          None => capture.push(first.to_string())
        }
      },
      None => panic!()
    }

    for remaining in segments {
      self.borrow_mut().push(remaining.to_string())
    }

    Ok(buf.len())
  }
  fn flush(&mut self) -> Result<()> {
    Ok(())
  }
}
