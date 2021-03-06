use laythe_core::{
  hooks::GcHooks,
  module::Module,
  native::{NativeFun, NativeMethod},
  object::Class,
  package::{Import, Package},
  value::Value,
  LyResult,
};
use laythe_env::managed::Managed;

pub fn to_dyn_method<T: 'static + NativeMethod>(
  hooks: &GcHooks,
  method: T,
) -> Managed<Box<dyn NativeMethod>> {
  hooks.manage(Box::new(method) as Box<dyn NativeMethod>)
}

pub fn to_dyn_fun<T: 'static + NativeFun>(
  hooks: &GcHooks,
  method: T,
) -> Managed<Box<dyn NativeFun>> {
  hooks.manage(Box::new(method) as Box<dyn NativeFun>)
}

pub fn create_meta_class(
  hooks: &GcHooks,
  name: Managed<String>,
  class_class: Managed<Class>,
) -> Managed<Class> {
  hooks.manage(Class::new(
    hooks,
    hooks.manage_str(format!("{} metaClass", name)),
    class_class,
    class_class,
  ))
}

pub fn default_class_inheritance(
  hooks: &GcHooks,
  package: &Package,
  class_name: &str,
) -> LyResult<Managed<Class>> {
  let name = hooks.manage_str(class_name.to_string());

  let import = Import::from_str(hooks, GLOBAL_PATH);
  let module = package.import(hooks, import)?;

  let object_class = load_class_from_module(hooks, &*module, "Object")?;
  let class_class = load_class_from_module(hooks, &*module, "Class")?;

  let meta = create_meta_class(hooks, name, class_class);

  Ok(hooks.manage(Class::new(hooks, name, meta, object_class)))
}

pub fn load_class_from_module(
  hooks: &GcHooks,
  module: &Module,
  name: &str,
) -> LyResult<Managed<Class>> {
  let name = hooks.manage_str(name.to_string());
  match module.import(hooks).get_field(&name) {
    Some(symbol) => {
      if symbol.is_class() {
        Ok(symbol.to_class())
      } else {
        Err(hooks.make_error(format!("Symbol {} is not a class.", name)))
      }
    }
    None => Err(hooks.make_error(format!(
      "Could not find symbol {} in module {}.",
      name,
      module.name()
    ))),
  }
}

pub fn export_and_insert(
  hooks: &GcHooks,
  module: &mut Module,
  name: Managed<String>,
  symbol: Value,
) -> LyResult<()> {
  module.insert_symbol(hooks, name, symbol);
  module.export_symbol(hooks, name)
}

#[cfg(test)]
pub use self::test::*;
use crate::GLOBAL_PATH;

#[cfg(test)]
mod test {
  use laythe_core::{
    hooks::{CallContext, GcContext, GcHooks, HookContext, Hooks},
    module::Module,
    object::Fun,
    signature::Arity,
    value::{Value, ValueKind},
    CallResult, LyError, iterator::SlIter,
  };
  use laythe_env::{
    managed::{Managed, Trace},
    memory::{Gc, NoGc, NO_GC},
    stdio::StdIo,
  };
  use std::path::PathBuf;

  pub struct MockedContext<'a> {
    gc: &'a Gc,
    no_gc: NoGc,
    responses: Vec<Value>,
    response_count: usize,
  }

  impl<'a> MockedContext<'a> {
    pub fn new(gc: &'a Gc, responses: &[Value]) -> Self {
      Self {
        gc,
        no_gc: NoGc(),
        responses: Vec::from(responses),
        response_count: 0,
      }
    }
  }

  impl<'a> HookContext for MockedContext<'a> {
    fn gc_context(&self) -> &dyn GcContext {
      self
    }

    fn call_context(&mut self) -> &mut dyn CallContext {
      self
    }
  }

  impl<'a> GcContext for MockedContext<'a> {
    fn gc(&self) -> &Gc {
      self.gc
    }
  }

  impl<'a> CallContext for MockedContext<'a> {
    fn call(&mut self, callable: Value, args: &[Value]) -> CallResult {
      let arity = match callable.kind() {
        ValueKind::Closure => callable.to_closure().fun.arity,
        ValueKind::Method => callable.to_method().method.to_closure().fun.arity,
        ValueKind::NativeFun => callable.to_native_fun().meta().signature.arity,
        ValueKind::NativeMethod => callable.to_native_method().meta().signature.arity,
        _ => {
          return Err(LyError::new(
            self.gc.manage_str("Not callable".to_string(), &NO_GC),
          ));
        }
      };

      match arity.check(args.len() as u8) {
        Ok(_) => (),
        Err(_) => {
          return Err(LyError::new(
            self
              .gc
              .manage_str("Incorrect function arity".to_string(), &NO_GC),
          ))
        }
      }

      if self.response_count < self.responses.len() {
        let response = self.responses[self.response_count];
        self.response_count += 1;
        return Ok(response);
      }

      Err(LyError::new(
        self.gc.manage_str("No mocked results".to_string(), &NO_GC),
      ))
    }

    fn call_method(&mut self, _this: Value, method: Value, args: &[Value]) -> CallResult {
      let arity = match method.kind() {
        ValueKind::Closure => method.to_closure().fun.arity,
        ValueKind::Method => method.to_method().method.to_closure().fun.arity,
        ValueKind::NativeFun => method.to_native_fun().meta().signature.arity,
        ValueKind::NativeMethod => method.to_native_method().meta().signature.arity,
        _ => {
          return Err(LyError::new(
            self.gc.manage_str("Not callable".to_string(), &NO_GC),
          ));
        }
      };

      match arity.check(args.len() as u8) {
        Ok(_) => (),
        Err(_) => {
          return Err(LyError::new(
            self
              .gc
              .manage_str("Incorrect function arity".to_string(), &NO_GC),
          ))
        }
      }

      if self.response_count < self.responses.len() {
        let response = self.responses[self.response_count];
        self.response_count += 1;
        return Ok(response);
      }

      Err(LyError::new(
        self.gc.manage_str("No mocked results".to_string(), &NO_GC),
      ))
    }

    fn call_method_by_name(
      &mut self,
      this: Value,
      method_name: Managed<String>,
      args: &[Value],
    ) -> CallResult {
      let arity = if this.is_instance() {
        let instance = this.to_instance();
        match instance.class.get_method(&method_name) {
          Some(method) => {
            if method.is_closure() {
              method.to_closure().fun.arity
            } else if method.is_native_method() {
              method.to_native_fun().meta().signature.arity
            } else {
              panic!("Only closures and native methods should be methods on an instance")
            }
          }
          None => {
            return Err(LyError::new(self.gc.manage_str(
              format!("No method {} exists on {:?}.", method_name, instance),
              &NO_GC,
            )));
          }
        }
      } else {
        Arity::Variadic(0)
      };

      match arity.check(args.len() as u8) {
        Ok(_) => (),
        Err(_) => {
          return Err(LyError::new(
            self
              .gc
              .manage_str("Incorrect method arity".to_string(), &NO_GC),
          ))
        }
      }

      if self.response_count < self.responses.len() {
        let response = self.responses[self.response_count];
        self.response_count += 1;
        return Ok(response);
      }

      Err(LyError::new(
        self.gc.manage_str("No mocked results".to_string(), &NO_GC),
      ))
    }
  }

  impl<'a> Trace for MockedContext<'a> {
    fn trace(&self) -> bool {
      self.no_gc.trace()
    }

    fn trace_debug(&self, stdio: &dyn StdIo) -> bool {
      self.no_gc.trace_debug(stdio)
    }
  }

  pub fn test_native_dependencies() -> Box<Gc> {
    Box::new(Gc::default())
  }


  #[derive(Trace, Debug)]
  pub struct TestIterator {
    current: usize,
  }

  impl TestIterator {
    fn new() -> Self {
      Self { current: 0 }
    }
  }

  impl SlIter for TestIterator {
    fn name(&self) -> &str {
      "Test Iterator"
    }

    fn current(&self) -> Value {
      Value::from(self.current as f64)
    }

    fn next(&mut self, _hooks: &mut Hooks) -> CallResult {
      if self.current > 4 {
        return Ok(Value::from(false));
      }
      
      self.current += 1;
      Ok(Value::from(true))
    }

    fn size_hint(&self) -> Option<usize> {
      Some(4)
    }

    fn size(&self) -> usize {
      8
    }
  }

  pub fn test_iter() -> Box<dyn SlIter> {
    Box::new(TestIterator::new())
  }

  pub fn fun_from_hooks(hooks: &GcHooks, name: String, module_name: &str) -> Managed<Fun> {
    let module = match Module::from_path(
      &hooks,
      hooks.manage(PathBuf::from(format!("path/{}.ly", module_name))),
    ) {
      Some(module) => module,
      None => unreachable!(),
    };

    let module = hooks.manage(module);
    hooks.manage(Fun::new(hooks.manage_str(name), module))
  }
}
