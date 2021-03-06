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
use laythe_env::{managed::Trace, stdio::StdIo};

pub const NATIVE_FUN_CLASS_NAME: &'static str = "Native Fun";

const NATIVE_FUN_NAME: NativeMeta = NativeMeta::new("name", Arity::Fixed(0), &[]);
const NATIVE_FUN_CALL: NativeMeta = NativeMeta::new(
  "call",
  Arity::Variadic(0),
  &[Parameter::new("args", ParameterKind::Any)],
);

pub fn declare_native_fun_class(
  hooks: &GcHooks,
  module: &mut Module,
  package: &Package,
) -> LyResult<()> {
  let class = default_class_inheritance(hooks, package, NATIVE_FUN_CLASS_NAME)?;
  export_and_insert(hooks, module, class.name, Value::from(class))
}

pub fn define_native_fun_class(hooks: &GcHooks, module: &Module, _: &Package) -> LyResult<()> {
  let mut class = load_class_from_module(hooks, module, NATIVE_FUN_CLASS_NAME)?;

  class.add_method(
    hooks,
    hooks.manage_str(String::from(NATIVE_FUN_NAME.name)),
    Value::from(to_dyn_method(hooks, NativeFunName())),
  );

  class.add_method(
    hooks,
    hooks.manage_str(String::from(NATIVE_FUN_CALL.name)),
    Value::from(to_dyn_method(hooks, NativeFunCall())),
  );

  Ok(())
}

#[derive(Clone, Debug, Trace)]
struct NativeFunName();

impl NativeMethod for NativeFunName {
  fn meta(&self) -> &NativeMeta {
    &NATIVE_FUN_NAME
  }

  fn call(&self, hooks: &mut Hooks, this: Value, _args: &[Value]) -> CallResult {
    Ok(Value::from(
      hooks.manage_str(String::from(this.to_native_fun().meta().name)),
    ))
  }
}

#[derive(Clone, Debug, Trace)]
struct NativeFunCall();

impl NativeMethod for NativeFunCall {
  fn meta(&self) -> &NativeMeta {
    &NATIVE_FUN_CALL
  }

  fn call(&self, hooks: &mut Hooks, this: Value, args: &[Value]) -> CallResult {
    hooks.call(this, args)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::{
    global::support::TestNative,
    support::{test_native_dependencies, MockedContext},
  };
  use laythe_env::managed::Managed;

  mod name {
    use super::*;
    use laythe_core::native::NativeFun;

    #[test]
    fn new() {
      let native_fun_name = NativeFunName();

      assert_eq!(native_fun_name.meta().name, "name");
      assert_eq!(native_fun_name.meta().signature.arity, Arity::Fixed(0));
    }

    #[test]
    fn call() {
      let native_fun_name = NativeFunName();
      let gc = test_native_dependencies();
      let mut context = MockedContext::new(&gc, &[]);
      let mut hooks = Hooks::new(&mut context);

      let managed: Managed<Box<dyn NativeFun>> = hooks.manage(Box::new(TestNative()));
      let result = native_fun_name.call(&mut hooks, Value::from(managed), &[]);
      match result {
        Ok(r) => assert_eq!(*r.to_str(), "test".to_string()),
        Err(_) => assert!(false),
      }
    }
  }

  mod call {
    use super::*;
    use crate::{
      global::support::TestNative,
      support::{test_native_dependencies, MockedContext},
    };
    use laythe_core::{native::NativeFun, value::VALUE_NIL};

    #[test]
    fn new() {
      let native_fun_call = NativeFunCall();

      assert_eq!(native_fun_call.meta().name, "call");
      assert_eq!(native_fun_call.meta().signature.arity, Arity::Variadic(0));
      assert_eq!(
        native_fun_call.meta().signature.parameters[0].kind,
        ParameterKind::Any
      );
    }

    #[test]
    fn call() {
      let native_fun_call = NativeFunCall();
      let gc = test_native_dependencies();
      let mut context = MockedContext::new(&gc, &[VALUE_NIL]);
      let mut hooks = Hooks::new(&mut context);

      let managed: Managed<Box<dyn NativeFun>> = hooks.manage(Box::new(TestNative()));
      let result = native_fun_call.call(&mut hooks, Value::from(managed), &[]);
      match result {
        Ok(r) => assert!(r.is_nil()),
        Err(_) => assert!(false),
      }
    }
  }
}
