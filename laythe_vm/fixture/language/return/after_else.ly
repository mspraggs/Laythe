fn f() {
  if (false) "no"; else return "ok";
}

assertEq(f(), "ok"); // expect: ok
