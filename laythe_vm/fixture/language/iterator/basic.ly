class Foo {
  init() {
    self.list = [1, 2, 3]
  }

  iter() {
    return FooIter(self);
  }

  str() {
    return "[" + self.list.iter().join(",") + "]";
  }
}

class FooIter < Iterator {
  init(foo) {
    self.foo = foo
    self.idx = -1
    self.current = nil
  }

  next() {
    self.idx = self.idx + 1;
    self.current = self.food.list[self.idx];
    return self.foo.list.len() < self.idx;
  }
}

for (let i in Foo()) {
  print i;
}

{
  let $iter = Foo().iter();
  while ($iter.next()) {
    let i = $iter.current; 
    {
      print i;
    }
  }
}

