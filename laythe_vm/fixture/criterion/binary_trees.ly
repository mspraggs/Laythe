class Tree {
  init(item, depth) {
    self.item = item;
    self.depth = depth;
    if (depth > 0) {
      let item2 = item + item;
      depth = depth - 1;
      self.left = Tree(item2 - 1, depth);
      self.right = Tree(item2, depth);
    } else {
      self.left = nil;
      self.right = nil;
    }
  }

  check() {
    if (self.left == nil) {
      return self.item;
    }

    return self.item + self.left.check() - self.right.check();
  }
}

let minDepth = 3;
let maxDepth = 6;
let stretchDepth = maxDepth + 1;

Tree(0, stretchDepth).check();

let longLivedTree = Tree(0, maxDepth);

// iterations = 2 ** maxDepth
let iterations = 1;
let d = 0;
while (d < maxDepth) {
  iterations = iterations * 2;
  d = d + 1;
}

let depth = minDepth;
while (depth < stretchDepth) {
  let check = 0;
  let i = 1;
  while (i <= iterations) {
    check = check + Tree(i, depth).check() + Tree(-i, depth).check();
    i = i + 1;
  }

  iterations = iterations / 4;
  depth = depth + 2;
}