use crate::support::{
  default_class_inheritance, export_and_insert, to_dyn_method, load_instance_from_module
};
use laythe_core::{
  hooks::{GcHooks, Hooks},
  module::Module,
  native::{NativeMeta, NativeMethod},
  package::Package,
  signature::{Arity, Parameter, ParameterKind},
  value::{VALUE_NIL, Value},
  CallResult, LyResult, object::Instance,
};
use laythe_env::{managed::Trace, stdio::StdioWrapper};

const STDOUT_CLASS_NAME: &str = "Stdout";
const STDOUT_INSTANCE_NAME: &str = "stdout";

const STDOUT_WRITE: NativeMeta = NativeMeta::new("write", Arity::Fixed(1), &[
  Parameter::new("string", ParameterKind::String)
]);

const STDOUT_WRITELN: NativeMeta = NativeMeta::new("writeln", Arity::Fixed(1), &[
  Parameter::new("string", ParameterKind::String)
]);

pub fn declare_stdout(
  hooks: &GcHooks,
  module: &mut Module,
  std: &Package,
) -> LyResult<()> {
  let class = default_class_inheritance(hooks, std, STDOUT_CLASS_NAME)?;
  let instance = hooks.manage(Instance::new(class));

  export_and_insert(
    hooks, module, 
    hooks.manage_str(STDOUT_INSTANCE_NAME.to_string()), 
    Value::from(instance)
  )
}

pub fn define_stdout(hooks: &GcHooks, module: &Module, _: &Package) -> LyResult<()> {
  let instance = load_instance_from_module(hooks, module, STDOUT_INSTANCE_NAME)?;
  let mut class = instance.class;

  class.add_method(
    hooks,
    hooks.manage_str(String::from(STDOUT_WRITE.name)),
    Value::from(to_dyn_method(hooks, StdoutWrite())),
  );

  class.add_method(
    hooks,
    hooks.manage_str(String::from(STDOUT_WRITELN.name)),
    Value::from(to_dyn_method(hooks, StdoutWriteln())),
  );

  Ok(())
}

#[derive(Clone, Debug, Trace)]
struct StdoutWrite();

impl NativeMethod for StdoutWrite {
  fn meta(&self) -> &NativeMeta {
    &STDOUT_WRITE
  }

  fn call(&self, hooks: &mut Hooks, _this: Value, args: &[Value]) -> CallResult {
    let io = hooks.to_io();
    let mut stdio = io.stdio();
    let stdout = stdio.stdout();

    match stdout.write(args[0].to_str().as_bytes()) {
      Ok(_) => Ok(VALUE_NIL),
      Err(err) => Err(hooks.make_error(err.to_string())),
    }
  }
}

#[derive(Clone, Debug, Trace)]
struct StdoutWriteln();

impl NativeMethod for StdoutWriteln {
  fn meta(&self) -> &NativeMeta {
    &STDOUT_WRITELN
  }

  fn call(&self, hooks: &mut Hooks, _this: Value, args: &[Value]) -> CallResult {
    let io = hooks.to_io();
    let mut stdio = io.stdio();
    let stdout = stdio.stdout();

    match writeln!(stdout, "{}", args[0].to_str()) {
      Ok(_) => Ok(VALUE_NIL),
      Err(err) => Err(hooks.make_error(err.to_string())),
    }
  }
}