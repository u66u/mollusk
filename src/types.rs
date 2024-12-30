use crate::error::VMError;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i32),
    Boolean(bool),
    Array(Vec<Value>),
    Null,
}

trait VMAdd {
    fn vm_add(&self, other: &Value) -> Result<Value, VMError>;
}

trait VMCompare {
    fn vm_eq(&self, other: &Value) -> bool;
    fn vm_lt(&self, other: &Value) -> Result<bool, VMError>;
}

impl VMAdd for Value {
    fn vm_add(&self, other: &Value) -> Result<Value, VMError> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::Array(a), Value::Array(b)) => {
                let mut result = a.clone();
                result.extend(b.iter().cloned());
                Ok(Value::Array(result))
            }
            _ => Err(VMError::TypeError { message: ("Invalid addition".to_string()) }),
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

    fn vm_lt(&self, other: &Value) -> Result<bool, VMError> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(a < b),
            _ => Err(VMError::TypeError { message: ("Invalid lt".to_string()) }),
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
