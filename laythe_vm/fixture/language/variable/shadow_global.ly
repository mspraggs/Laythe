let a = "global";
{
  let a = "shadow";
  assertEq(a, "shadow"); // expect: shadow
}
assertEq(a, "global"); // expect: global
