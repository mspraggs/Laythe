let map = {
  "inner1": {
    "first": 1,
    "second": 2,
  },
  "inner2": {
    "third": 3,
    "fourth": 4,
  },
};

map["inner1"]["second"] = 20;
assertEq(map["inner1"]["first"], 1);
assertEq(map["inner1"]["second"], 20);

map["inner2"]["third"] = 30;
assertEq(map["inner2"]["third"], 30);
assertEq(map["inner2"]["fourth"], 4);