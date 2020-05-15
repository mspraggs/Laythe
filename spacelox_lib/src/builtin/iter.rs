use spacelox_core::{
  arity::ArityKind,
  hooks::Hooks,
  io::StdIo,
  managed::{Managed, Trace},
  module::Module,
  native::{NativeMeta, NativeMethod},
  package::Package,
  value::{Class, Value},
  CallResult, ModuleResult, iterator::{SlIterator, SlIter}, utils::is_falsey,
};
use std::mem;

pub const ITER_CLASS_NAME: &'static str = "Iter";
const ITER_STR: NativeMeta = NativeMeta::new("str", ArityKind::Fixed(0));
const ITER_NEXT: NativeMeta = NativeMeta::new("next", ArityKind::Fixed(0));
const ITER_ITER: NativeMeta = NativeMeta::new("iter", ArityKind::Fixed(0));
const ITER_MAP: NativeMeta = NativeMeta::new("map", ArityKind::Fixed(1));
const ITER_FILTER: NativeMeta = NativeMeta::new("filter", ArityKind::Fixed(1));

pub fn declare_iter_class(hooks: &Hooks, self_module: &mut Module) -> ModuleResult<()> {
  let name = hooks.manage_str(String::from(ITER_CLASS_NAME));
  let class = hooks.manage(Class::new(name));

  self_module.add_export(hooks, name, Value::Class(class))
}

pub fn define_iter_class(hooks: &Hooks, self_module: &Module, _: &Package) {
  let name = hooks.manage_str(String::from(ITER_CLASS_NAME));
  let mut class = self_module.get_symbol(hooks, name).unwrap().to_class();

  class.add_method(
    hooks,
    hooks.manage_str(String::from(ITER_STR.name)),
    Value::NativeMethod(hooks.manage(Box::new(IterStr::new()))),
  );

  class.add_method(
    hooks,
    hooks.manage_str(String::from(ITER_NEXT.name)),
    Value::NativeMethod(hooks.manage(Box::new(IterNext::new()))),
  );

  class.add_method(
    hooks,
    hooks.manage_str(String::from(ITER_ITER.name)),
    Value::NativeMethod(hooks.manage(Box::new(IterIter::new()))),
  );

  let class_copy = class.clone();
  class.add_method(
    hooks,
    hooks.manage_str(String::from(ITER_MAP.name)),
    Value::NativeMethod(hooks.manage(Box::new(IterMap::new(class_copy)))),
  );

  class.add_method(
    hooks,
    hooks.manage_str(String::from(ITER_FILTER.name)),
    Value::NativeMethod(hooks.manage(Box::new(IterFilter::new(class_copy)))),
  );
}

#[derive(Clone, Debug, Trace)]
struct IterStr {
  meta: &'static NativeMeta,
}

impl IterStr {
  fn new() -> Self {
    Self { meta: &ITER_STR }
  }
}

impl NativeMethod for IterStr {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, hooks: &mut Hooks, this: Value, _args: &[Value]) -> CallResult {
    Ok(this.to_iter().name(hooks))
  }
}

#[derive(Clone, Debug, Trace)]
struct IterNext {
  meta: &'static NativeMeta,
}

impl IterNext {
  fn new() -> Self {
    Self { meta: &ITER_NEXT }
  }
}

impl NativeMethod for IterNext {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, hooks: &mut Hooks, this: Value, _args: &[Value]) -> CallResult {
    this.to_iter().next(hooks)
  }
}

#[derive(Clone, Debug, Trace)]
struct IterIter {
  meta: &'static NativeMeta,
}

impl IterIter {
  fn new() -> Self {
    Self { meta: &ITER_ITER }
  }
}

impl NativeMethod for IterIter {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, _hooks: &mut Hooks, this: Value, _args: &[Value]) -> CallResult {
    Ok(this)
  }
}

struct IterMap {
  meta: &'static NativeMeta,
  iter_class: Managed<Class>,
}

impl IterMap {
  fn new(iter_class: Managed<Class>) -> Self {
    Self {
      meta: &ITER_MAP,
      iter_class
    }
  }
}

impl NativeMethod for IterMap {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, hooks: &mut Hooks, this: Value, args: &[Value]) -> CallResult {
    let inner_iter: Box<dyn SlIter> = Box::new(MapIterator::new(this.to_iter(), args[0]));
    let iter = SlIterator::new(inner_iter, self.iter_class);
    let iter = hooks.manage(iter);

    Ok(Value::Iter(iter))
  }
}

impl Trace for IterMap {
  fn trace(&self) -> bool {
    self.iter_class.trace();
    true
  }

  fn trace_debug(&self, stdio: &dyn StdIo) -> bool {
    self.iter_class.trace_debug(stdio);
    true
  }
}

#[derive(Debug)]
struct MapIterator {
  current: Value,
  iter: Managed<SlIterator>,
  callable: Value
}

impl MapIterator {
  fn new(iter: Managed<SlIterator>, callable: Value) -> Self {
    Self {
      current: Value::Nil,
      iter,
      callable,
    }
  }
}

impl SlIter for MapIterator {
  fn name(&self) -> &str {
    "Map Iterator"
  }

  fn current(&self) -> Value {
    self.current
  }

  fn next(&mut self, hooks: &mut Hooks) -> CallResult {
    if is_falsey(self.iter.next(hooks)?) {
      Ok(Value::Bool(false))
    } else {
      let current = self.iter.current();
      self.current = hooks.call(self.callable, &[current])?;
      Ok(Value::Bool(true))
    }
  }

  fn size(&self) -> usize {
    mem::size_of::<Self>()
  }
}

impl Trace for MapIterator {
  fn trace(&self) -> bool {
    self.current.trace();
    self.iter.trace();
    self.callable.trace();
    true
  }

  fn trace_debug(&self, stdio: &dyn StdIo) -> bool {
    self.current.trace_debug(stdio);
    self.iter.trace_debug(stdio);
    self.callable.trace_debug(stdio);
    true
  }
}

struct IterFilter {
  meta: &'static NativeMeta,
  iter_class: Managed<Class>,
}

impl IterFilter {
  fn new(iter_class: Managed<Class>) -> Self {
    Self {
      meta: &ITER_MAP,
      iter_class
    }
  }
}

impl NativeMethod for IterFilter {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, hooks: &mut Hooks, this: Value, args: &[Value]) -> CallResult {
    let inner_iter: Box<dyn SlIter> = Box::new(FilterIterator::new(this.to_iter(), args[0]));
    let iter = SlIterator::new(inner_iter, self.iter_class);
    let iter = hooks.manage(iter);

    Ok(Value::Iter(iter))
  }
}

impl Trace for IterFilter {
  fn trace(&self) -> bool {
    self.iter_class.trace();
    true
  }

  fn trace_debug(&self, stdio: &dyn StdIo) -> bool {
    self.iter_class.trace_debug(stdio);
    true
  }
}

#[derive(Debug)]
struct FilterIterator {
  current: Value,
  iter: Managed<SlIterator>,
  callable: Value
}

impl FilterIterator {
  fn new(iter: Managed<SlIterator>, callable: Value) -> Self {
    Self {
      current: Value::Nil,
      iter,
      callable,
    }
  }
}

impl SlIter for FilterIterator {
  fn name(&self) -> &str {
    "Filter Iterator"
  }

  fn current(&self) -> Value {
    self.current
  }

  fn next(&mut self, hooks: &mut Hooks) -> CallResult {
    while !is_falsey(self.iter.next(hooks)?) {
      let current = self.iter.current();
      let should_keep = hooks.call(self.callable, &[current])?;

      if !is_falsey(should_keep) {
        self.current = current;
        return Ok(Value::Bool(true));
      }
    }

    Ok(Value::Bool(false))
  }

  fn size(&self) -> usize {
    mem::size_of::<Self>()
  }
}

impl Trace for FilterIterator {
  fn trace(&self) -> bool {
    self.current.trace();
    self.iter.trace();
    self.callable.trace();
    true
  }

  fn trace_debug(&self, stdio: &dyn StdIo) -> bool {
    self.current.trace_debug(stdio);
    self.iter.trace_debug(stdio);
    self.callable.trace_debug(stdio);
    true
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use spacelox_core::{iterator::SlIter, managed::Managed};

  #[derive(Trace, Debug)]
  struct TestIterator {
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
      Value::Number(self.current as f64)
    }

    fn next(&mut self, _hooks: &mut Hooks) -> CallResult {
      if self.current > 4 {
        return Ok(Value::Bool(false));
      }

      self.current += 1;
      Ok(Value::Bool(true))
    }

    fn size(&self) -> usize {
      8
    }
  }

  fn test_input(hooks: &Hooks) -> (Box<dyn SlIter>, Managed<Class>) {
    (
      Box::new(TestIterator::new()),
      hooks.manage(Class::new(hooks.manage_str(String::from("test")))),
    )
  }

  #[cfg(test)]
  mod str {
    use super::*;
    use crate::support::{test_native_dependencies, TestContext};
    use spacelox_core::iterator::SlIterator;

    #[test]
    fn new() {
      let iter_str = IterStr::new();

      assert_eq!(iter_str.meta.name, "str");
      assert_eq!(iter_str.meta.arity, ArityKind::Fixed(0));
    }

    #[test]
    fn call() {
      let iter_str = IterStr::new();
      let gc = test_native_dependencies();
      let mut context = TestContext::new(&gc, &[]);
      let mut hooks = Hooks::new(&mut context);

      let (iter, class) = test_input(&hooks);
      let this = hooks.manage(SlIterator::new(iter, class));

      let result = iter_str.call(&mut hooks, Value::Iter(this), &[]);
      match result {
        Ok(r) => assert_eq!(&*r.to_str(), "Test Iterator"),
        Err(_) => assert!(false),
      }
    }
  }

  #[cfg(test)]
  mod next {
    use super::*;
    use crate::support::{test_native_dependencies, TestContext};
    use spacelox_core::iterator::SlIterator;

    #[test]
    fn new() {
      let iter_next = IterNext::new();

      assert_eq!(iter_next.meta.name, "next");
      assert_eq!(iter_next.meta.arity, ArityKind::Fixed(0));
    }

    #[test] 
    fn call() {
      let iter_next = IterNext::new();
      let gc = test_native_dependencies();
      let mut context = TestContext::new(&gc, &[]);
      let mut hooks = Hooks::new(&mut context);

      let (iter, class) = test_input(&hooks);
      let this = hooks.manage(SlIterator::new(iter, class));

      let result = iter_next.call(&mut hooks, Value::Iter(this), &[]);
      match result {
        Ok(r) => assert_eq!(r.to_bool(), true),
        Err(_) => assert!(false),
      }
    }
  }
  #[cfg(test)]
  mod iter {
    use super::*;
    use crate::support::{test_native_dependencies, TestContext};
    use spacelox_core::iterator::SlIterator;

    #[test]
    fn new() {
      let iter_iter = IterIter::new();

      assert_eq!(iter_iter.meta.name, "iter");
      assert_eq!(iter_iter.meta.arity, ArityKind::Fixed(0));
    }

    #[test]
    fn call() {
      let iter_iter = IterIter::new();
      let gc = test_native_dependencies();
      let mut context = TestContext::new(&gc, &[]);
      let mut hooks = Hooks::new(&mut context);

      let (iter, class) = test_input(&hooks);
      let managed = hooks.manage(SlIterator::new(iter, class));
      let this = Value::Iter(managed);

      let result = iter_iter.call(&mut hooks, this, &[]);
      match result {
        Ok(r) => assert_eq!(r, this),
        Err(_) => assert!(false),
      }
    }
  }

  #[cfg(test)]
  mod map {
    use super::*;
    use crate::support::{test_native_dependencies, TestContext};
    use spacelox_core::{value::{Closure, Fun}, iterator::SlIterator};

    #[test]
    fn new() {
      let gc = test_native_dependencies();
      let mut context = TestContext::new(&gc, &[]);
      let hooks = Hooks::new(&mut context);

      let iter_map =
        IterMap::new(hooks.manage(Class::new(hooks.manage_str(String::from("something")))));

      assert_eq!(iter_map.meta.name, "map");
      assert_eq!(iter_map.meta.arity, ArityKind::Fixed(1));
    }

    #[test]
    fn call() {
      let gc = test_native_dependencies();
      let mut context = TestContext::new(&gc, &[Value::Number(5.0)]);
      let mut hooks = Hooks::new(&mut context);
      let iter_map =
        IterMap::new(hooks.manage(Class::new(hooks.manage_str(String::from("something")))));

      let (iter, class) = test_input(&hooks);
      let managed = hooks.manage(SlIterator::new(iter, class));
      let this = Value::Iter(managed);
      let fun = Value::Closure(hooks.manage(Closure::new(
        hooks.manage(Fun::new(hooks.manage_str(String::from("example"))))
      )));

      fun.to_closure().fun.arity = ArityKind::Fixed(1);
        
      let result = iter_map.call(&mut hooks, this, &[fun]);
      match result {
        Ok(r) => {
          let mut map_iter = r.to_iter();
          assert_eq!(map_iter.next(&mut hooks).unwrap(), Value::Bool(true));
          assert_eq!(map_iter.current(), Value::Number(5.0));
        }
        Err(_) => assert!(false),
      }
    }
  }

}
