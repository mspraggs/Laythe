class Base {
  init(a) {
    self.a = a;
  }
}

class Derived < Base {
  init(a, b) {
    super.init(a);
    self.b = b;
  }
}

let derived = Derived("a", "b");
assertEq(derived.a, "a"); // expect: a
assertEq(derived.b, "b"); // expect: b
