use crate::support::{
  default_class_inheritance, export_and_insert, load_class_from_module, to_dyn_method,
};
use laythe_core::{
  hooks::{GcHooks, Hooks},
  module::Module,
  native::{NativeMeta, NativeMethod},
  package::Package,
  signature::{Arity, Parameter, ParameterKind},
  value::Value,
  CallResult, LyResult,
};
use laythe_env::{managed::Trace, stdio::Stdio};

const STDOUT_WRITE: NativeMeta = NativeMeta::new("write", Arity::Fixed(1), &[
  Parameter::new("string", ParameterKind::String)
]);

pub fn define_stdout_funs(
  hooks: &GcHooks,
  module: &mut Module,
  package: &Package,
) -> LyResult<()> {
  let class = default_class_inheritance(hooks, package, CLOSURE_CLASS_NAME)?;
  export_and_insert(hooks, module, class.name, Value::from(class))
}

#[derive(Clone, Debug, Trace)]
struct StdoutWrite();

impl NativeMethod for StdoutWrite {
  fn meta(&self) -> &NativeMeta {
    &STDOUT_WRITE
  }

  fn call(&self, _hooks: &mut Hooks, this: Value, _args: &[Value]) -> CallResult {
    Ok(Value::from(this.to_closure().fun.name))
  }
}