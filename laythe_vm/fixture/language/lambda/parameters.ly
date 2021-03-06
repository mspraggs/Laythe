let f0 = || 0;
print f0(); // expect: 0

let f1 = |a| a;
print f1(1); // expect: 1

let f2 = |a, b| a + b;
print f2(1, 2); // expect: 3

let f3 = |a, b, c| a + b + c;
print f3(1, 2, 3); // expect: 6

let f4 = |a, b, c, d| a + b + c + d;
print f4(1, 2, 3, 4); // expect: 10

let f5 = |a, b, c, d, e| a + b + c + d + e;
print f5(1, 2, 3, 4, 5); // expect: 15

let f6 = |a, b, c, d, e, f| a + b + c + d + e + f;
print f6(1, 2, 3, 4, 5, 6); // expect: 21

let f7 = |a, b, c, d, e, f, g| a + b + c + d + e + f + g;
print f7(1, 2, 3, 4, 5, 6, 7); // expect: 28

let f8 = |a, b, c, d, e, f, g, h| a + b + c + d + e + f + g + h;
print f8(1, 2, 3, 4, 5, 6, 7, 8); // expect: 36
