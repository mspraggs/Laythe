use std::fmt;
use std::mem::{discriminant};
use crate::object::{Obj};

/// Enum of value types in spacelox
#[derive(Debug, Clone)]
pub enum Value {
  Bool(bool),
  Nil,
  Number(f64),
  Obj(Obj)
}

impl fmt::Display for Value {
  /// Implement display for value in spacelox
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Number(num) => write!(f, "{}", num),
      Value::Bool(b) => write!(f, "{}", b),
      Value::Nil => write!(f, "nil"),
      Value::Obj(obj) => write!(f, "{}", obj)
    }
  }
}

impl PartialEq for Value {
  /// Determine if this `Value` and another `Value` are equal inside
  /// of the spacelox runtime
  /// 
  /// # Examples
  /// ```
  /// use lox_runtime::value::Value;
  /// 
  /// let val1 = Value::Bool(false);
  /// let val2 = Value::Bool(true);
  /// 
  /// assert_eq!(val1 == val2, false);
  /// ```
  fn eq(&self, other: &Value) -> bool {
    // check we're the same variant
    if discriminant(self) != discriminant(other) {
      return false
    }

    // check the the variants have the same value
    match self {
      Value::Number(num1) => match other {
        Value::Number(num2) => num1 == num2,
        _ => unreachable!()
      },
      Value::Bool(b1) => match other {
        Value::Bool(b2) => b1 == b2,
        _ => unreachable!()
      },
      Value::Nil => true,
      Value::Obj(obj1) => match other {
        Value::Obj(obj2) => obj1 == obj2,
        _ => unreachable!()
      }
    }
  }
}

impl Value {
  /// Convert spacelox value to number, panics if not a number
  /// 
  /// # Examples
  /// ```
  /// use lox_runtime::value::Value;
  /// 
  /// let val1 = Value::Number(20.0);
  /// assert_eq!(val1.to_num(), 20.0);
  /// ```
  pub fn to_num(&self) -> f64 {
    match self {
      Value::Number(num) => *num,
      _ => panic!("Value is not number")
    }
  }

  /// Convert spacelox value to boolean, panics if not a bool
  /// 
  /// # Examples
  /// ```
  /// use lox_runtime::value::Value;
  /// 
  /// let b1 = Value::Bool(false);
  /// assert_eq!(b1.to_bool(), false);
  /// ```
  pub fn to_bool(&self) -> bool {
    match self {
      Value::Bool(b1) => *b1,
      _ => panic!("Value is not boolean")
    }
  }

  /// Convert spacelox value to an object, panics if not a object
  /// 
  /// # Examples
  /// ```
  /// use lox_runtime::value::Value;
  /// use lox_runtime::object::{Obj, ObjValue};
  /// 
  /// let str1 = Value::Obj(Obj::new(ObjValue::String("example".to_string())));
  /// assert_eq!(str1.move_obj().move_string(), "example");
  /// ```
  pub fn move_obj(self) -> Obj {
    match self {
      Value::Obj(obj) => obj,
      _ => panic!("Value is not string")
    }
  }
}

// Represents a collection of values
#[derive(Debug, Clone, Default)]
pub struct ValueVec {
  pub values: Vec<Value>
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::object::{ObjValue};


  fn example_each() -> Vec<Value> {
    vec![
      Value::Bool(true),
      Value::Nil,
      Value::Number(10.0),
      Value::Obj(Obj::new(ObjValue::String("example".to_string())))
    ]
  }

  #[test]
  fn test_diff_type_no_equal() {
    let examples = example_each();
    for i in 0..examples.len() {
      for j in 0..examples.len() {
        if i == j {
          assert_eq!(examples[i] == examples[j], true);
        } else {
          assert_eq!(examples[i] == examples[j], false);
        }
      }
    }
  }
}