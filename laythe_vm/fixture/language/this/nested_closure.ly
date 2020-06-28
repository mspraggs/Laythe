class Foo {
  getClosure() {
    fn f() {
      fn g() {
        fn h() {
          return self.toString();
        }
        return h;
      }
      return g;
    }
    return f;
  }

  toString() { return "Foo"; }
}

let closure = Foo().getClosure();
print closure()()(); // expect: Foo
