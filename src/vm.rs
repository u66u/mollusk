use std::collections::HashMap;
use crate::types::Value;

#[derive(Debug, Clone)]
pub enum ArrayOperation {
    Push,
    Pop,
    Get(usize),
    Set(usize),
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Push(Value),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Greater,
    Less,
    Equal,
    NotEqual,
    Jmp(usize),
    Jz(usize),
    Label(String),
    Store(String),
    Load(String),
    BeginScope,
    EndScope,
    CreateArray,
    ArrayOp(ArrayOperation),
}

struct VM {
    stack: Vec<i32>,
    ip: usize,
    env_stack: Vec<HashMap<String, i32>>,
}