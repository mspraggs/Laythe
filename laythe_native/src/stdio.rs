
#[derive(Debug, Clone)]
pub struct NativeStdIo();

impl Default for NativeStdIo {
  fn default() -> Self {
    Self()
  }
}

impl StdIo for NativeStdIo {
  fn print(&self, message: &str) {
    print!("{}", message);
  }
  fn println(&self, message: &str) {
    println!("{}", message);
  }
  fn eprint(&self, message: &str) {
    eprint!("{}", message);
  }
  fn eprintln(&self, message: &str) {
    eprintln!("{}", message);
  }
  fn flush(&self) -> Result<()> {
    stdout().flush()
  }
  fn read_line(&self, buffer: &mut String) -> Result<usize> {
    stdin().read_line(buffer)
  }
}
