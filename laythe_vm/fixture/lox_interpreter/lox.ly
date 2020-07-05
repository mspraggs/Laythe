// Lox interpreter written in ... Lox!


// Scanner: converts Lox source code input into tokens

// One-character tokens (values are the ASCII codes)
let LEFT_PAREN = 40;
let RIGHT_PAREN = 41;
let LEFT_BRACE = 123;
let RIGHT_BRACE = 125;
let COMMA = 44;
let DOT = 46;
let MINUS = 45;
let PLUS = 43;
let SEMICOLON = 59;
let SLASH = 47;
let STAR = 42;
let BANG = 33;
let EQUAL = 61;
let GREATER = 62;
let LESS = 60;

// Two-character tokens
let BANG_EQUAL = 256;
let EQUAL_EQUAL = 257;
let GREATER_EQUAL = 258;
let LESS_EQUAL = 259;

// Literals
let IDENTIFIER = 260;
let STRING = 261;
let NUMBER = 262;

// Keywords
let AND = 263;
let CLASS = 264;
let ELSE = 265;
let FALSE = 266;
let FUN = 267;
let FOR = 268;
let IF = 269;
let NIL = 270;
let OR = 271;
let PRINT = 272;
let RETURN = 273;
let SUPER = 274;
let THIS = 275;
let TRUE = 276;
let VAR = 277;
let WHILE = 278;

// Misc tokens
let EOF = 279;
let INVALID = 280;

class Token {
  init(type, value, line) {
    self.type = type;
    self.value = value;
    self.line = line;
  }
}

// Convert a token type constant back to the source string that the
// token represents.
fn tokenTypeStr(type) {
  if (type == LEFT_PAREN) return "(";
  if (type == RIGHT_PAREN) return ")";
  if (type == LEFT_BRACE) return "{";
  if (type == RIGHT_BRACE) return "}";
  if (type == COMMA) return ",";
  if (type == DOT) return ".";
  if (type == MINUS) return "-";
  if (type == PLUS) return "+";
  if (type == SEMICOLON) return ";";
  if (type == SLASH) return "/";
  if (type == STAR) return "*";
  if (type == BANG) return "!";
  if (type == EQUAL) return "=";
  if (type == GREATER) return ">";
  if (type == LESS) return "<";

  if (type == BANG_EQUAL) return "!=";
  if (type == EQUAL_EQUAL) return "==";
  if (type == GREATER_EQUAL) return ">=";
  if (type == LESS_EQUAL) return "<=";

  if (type == IDENTIFIER) return "<identifier>";
  if (type == STRING) return "<string>";
  if (type == NUMBER) return "<number>";

  if (type == AND) return "and";
  if (type == CLASS) return "class";
  if (type == ELSE) return "else";
  if (type == FALSE) return "false";
  if (type == FUN) return "fun";
  if (type == FOR) return "for";
  if (type == IF) return "if";
  if (type == NIL) return "nil";
  if (type == OR) return "or";
  if (type == PRINT) return "print";
  if (type == RETURN) return "return";
  if (type == SUPER) return "super";
  if (type == THIS) return "self";
  if (type == TRUE) return "true";
  if (type == VAR) return "var";
  if (type == WHILE) return "while";

  if (type == EOF) return "<eof>";
  if (type == INVALID) return "<invalid>";

  return nil;
}

// Convert a keyword string to its token type constant, or return nil
// if it's not a keyword.
fn keywordType(name) {
  if (name == "and") return AND;
  if (name == "class") return CLASS;
  if (name == "else") return ELSE;
  if (name == "false") return FALSE;
  if (name == "fun") return FUN;
  if (name == "for") return FOR;
  if (name == "if") return IF;
  if (name == "nil") return NIL;
  if (name == "or") return OR;
  if (name == "print") return PRINT;
  if (name == "return") return RETURN;
  if (name == "super") return SUPER;
  if (name == "self") return THIS;
  if (name == "true") return TRUE;
  if (name == "var") return VAR;
  if (name == "while") return WHILE;
  return nil;
}

fn isNameStart(ch) {
  // '_' or 'a'<=ch<='z' or 'A'<=ch<='Z'
  return ch == 95 or (ch >= 97 and ch <= 122) or (ch >= 65 and ch <= 90);
}

fn isDigit(ch) {
  // '0'<=ch<='9'
  return ch >= 48 and ch <= 57;
}

class Scanner {
  init() {
    self.line = 1;
    self.buffered = nil;
    self.ch = -1;
    self.advance();
  }

  advance() {
    let ch = self.ch;
    self.ch = getc();
    return ch;
  }

  next() {
    // Support a one-character buffer for "123." use case to mimic Lox
    if (self.buffered != nil) {
      let buffered = self.buffered;
      self.buffered = nil;
      return buffered;
    }

    while (self.ch >= 0) {
      let ch = self.advance();
      if (ch == LEFT_PAREN) return Token(LEFT_PAREN, nil, self.line);
      else if (ch == RIGHT_PAREN) return Token(RIGHT_PAREN, nil, self.line);
      else if (ch == LEFT_BRACE) return Token(LEFT_BRACE, nil, self.line);
      else if (ch == RIGHT_BRACE) return Token(RIGHT_BRACE, nil, self.line);
      else if (ch == COMMA) return Token(COMMA, nil, self.line);
      else if (ch == DOT) return Token(DOT, nil, self.line);
      else if (ch == MINUS) return Token(MINUS, nil, self.line);
      else if (ch == PLUS) return Token(PLUS, nil, self.line);
      else if (ch == SEMICOLON) return Token(SEMICOLON, nil, self.line);
      else if (ch == STAR) return Token(STAR, nil, self.line);
      else if (ch == BANG) {
        if (self.ch == EQUAL) {
          self.advance();
          return Token(BANG_EQUAL, nil, self.line);
        }
        return Token(BANG, nil, self.line);
      }
      else if (ch == EQUAL) {
        if (self.ch == EQUAL) {
          self.advance();
          return Token(EQUAL_EQUAL, nil, self.line);
        }
        return Token(EQUAL, nil, self.line);
      }
      else if (ch == LESS) {
        if (self.ch == EQUAL) {
          self.advance();
          return Token(LESS_EQUAL, nil, self.line);
        }
        return Token(LESS, nil, self.line);
      }
      else if (ch == GREATER) {
        if (self.ch == EQUAL) {
          self.advance();
          return Token(GREATER_EQUAL, nil, self.line);
        }
        return Token(GREATER, nil, self.line);
      }
      else if (ch == SLASH) {
        if (self.ch != SLASH) return Token(SLASH, nil, self.line);
        // Comment till end of line
        self.advance();
        while (self.ch >= 0 and self.ch != 10) {
          self.advance();
        }
        if (self.ch == 10) {
          self.line = self.line + 1;
        }
        self.advance();
      }
      else if (ch == 32 or ch == 9 or ch == 13) {
        // Ignore whitespace: space, tab, and carriage return
      }
      else if (ch == 10) {
        // Newline
        self.line = self.line + 1;  
      }
      else if (ch == 34) {
        // Strings (start and end with double quote)
        let line = self.line;
        let value = "";
        while (self.ch >= 0 and self.ch != 34) {
          if (self.ch == 10) {
            self.line = self.line + 1;
          }
          value = value + chr(self.ch);
          self.advance();
        }
        if (self.ch < 0) {
          return Token(INVALID, "Unterminated string.", self.line);
        }
        self.advance();
        return Token(STRING, value, line);
      }
      else if (isDigit(ch)) {
        // Numbers ('0' == 48)
        let num = ch - 48;
        while (isDigit(self.ch)) {
          num = num*10 + self.ch - 48;
          self.advance();
        }
        // Fractional part
        if (self.ch == DOT) {
          self.advance();
          if (!isDigit(self.ch)) {
            self.buffered = Token(DOT, nil, self.line);
            return Token(NUMBER, num, self.line);
          }
          let numerator = 0;
          let denominator = 1;
          while (isDigit(self.ch)) {
            numerator = numerator*10 + self.ch - 48;
            denominator = denominator*10;
            self.advance();
          }
          num = num + numerator/denominator;
        }
        return Token(NUMBER, num, self.line);
      }
      else if (isNameStart(ch)) {
        // Identifiers and keywords
        let name = chr(ch);
        while (isNameStart(self.ch) or isDigit(self.ch)) {
          name = name + chr(self.ch);
          self.advance();
        }
        let keyword = keywordType(name);
        if (keyword != nil) {
          return Token(keyword, nil, self.line);
        }
        return Token(IDENTIFIER, name, self.line);
      }
      else {
        return Token(INVALID, "Unexpected character.", self.line);
      }
    }
    return Token(EOF, nil, self.line);
  }
}

// Test scanner by reading input and displaying tokens
fn testScanner() {
  let scanner = Scanner();
  let done = false;
  while (!done) {
    let token = scanner.next();
    if (token.type == EOF) {
      done = true;
    } else if (token.type == INVALID) {
      done = true;
      print token.value + " on line:";
      print token.line;
    } else {
      if (token.value != nil) {
        print tokenTypeStr(token.type);
        print token.value;
      } else {
        print tokenTypeStr(token.type);
      }
    }
  }
}

// testScanner();


// Environment: used in interpreter to store and resolve variables.
class Environment {
  init(enclosing) {
    self.enclosing = enclosing;
    self.values = {};
  }

  // Find named value in environment, also looking in enclosing
  // environments. Exit with runtime error if name not found.
  get(name) {
    let item = self.values.get(name);
    if (!item) {
      return item;
    }

    if (self.enclosing != nil) {
      return self.enclosing.get(name);
    }

    runtimeError("Undefined variable '" + name + "'.");
  }

  // Assign an already-defined name in environment to value, also
  // looking in enclosing environments. Exit with runtime error if
  // name not found.
  assign(name, value) {
    if (self.values.has(name)) {
      self.values.set(name, value);
      return;
    }

    if (self.enclosing != nil) {
      self.enclosing.assign(name, value);
      return;
    }
    runtimeError("Undefined variable '" + name + "'.");
  }

  // Define a name in this environment and set to value.
  define(name, value) {
    self.values.set(name, value);
  }

  ancestor(distance) {
    let environment = self;
    for (let i = 0; i < distance; i = i + 1) {
      environment = environment.enclosing;
    }
    return environment;
  }

  getAt(distance, name) {
    return self.ancestor(distance).values.get(name);
  }

  assignAt(distance, name, value) {
    self.ancestor(distance).values.set(name, value);
  }
}


// Convert a number to its decimal string representation. Used only
// for pretty-printing the AST (disabled by default) and for showing
// things like line numbers in error messages.
fn numberStr(num) {
  let s = "";
  if (num < 0) {
    s = "-";
    num = -num;
  }
  let mult = 1;
  while (mult*10 <= num) {
    mult = mult * 10;
  }
  while (mult >= 1) {
    let digit = 0;
    while (mult*(digit+1) <= num) {
      digit = digit + 1;
    }
    s = s + chr(digit + 48);
    num = num - mult*digit;
    mult = mult / 10;
  }
  if (num != 0) {
    // Fractional part: only handles a fixed number of decimal
    // places right now (3), but this is only used for pretty-printing
    // the syntax tree and error messages, so that's probably fine.
    s = s + ".";
    num = num + 0.0005;
    for (let places = 0; places < 3; places = places + 1) {
      num = num * 10;
      let digit = 0;
      while (digit+1 <= num) {
        digit = digit + 1;
      }
      s = s + chr(digit + 48);
      num = num - digit;
    }
  }
  return s;
}


// Syntax tree nodes. Note that this is a NVAST (Not Very Abstract
// Syntax Tree :-), because each node includes a method to resolve
// and evaluate/execute itself. Done for simplicity.

// Expressions

class Assign {
  init(name, value) {
    self.type = "Assign";
    self.name = name;
    self.value = value;
  }

  str() {
    return self.name + " = " + self.value.str();
  }

  resolve(resolver) {
    self.value.resolve(resolver);
    resolver.resolveVar(self, self.name);
  }

  evaluate(interpreter) {
    let value = self.value.evaluate(interpreter);
    let distance = interpreter.locals.get(self);
    if (distance != nil) {
      interpreter.environment.assignAt(distance, self.name, value);
    } else {
      interpreter.globals.assign(self.name, value);
    }
    return value;
  }
}

class Binary {
  init(left, operator, right) {
    self.type = "Binary";
    self.left = left;
    self.operator = operator;
    self.right = right;
  }

  resolve(resolver) {
    self.left.resolve(resolver);
    self.right.resolve(resolver);
  }

  str() {
    return "(" + self.left.str() + " " + tokenTypeStr(self.operator) +
           " " + self.right.str() + ")";
  }

  evaluate(interpreter) {
    let left = self.left.evaluate(interpreter);
    let right = self.right.evaluate(interpreter);
    if (self.operator == PLUS) return left + right;
    if (self.operator == MINUS) return left - right;
    if (self.operator == STAR) return left * right;
    if (self.operator == SLASH) return left / right;
    if (self.operator == BANG_EQUAL) return left != right;
    if (self.operator == EQUAL_EQUAL) return left == right;
    if (self.operator == GREATER) return left > right;
    if (self.operator == GREATER_EQUAL) return left >= right;
    if (self.operator == LESS) return left < right;
    if (self.operator == LESS_EQUAL) return left <= right;
    return nil; // Unreachable
  }
}

class Call {
  init(callee, arguments) {
    self.type = "Call";
    self.callee = callee;
    self.arguments = arguments;
  }

  str() {
    let addComma = false;
    return self.arguments
      .iter()
      .reduce(self.callee.str() + "(", |s, argument| {
        if (addComma) {
          return s + ", ";
        }
        addComma = true;
        return s + argument.str();
      }) + ")";
  }

  resolve(resolver) {
    self.callee.resolve(resolver);
    self.arguments.iter().each(|argument| {
      return argument.resolve(resolver);
    });
  }

  evaluate(interpreter) {
    let callee = self.callee.evaluate(interpreter);
    let argument = self.arguments
      .iter()
      .map(|argument| {
        return argument.evaluate(interpreter);
      })
      .into(List.collect);

    if (arguments.size() != callee.arity()) {
      runtimeError("Expected " + numberStr(callee.arity()) +
                   " arguments but got " + numberStr(arguments.length()) + ".");
    }
    return callee.call(interpreter, arguments);
  }
}

class Get {
  init(object, name) {
    self.type = "Get";
    self.object = object;
    self.name = name;
  }
  
  str() {
    return self.object.str() + "." + self.name;
  }

  resolve(resolver) {
    self.object.resolve(resolver);
  }

  evaluate(interpreter) {
    let object = self.object.evaluate(interpreter);
    return object.get(self.name);
  }
}

class Grouping {
  init(expr) {
    self.type = "Grouping";
    self.expr = expr;
  }

  str() {
    return "(" + self.expr.str() + ")";
  }

  resolve(resolver) {
    self.expr.resolve(resolver);
  }

  evaluate(interpreter) {
    return self.expr.evaluate(interpreter);
  }
}

class Literal {
  init(kind, value) {
    self.type = "Literal";
    self.kind = kind;
    self.value = value;
  }

  str() {
    if (self.kind == "boolean") {
      if (self.value) {
        return "true";
      } else {
        return "false";
      }
    }
    if (self.kind == "nil") {
      return "nil";
    }
    if (self.kind == "number") {
      return numberStr(self.value);
    }
    if (self.kind == "string") {
      return chr(34) + self.value + chr(34);
    }
    return "<unexpected kind>";
  }

  resolve(resolver) {
  }

  evaluate(interpreter) {
    return self.value;
  }
}

class Logical {
  init(left, operator, right) {
    self.type = "Logical";
    self.left = left;
    self.operator = operator;
    self.right = right;
  }

  str() {
    return "(" + self.left.str() + " " + tokenTypeStr(self.operator) +
           " " + self.right.str() + ")";
  }

  resolve(resolver) {
    self.left.resolve(resolver);
    self.right.resolve(resolver);
  }

  evaluate(interpreter) {
    let left = self.left.evaluate(interpreter);
    if (self.operator == AND) {
      return left and self.right.evaluate(interpreter);
    } else {
      return left or self.right.evaluate(interpreter);
    }
  }
}

class Set {
  init(object, name, value) {
    self.type = "Set";
    self.object = object;
    self.name = name;
    self.value = value;
  }

  str() {
    return self.object.str() + "." + self.name + " = " + self.value.str();
  }

  resolve(resolver) {
    self.value.resolve(resolver);
    self.object.resolve(resolver);
  }

  evaluate(interpreter) {
    let object = self.object.evaluate(interpreter);
    let value = self.value.evaluate(interpreter);
    object.set(self.name, value);
    return value;
  }
}

class Super {
  init(method) {
    self.type = "Super";
    self.method = method;
  }

  str() {
    return "super." + self.method;
  }

  resolve(resolver) {
    resolver.resolveVar(self, "super");
  }

  evaluate(interpreter) {
    let distance = interpreter.locals.get(self);
    let superclass = interpreter.environment.getAt(distance, "super");
    let object = interpreter.environment.getAt(distance - 1, "this");
    let method = superclass.findMethod(object, self.method);
    if (method == nil) {
      runtimeError("Undefined property '" + self.method + "'.");
    }
    return method;
  }
}

class This {
  init() {
    self.type = "This";
  }

  str() {
    return "this";
  }

  resolve(resolver) {
    resolver.resolveVar(self, "this");
  }

  evaluate(interpreter) {
    return interpreter.lookupVariable("this", self);
  }
}

class Unary {
  init(operator, right) {
    self.type = "Unary";
    self.operator = operator;
    self.right = right;
  }

  str() {
    return tokenTypeStr(self.operator) + self.right.str();
  }

  resolve(resolver) {
    self.right.resolve(resolver);
  }

  evaluate(interpreter) {
    let right = self.right.evaluate(interpreter);
    if (self.operator == BANG) return !right;
    return -right;
  }
}

class Variable {
  init(name) {
    self.type = "Variable";
    self.name = name;
  }

  str() {
    return self.name;
  }

  resolve(resolver) {
    if (resolver.scopes.length() != 0 and resolver.scopes.last().get(self.name) == false) {
      resolver.error("Error at '" + self.name + "': Cannot read local variable in its own initializer.");
    }
    resolver.resolveVar(self, self.name);
  }

  evaluate(interpreter) {
    return interpreter.lookupVariable(self.name, self);
  }
}

// Statements

class Program {
  init(statements) {
    self.type = "Program";
    self.statements = statements;
  }

  str() {
    return self.statements.iter().reduce("", |s, statment| {
      return s + statement.str();
    });
  }

  resolve(resolver) {
    for (let statement in self.statements) {
      statement.resolve(resolver);
    }
  }

  execute(interpreter) {
    for (let statement in self.statements) {
      statement.execute(interpreter);
    }
  }
}

let indent = 0;

class Block {
  init(statements) {
    self.type = "Block";
    self.statements = statements;
  }

  str() {
    indent = indent + 1;
    let s = "{" + chr(10);

    let indentStr = indent.times()
      .reduce("", |indentStr, _| indentStr + "  ");

    s = self.statements
      .iter()
      .reduce(s, |s, statement| indentStr + statement.str());

    indent = indent - 1;

    indentStr = indent.times()
      .reduce("", |indentStr, _| indentStr + "  ");

    return s + indentStr + "}" + chr(10);
  }

  resolve(resolver) {
    resolver.beginScope();
    for (let statement in self.statements) {
      statement.resolve(resolver);
    }
    resolver.endScope();
  }

  execute(interpreter) {
    return interpreter.executeBlock(self.statements, Environment(interpreter.environment));
  }
}

class Class {
  init(name, superclass, methods) {
    self.type = "Class";
    self.name = name;
    self.superclass = superclass;
    self.methods = methods;
  }

  str() {
    let s = "class " + self.name + " ";
    if (self.superclass != nil) {
      s = s + "< " + self.superclass.name + " ";
    }
    s = s + Block(self.methods).str(); // Reuse Block.str()
    return s;
  }

  resolve(resolver) {
    resolver.declare(self.name);
    if (self.superclass != nil) {
      self.superclass.resolve(resolver);
    }
    resolver.define(self.name);
    if (self.superclass != nil) {
      resolver.beginScope();
      resolver.scopes.last().set("super", true);
    }
    resolver.beginScope();
    resolver.scopes.last().set("this", true);

    fn resolveMethod(method) {
      resolver.resolveFunction(method);
    }
    self.methods.foreach(resolveMethod);

    resolver.endScope();
    if (self.superclass != nil) {
      resolver.endScope();
    }
  }

  execute(interpreter) {
    let superclass;
    if (self.superclass != nil) {
      superclass = self.superclass.evaluate(interpreter);
      // The following will cause a runtime error if the superclass
      // is not a class
      superclass.findMethod;
    }
    interpreter.environment.define(self.name, nil);
    if (self.superclass != nil) {
      interpreter.environment = Environment(interpreter.environment);
      interpreter.environment.define("super", superclass);
    }

    let methods = self.methods.iter().reduce({}, |methods, method| {
      let function = LoxFunction(method, interpreter.environment,
                                 method.name == "init");
      methods.set(method.name, function);
      return methods;
    });

    let klass = LoxClass(self.name, superclass, methods);
    if (self.superclass != nil) {
      interpreter.environment = interpreter.environment.enclosing;
    }
    interpreter.environment.assign(self.name, klass);
  }
}

class Expression {
  init(expression) {
    self.type = "Expression";
    self.expression = expression;
  }

  str() {
    return self.expression.str() + ";" + chr(10);
  }

  resolve(resolver) {
    self.expression.resolve(resolver);
  }

  execute(interpreter) {
    self.expression.evaluate(interpreter);
  }
}

class Function {
  init(name, params, body) {
    self.type = "Function";
    self.name = name;
    self.params = params;
    self.body = body;
  }

  str() {
    let s = "fun " + self.name + "(";
    let addComma = false;
    self.params.iter().reduce(s, |s, param| {
      if (addComma) {
        return s + ", ";
      }
      addComma = true;
      return s + param;
    });

    s = s + ") ";
    s = s + Block(self.body).str();
    return s;
  }

  resolve(resolver) {
    resolver.declare(self.name);
    resolver.define(self.name);
    resolver.resolveFunction(self);
  }

  execute(interpreter) {
    let function = LoxFunction(self, interpreter.environment, false);
    interpreter.environment.define(self.name, function);
  }
}

class If {
  init(condition, thenBranch, elseBranch) {
    self.type = "If";
    self.condition = condition;
    self.thenBranch = thenBranch;
    self.elseBranch = elseBranch;
  }

  str() {
    let s = "if (" + self.condition.str() + ") " + self.thenBranch.str();
    if (self.elseBranch != nil) {
      for (let i = 0; i < indent; i = i + 1) {
        s = s + "  ";
      }
      s = s + "else " + self.elseBranch.str();
    }
    return s;
  }

  resolve(resolver) {
    self.condition.resolve(resolver);
    self.thenBranch.resolve(resolver);
    if (self.elseBranch != nil) {
      self.elseBranch.resolve(resolver);
    }
  }

  execute(interpreter) {
    if (self.condition.evaluate(interpreter)) {
      return self.thenBranch.execute(interpreter);
    } else if (self.elseBranch != nil) {
      return self.elseBranch.execute(interpreter);
    }
  }
}

class Print {
  init(expression) {
    self.type = "Print";
    self.expression = expression;
  }

  str() {
    return "print " + self.expression.str() + ";" + chr(10);
  }

  resolve(resolver) {
    self.expression.resolve(resolver);
  }

  execute(interpreter) {
    print self.expression.evaluate(interpreter);
  }
}

class ReturnValue {
  init(value) {
    self.value = value;
  }
}

class Return {
  init(value) {
    self.type = "Return";
    self.value = value;
  }

  str() {
    return "return " + self.value.str() + ";" + chr(10);
  }

  resolve(resolver) {
    if (self.value != nil) {
      self.value.resolve(resolver);
    }
  }

  execute(interpreter) {
    let value;
    if (self.value != nil) {
      value = self.value.evaluate(interpreter);
    }
    return ReturnValue(value);
  }
}

class Var {
  init(name, initializer) {
    self.type = "Var";
    self.name = name;
    self.initializer = initializer;
  }

  str() {
    let s = "var " + self.name;
    if (self.initializer != nil) {
      s = s + " = " + self.initializer.str();
    }
    s = s + ";" + chr(10);
    return s;
  }

  resolve(resolver) {
    resolver.declare(self.name);
    if (self.initializer != nil) {
      self.initializer.resolve(resolver);
    }
    resolver.define(self.name);
  }

  execute(interpreter) {
    let value;
    if (self.initializer != nil) {
      value = self.initializer.evaluate(interpreter);
    }
    interpreter.environment.define(self.name, value);
  }
}

class While {
  init(condition, body) {
    self.type = "While";
    self.condition = condition;
    self.body = body;
  }

  str() {
    return "while (" + self.condition.str() + ") " + self.body.str();
  }

  resolve(resolver) {
    self.condition.resolve(resolver);
    self.body.resolve(resolver);
  }

  execute(interpreter) {
    while (self.condition.evaluate(interpreter)) {
      let ret = self.body.execute(interpreter);
      if (ret) {
        return ret;
      }
    }
  }
}


// Lox runtime objects

class LoxFunction {
  init(declaration, closure, isInitializer) {
    self.declaration = declaration;
    self.closure = closure;
    self.isInitializer = isInitializer;
  }

  bind(instance) {
    let environment = Environment(self.closure);
    environment.define("this", instance);
    return LoxFunction(self.declaration, environment, self.isInitializer);
  }

  arity() {
    return self.declaration.params.length();
  }

  call(interpreter, arguments) {
    let environment = Environment(self.closure);
    self.declaration.params
      .iter()
      .zip(arguments.iter())
      .each(|items| {
        environment.define(items[0], items[1]);
      });

    let ret = interpreter.executeBlock(self.declaration.body, environment);
    if (self.isInitializer) {
      return self.closure.get("this");
    }
    if (ret) {
      return ret.value;
    }
    return nil;
  }
}

class LoxInstance {
  init(klass) {
    self.klass = klass;
    self.fields = {};
  }

  get(name) {
    let field = self.fields.get(name);
    if (field != nil) {
      return field.value;
    }
    let method = self.klass.findMethod(self, name);
    if (method != nil) {
      return method;
    }
    runtimeError("Undefined property '" + name + "'.");
  }

  set(name, value) {
    self.fields.set(name, value);
  }
}

class LoxClass {
  init(name, superclass, methods) {
    self.name = name;
    self.superclass = superclass;
    self.methods = methods;
  }

  findMethod(instance, name) {
    let method = self.methods.find(name);
    if (method != nil) {
      return method.value.bind(instance);
    }
    if (self.superclass != nil) {
      return self.superclass.findMethod(instance, name);
    }
    return nil;
  }

  call(interpreter, arguments) {
    let instance = LoxInstance(self);
    let initializer = self.methods.find("init");
    if (initializer != nil) {
      initializer.value.bind(instance).call(interpreter, arguments);
    }
    return instance;
  }

  arity() {
    let initializer = self.methods.find("init");
    if (initializer != nil) {
      return initializer.value.arity();
    }
    return 0;
  }
}

class Builtin0 {
  init(f) {
    self.f = f;
  }

  arity() {
    return 0;
  }

  call(interpreter, arguments) {
    return self.f();
  }
}

class Builtin1 {
  init(f) {
    self.f = f;
  }

  arity() {
    return 1;
  }

  call(interpreter, arguments) {
    return self.f(arguments.get(0));
  }
}


// The parser

class Parser {
  init() {
    self.functionDepth = 0;
    self.classDepth = 0;
    self.token = nil;
    self.scanner = Scanner();
    self.next();
  }

  next() {
    self.previous = self.token;
    self.token = self.scanner.next();
    if (self.token.type == INVALID) {
      self.error(self.token, self.token.value);
    }
  }

  error(token, message) {
    let at;
    if (token.type == INVALID) {
      at = "Error: ";
    } else if (token.type == EOF) {
      at = "Error at end: ";
    } else if (token.type == IDENTIFIER) {
      at = "Error at '" + token.value + "': ";
    } else if (token.type == NUMBER) {
      at = "Error at '" + numberStr(token.value) + "': ";
    } else {
      at = "Error at '" + tokenTypeStr(token.type) + "': ";
    }
    print_error("[line " + numberStr(self.token.line) + "] " + at + message);
    exit(65);
  }

  match(type) {
    if (self.token.type == type) {
      let previous = self.token;
      self.next();
      return previous;
    }
    return nil;
  }

  consume(type, message) {
    let token = self.token;
    if (self.match(type)) {
      return token;
    }
    self.error(self.token, message);
  }

  parse() {
    let statements = [];
    while (self.token.type != EOF) {
      statements.push(self.declaration());
    }
    return Program(statements);
  }

  expression() {
    return self.assignment();
  }

  declaration() {
    if (self.match(CLASS)) return self.classDeclaration();
    if (self.match(FUN)) return self.function("function");
    if (self.match(VAR)) return self.varDeclaration();
    return self.statement();
  }

  classDeclaration() {
    let name = self.consume(IDENTIFIER, "Expect class name.");
    let superclass;
    if (self.match(LESS)) {
      let superName = self.consume(IDENTIFIER, "Expect superclass name.");
      superclass = Variable(superName.value);
    }

    self.classDepth = self.classDepth + 1;
    self.hasSuperClass = superclass != nil;
    self.consume(LEFT_BRACE, "Expect '{' before class body.");
    let methods = List();
    while (self.token.type != EOF and self.token.type != RIGHT_BRACE) {
      methods.append(self.function("method"));
    }
    self.consume(RIGHT_BRACE, "Expect '}' after class body.");
    self.classDepth = self.classDepth - 1;
    self.hasSuperClass = false;

    return Class(name.value, superclass, methods);
  }

  statement() {
    if (self.match(FOR)) return self.forStatement();
    if (self.match(IF)) return self.ifStatement();
    if (self.match(PRINT)) return self.printStatement();
    if (self.match(RETURN)) return self.returnStatement();
    if (self.match(WHILE)) return self.whileStatement();
    if (self.match(LEFT_BRACE)) return Block(self.block());
    return self.expressionStatement();
  }

  forStatement() {
    self.consume(LEFT_PAREN, "Expect '(' after 'for'.");

    let initializer;
    if (self.match(SEMICOLON)) {
      // No initializer
    } else if (self.match(VAR)) {
      initializer = self.varDeclaration();
    } else {
      initializer = self.expressionStatement();
    }

    let condition;
    if (self.token.type != SEMICOLON) {
      condition = self.expression();
    }
    self.consume(SEMICOLON, "Expect ';' after loop condition.");

    let increment;
    if (self.token.type != RIGHT_PAREN) {
      increment = Expression(self.expression());
    }
    self.consume(RIGHT_PAREN, "Expect ')' after for clauses.");

    let body = self.statement();

    // Desugar increment
    if (increment != nil) {
      let statements = [];
      statements.push(body);
      statements.push(increment);
      body = Block(statements);
    }

    // Desugar condition
    if (condition == nil) {
      condition = Literal("boolean", true);
    }
    body = While(condition, body);

    // Desugar initializer
    if (initializer != nil) {
      let statements = [];
      statements.push(initializer);
      statements.push(body);
      body = Block(statements);
    }

    return body;
  }

  ifStatement() {
    self.consume(LEFT_PAREN, "Expect '(' after 'if'.");
    let condition = self.expression();
    self.consume(RIGHT_PAREN, "Expect ')' after if condition.");
    let thenBranch = self.statement();
    let elseBranch;
    if (self.match(ELSE)) {
      elseBranch = self.statement();
    }
    return If(condition, thenBranch, elseBranch);
  }

  printStatement() {
    let value = self.expression();
    self.consume(SEMICOLON, "Expect ';' after value.");
    return Print(value);
  }

  returnStatement() {
    if (self.functionDepth <= 0) {
      self.error(self.previous, "Cannot return from top-level code.");
    }
    let value;
    if (self.token.type != SEMICOLON) {
      if (self.inInitializer) {
        self.error(self.previous, "Cannot return a value from an initializer.");
      }
      value = self.expression();
    }
    self.consume(SEMICOLON, "Expect ';' after return value.");
    return Return(value);
  }

  varDeclaration() {
    let name = self.consume(IDENTIFIER, "Expect letiable name.");
    let initializer;
    if (self.match(EQUAL)) {
      initializer = self.expression();
    }
    self.consume(SEMICOLON, "Expect ';' after variable declaration.");
    return Var(name.value, initializer);
  }

  whileStatement() {
    self.consume(LEFT_PAREN, "Expect '(' after 'while'.");
    let condition = self.expression();
    self.consume(RIGHT_PAREN, "Expect ')' after condition.");
    let body = self.statement();
    return While(condition, body);
  }

  expressionStatement() {
    let expr = self.expression();
    self.consume(SEMICOLON, "Expect ';' after expression.");
    return Expression(expr);
  }

  function(kind) {
    let name = self.consume(IDENTIFIER, "Expect " + kind + " name.");
    self.consume(LEFT_PAREN, "Expect '(' after " + kind + " name.");
    let parameters = List();
    let n = 0;
    while (self.token.type != RIGHT_PAREN) {
      if (n > 0) {
        self.consume(COMMA, "Expect ')' after parameters.");
      }
      if (n >= 8) {
        self.error(self.token, "Cannot have more than 8 parameters.");
      }
      let paramName = self.consume(IDENTIFIER, "Expect parameter name.");
      parameters.append(paramName.value);
      n = n + 1;
    }
    self.consume(RIGHT_PAREN, "Expect ')' after parameters.");
    self.consume(LEFT_BRACE, "Expect '{' before " + kind + " body.");

    self.functionDepth = self.functionDepth + 1;
    self.inInitializer = kind == "method" and name.value == "init";
    let body = self.block();
    self.functionDepth = self.functionDepth - 1;
    self.inInitializer = false;

    return Function(name.value, parameters, body);
  }

  block() {
    let statements = List();
    while (self.token.type != EOF and self.token.type != RIGHT_BRACE) {
      statements.append(self.declaration());
    }
    self.consume(RIGHT_BRACE, "Expect '}' after block.");
    return statements;
  }

  assignment() {
    let expr = self.or_();
    if (self.match(EQUAL)) {
      let equals = self.previous;
      let value = self.assignment();
      if (expr.type == "Variable") {
        return Assign(expr.name, value);
      }
      if (expr.type == "Get") {
        return Set(expr.object, expr.name, value);
      }
      self.error(equals, "Invalid assignment target.");
    }
    return expr;
  }

  or_() {
    let expr = self.and_();
    while (self.match(OR)) {
      let right = self.and_();
      expr = Logical(expr, OR, right);
    }
    return expr;
  }

  and_() {
    let expr = self.equality();
    while (self.match(AND)) {
      let right = self.equality();
      expr = Logical(expr, AND, right);
    }
    return expr;
  }

  equality() {
    let expr = self.comparison();
    while (self.token.type == BANG_EQUAL or self.token.type == EQUAL_EQUAL) {
      let operator = self.token.type;
      self.next();
      let right = self.comparison();
      expr = Binary(expr, operator, right);
    }
    return expr;
  }

  comparison() {
    let expr = self.addition();
    while (self.token.type == GREATER or self.token.type == GREATER_EQUAL or
           self.token.type == LESS or self.token.type == LESS_EQUAL) {
      let operator = self.token.type;
      self.next();
      let right = self.addition();
      expr = Binary(expr, operator, right);
    }
    return expr;
  }

  addition() {
    let expr = self.multiplication();
    while (self.token.type == MINUS or self.token.type == PLUS) {
      let operator = self.token.type;
      self.next();
      let right = self.multiplication();
      expr = Binary(expr, operator, right);
    }
    return expr;
  }

  multiplication() {
    let expr = self.unary();
    while (self.token.type == SLASH or self.token.type == STAR) {
      let operator = self.token.type;
      self.next();
      let right = self.unary();
      expr = Binary(expr, operator, right);
    }
    return expr;
  }

  unary() {
    if (self.token.type == BANG or self.token.type == MINUS) {
      let operator = self.token.type;
      self.next();
      let right = self.unary();
      return Unary(operator, right);
    }
    return self.call();
  }

  call() {
    let expr = self.primary();
    let loop = true;
    while (loop) {
      if (self.match(LEFT_PAREN)) {
        let arguments = List();
        let n = 0;
        while (self.token.type != RIGHT_PAREN) {
          if (n > 0) {
            self.consume(COMMA, "Expect ')' after arguments.");
          }
          if (n >= 8) {
            self.error(self.token, "Cannot have more than 8 arguments.");
          }
          arguments.append(self.expression());
          n = n + 1;
        }
        self.consume(RIGHT_PAREN, "Expect ')' after arguments.");
        expr = Call(expr, arguments);
      } else if (self.match(DOT)) {
        let name = self.consume(IDENTIFIER, "Expect property name after '.'.");
        expr = Get(expr, name.value);
      } else {
        loop = false;
      }
    }
    return expr;
  }

  primary() {
    if (self.match(FALSE)) return Literal("boolean", false);
    if (self.match(TRUE)) return Literal("boolean", true);
    if (self.match(NIL)) return Literal("nil", nil);
    let number = self.match(NUMBER);
    if (number) {
      return Literal("number", number.value);
    }
    let string = self.match(STRING);
    if (string) {
      return Literal("string", string.value);
    }
    if (self.match(SUPER)) {
      if (self.classDepth <= 0) {
        self.error(self.previous, "Cannot use 'super' outside of a class.");
      }
      if (!self.hasSuperClass) {
        self.error(self.previous, "Cannot use 'super' in a class with no superclass.");
      }
      self.consume(DOT, "Expect '.' after 'super'.");
      let method = self.consume(IDENTIFIER, "Expect superclass method name.");
      return Super(method.value);
    }
    if (self.match(THIS)) {
      if (self.classDepth <= 0) {
        self.error(self.previous, "Cannot use 'this' outside of a class.");
      }
      return This();
    }
    let identifier = self.match(IDENTIFIER);
    if (identifier) {
      return Variable(identifier.value);
    }
    if (self.match(LEFT_PAREN)) {
      let expr = self.expression();
      self.consume(RIGHT_PAREN, "Expect ')' after expression.");
      return Grouping(expr);
    }
    self.error(self.token, "Expect expression.");
  }
}


// The variable resolver

class Resolver {
  init(program, interpreter) {
    self.program = program;
    self.interpreter = interpreter;
    self.scopes = []; // List can be used as a stack too
  }

  error(message) {
    print_error("[line 1] " + message); // This is hack for tests (it's not always line 1)
    exit(65);
  }

  resolve() {
    self.program.resolve(self);
  }

  resolveFunction(function) {
    self.beginScope();
    for (let param in function.params) {
      self.declare(param);
      self.define(param);
    }

    for (let statement in function.body) {
      statement.resolve(resolver);
    }
    self.endScope();
  }

  beginScope() {
    self.scopes.push({});
  }

  endScope() {
    self.scopes.pop();
  }

  declare(name) {
    if (self.scopes.size() == 0) {
      return;
    }
    let scope = self.scopes[this.scopes.size() - 1];
    if (scope.index(name)) {
      self.error("Error at '" + name + "': Variable with this name already declared in this scope.");
    }
    scope.set(name, false);
  }

  define(name) {
    if (self.scopes.size() == 0) {
      return;
    }
    self.scopes[self.scopes.size() - 1].set(name, true);
  }

  resolveVar(expr, name) {
    for (let i = self.scopes.size() - 1; i >= 0; i = i - 1) {
      if (self.scopes[i].has(name)) {
        self.interpreter.resolve(expr, self.scopes.size() - 1 - i);
        return;
      }
    }
    // Not found, assume it's a global
  }
}


// The tree-walking interpreter

class Interpreter {
  init(program) {
    self.program = program;
    self.globals = Environment(nil);
    self.environment = self.globals;
    self.locals = {};

    // Define built-in functions
    self.globals.define("clock", Builtin0(clock));
    self.globals.define("getc", Builtin0(getc));
    self.globals.define("chr", Builtin1(chr));
    self.globals.define("exit", Builtin1(exit));
    self.globals.define("print_error", Builtin1(print_error));
  }

  interpret() {
    self.program.execute(self);
  }

  executeBlock(statements, environment) {
    let previous = self.environment;
    self.environment = environment;
    let ret;

    for (let statement in statements) {
      // Stop early if there was "return" statement
      // TODO break statement
      if (!ret) {
        ret = statement.execute(self);
      }
    }

    self.environment = previous;
    return ret;
  }

  resolve(expr, depth) {
    self.locals.set(expr, depth);
  }

  lookupVariable(name, expr) {
    let distance = self.locals.get(expr);
    if (distance != nil) {
      return self.environment.getAt(distance, name);
    } else {
      return self.globals.get(name);
    }
  }
}

fn runtimeError(message) {
  print_error(message);
  print_error("[line 1]"); // This is hack for tests (it's not always line 1)
  exit(70);
}

let parser = Parser();
let program = parser.parse();

// Uncomment these two lines to pretty-print the parsed syntax tree
// print program.str();
// print "-----";

let interpreter = Interpreter(program);
let resolver = Resolver(program, interpreter);
resolver.resolve();
interpreter.interpret();