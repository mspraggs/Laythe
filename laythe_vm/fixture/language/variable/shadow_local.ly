{
  let a = "local";
  {
    let a = "shadow";
    assertEq(a, "shadow"); // expect: shadow
  }
  assertEq(a, "local"); // expect: local
}
