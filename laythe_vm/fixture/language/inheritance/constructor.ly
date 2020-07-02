class A {
  init(param) {
    self.field = param;
  }

  test() {
    return self.field;
  }
}

class B < A {}

let b = B("value");
assertEq(b.test(), "value"); // expect: value
