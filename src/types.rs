#[derive(Debug, Clone, PartialEq)]
enum Value {
    Number(i32),
    Boolean(bool),
    Array(Vec<Value>),
    Null,
}

#[derive(Debug)]
enum TypeError {
    InvalidOperation {
        op: String,
        left: &'static str,
        right: &'static str,
    },
    InvalidIndex,
    InvalidType {
        expected: &'static str,
        got: &'static str,
    },
}
trait VMAdd {
    fn vm_add(&self, other: &Value) -> Result<Value, TypeError>;
}

trait VMCompare {
    fn vm_eq(&self, other: &Value) -> bool;
    fn vm_lt(&self, other: &Value) -> Result<bool, TypeError>;
}

impl VMAdd for Value {
    fn vm_add(&self, other: &Value) -> Result<Value, TypeError> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::Array(a), Value::Array(b)) => {
                let mut result = a.clone();
                result.extend(b.iter().cloned());
                Ok(Value::Array(result))
            }
            _ => Err(TypeError::InvalidOperation {
                op: "add".to_string(),
                left: self.type_name(),
                right: other.type_name(),
            }),
        }
    }
}

impl VMCompare for Value {
    fn vm_eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }

    fn vm_lt(&self, other: &Value) -> Result<bool, TypeError> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(a < b),
            _ => Err(TypeError::InvalidOperation {
                op: "less than".to_string(),
                left: self.type_name(),
                right: other.type_name(),
            }),
        }
    }
}

impl Value {
    fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::Boolean(_) => "boolean",
            Value::Array(_) => "array",
            Value::Null => "null",
        }
    }
}
