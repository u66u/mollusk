use crate::error::VMError;

#[derive(Debug, Clone)]
pub enum Value {
    Number(i32),
    Boolean(bool),
    Array(Vec<Value>),
    Null,
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

    fn as_array_mut(&mut self) -> Result<&mut Vec<Value>, VMError> {
        match self {
            Value::Array(arr) => Ok(arr),
            _ => Err(VMError::TypeError { message: format!("Expected array, got {}", self.type_name()) }),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Number(n) => *n > 0,
            Value::Boolean(b) => *b,
            Value::Array(arr) => !arr.is_empty(),
            Value::Null => false,
        }
    }
}

pub trait VMBinaryOp {
    fn add(&self, other: &Value) -> Result<Value, VMError>;
    fn sub(&self, other: &Value) -> Result<Value, VMError>;
    fn mul(&self, other: &Value) -> Result<Value, VMError>;
    fn div(&self, other: &Value) -> Result<Value, VMError>;
}

pub trait VMCompare {
    fn eq(&self, other: &Value) -> bool;
    fn lt(&self, other: &Value) -> Result<bool, VMError>;
    fn gt(&self, other: &Value) -> Result<bool, VMError>;
}

pub trait VMArray {
    fn push(&mut self, value: Value) -> Result<(), VMError>;
    fn pop(&mut self) -> Result<Value, VMError>;
    fn get(&self, index: Option<i32>) -> Result<Value, VMError>;
    fn set(&mut self, index: Option<i32>, value: Value) -> Result<(), VMError>;
}


impl VMBinaryOp for Value {
    fn add(&self, other: &Value) -> Result<Value, VMError> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            _ => Err(VMError::TypeError {
                message: format!("Cannot add {:?} and {:?}", self, other),
            }),
        }
    }

    fn sub(&self, other: &Value) -> Result<Value, VMError> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            _ => Err(VMError::TypeError {
                message: format!("Cannot subtract {:?} and {:?}", self, other),
            }),
        }
    }

    fn mul(&self, other: &Value) -> Result<Value, VMError> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            _ => Err(VMError::TypeError {
                message: format!("Cannot multiply {:?} and {:?}", self, other),
            }),
        }
    }

    fn div(&self, other: &Value) -> Result<Value, VMError> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                if *b == 0 {
                    Err(VMError::DivisionByZero)
                } else {
                    Ok(Value::Number(a / b))
                }
            }
            _ => Err(VMError::TypeError {
                message: format!("Cannot divide {:?} and {:?}", self, other),
            }),
        }
    }
}

impl VMCompare for Value {
    fn eq(&self, other: &Value) -> bool {
        let self_truthy = self.is_truthy();
        let other_truthy = other.is_truthy();
        self_truthy == other_truthy
    }

    fn lt(&self, other: &Value) -> Result<bool, VMError> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(a < b),
            _ => Err(VMError::TypeError {
                message: format!("Cannot compare {:?} and {:?} with <", self, other),
            }),
        }
    }

    fn gt(&self, other: &Value) -> Result<bool, VMError> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(a > b),
            _ => Err(VMError::TypeError {
                message: format!("Cannot compare {:?} and {:?} with >", self, other),
            }),
        }
    }
}


impl VMArray for Value {
    fn push(&mut self, value: Value) -> Result<(), VMError> {
        match self {
            Value::Array(arr) => {
                arr.push(value);
                Ok(())
            }
            _ => Err(VMError::NotAnArray),
        }
    }

    fn pop(&mut self) -> Result<Value, VMError> {
        match self {
            Value::Array(arr) => arr.pop().ok_or(VMError::IndexError {
                index: arr.len() as i32,
                len: arr.len(),
            }),
            _ => Err(VMError::NotAnArray),
        }
    }

    fn get(&self, index: Option<i32>) -> Result<Value, VMError> {
        match self {
            Value::Array(arr) => {
                let idx = index.ok_or(VMError::IndexError {
                    index: -1,
                    len: arr.len(),
                })?;
                if idx < 0 || idx >= arr.len() as i32 {
                    Err(VMError::IndexError {
                        index: idx,
                        len: arr.len(),
                    })
                } else {
                    Ok(arr[idx as usize].clone())
                }
            }
            _ => Err(VMError::NotAnArray),
        }
    }

    fn set(&mut self, index: Option<i32>, value: Value) -> Result<(), VMError> {
        match self {
            Value::Array(arr) => {
                let idx = index.ok_or(VMError::IndexError {
                    index: -1,
                    len: arr.len(),
                })?;
                if idx < 0 || idx >= arr.len() as i32 {
                    Err(VMError::IndexError {
                        index: idx,
                        len: arr.len(),
                    })
                } else {
                    arr[idx as usize] = value;
                    Ok(())
                }
            }
            _ => Err(VMError::NotAnArray),
        }
    }
}