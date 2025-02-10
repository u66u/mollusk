use crate::ast::ASTNode;
use crate::error::VMError;
use crate::tokenizer::Token;
use crate::types::{VMArray, VMBinaryOp, VMCompare, Value};
use std::collections::HashMap;

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

pub struct VM {
    pub stack: Vec<Value>,
    pub ip: usize,
    pub env_stack: Vec<HashMap<String, Value>>,
    max_stack_size: usize,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Vec::new(),
            ip: 0,
            env_stack: vec![HashMap::new()], // Start with global scope
            max_stack_size: 4000, 
        }
    }

    fn current_env(&mut self) -> &mut HashMap<String, Value> {
        self.env_stack.last_mut().expect("No environment on stack")
    }

    fn get_var(&self, name: &str) -> Option<Value> {
        for env in self.env_stack.iter().rev() {
            if let Some(value) = env.get(name) {
                return Some(value.clone());
            }
        }
        None
    }

    fn push(&mut self, value: Value) -> Result<(), VMError> {
        if self.stack.len() >= self.max_stack_size {
            return Err(VMError::StackOverflow);
        }
        self.stack.push(value);
        Ok(())
    }

    fn check_array_bounds(&self, idx: i32, len: usize) -> Result<usize, VMError> {
        if idx < 0 || idx as usize >= len {
            return Err(VMError::IndexError { index: idx, len });
        }
        Ok(idx as usize)
    }

    pub fn execute(&mut self, instructions: &[Instruction]) -> Result<(), VMError> {
        let mut scope_depth = 0;
        
        while self.ip < instructions.len() {
            match &instructions[self.ip] {
                Instruction::Push(value) => {
                    self.push(value.clone())?;
                }
                Instruction::Pop => {
                    self.stack.pop().ok_or(VMError::StackUnderflow)?;
                }
                Instruction::Add => {
                    let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    let result = a.add(&b)?;
                    self.stack.push(result);
                }
                Instruction::Sub => {
                    let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    let result = a.sub(&b)?;
                    self.stack.push(result);
                }
                Instruction::Mul => {
                    let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    let result = a.mul(&b)?;
                    self.stack.push(result);
                }
                Instruction::Div => {
                    let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    let result = a.div(&b)?;
                    self.stack.push(result);
                }
                Instruction::Greater => {
                    let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    let result = a.gt(&b)?;
                    self.stack.push(Value::Boolean(result));
                }
                Instruction::Less => {
                    let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    let result = a.lt(&b)?;
                    self.stack.push(Value::Boolean(result));
                }
                Instruction::Equal => {
                    let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    let result = a.eq(&b);
                    self.stack.push(Value::Boolean(result));
                }
                Instruction::NotEqual => {
                    let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    let result = !a.eq(&b);
                    self.stack.push(Value::Boolean(result));
                }
                Instruction::Jmp(target) => {
                    if *target >= instructions.len() {
                        return Err(VMError::ExecutionError {
                            message: format!("Jump target {} out of bounds", target),
                            line: 0,
                            position: 0,
                        });
                    }
                    self.ip = *target;
                    continue;
                }
                Instruction::Jz(target) => {
                    if *target >= instructions.len() {
                        return Err(VMError::InvalidJump { 
                            target: *target,
                            max: instructions.len() 
                        });
                    }
                    let condition = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    if !condition.is_truthy() {
                        self.ip = *target;
                        continue;
                    }
                }
                Instruction::Label(_) => {}
                Instruction::Store(name) => {
                    let value = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    self.current_env().insert(name.clone(), value);
                }
                Instruction::Load(name) => {
                    if let Some(value) = self.get_var(name) {
                        self.stack.push(value);
                    } else {
                        return Err(VMError::UndefinedVariable { name: name.clone() });
                    }
                }
                Instruction::BeginScope => {
                    scope_depth += 1;
                    self.env_stack.push(HashMap::new());
                }
                Instruction::EndScope => {
                    scope_depth -= 1;
                    if self.env_stack.pop().is_none() {
                        return Err(VMError::NoScopeToEnd);
                    }
                }
                Instruction::CreateArray => {
                    self.stack.push(Value::Array(Vec::new()));
                }
                Instruction::ArrayOp(op) => match op {
                    ArrayOperation::Push => {
                        let value = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                        let mut array = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                        array.push(value)?;
                        self.stack.push(array);
                    }
                    ArrayOperation::Pop => {
                        let mut array = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                        let value = array.pop()?;
                        self.stack.push(array);
                        self.stack.push(value);
                    }
                    ArrayOperation::Get(_) => {
                        let index = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                        let array = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                        
                        if let (Value::Number(idx), Value::Array(arr)) = (index, array) {
                            let bound_idx = self.check_array_bounds(idx, arr.len())?;
                            self.stack.push(arr[bound_idx].clone());
                        } else {
                            return Err(VMError::TypeError {
                                message: "Invalid array access".to_string(),
                            });
                        }
                    }
                    ArrayOperation::Set(_) => {
                        let value = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                        let index = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                        let array = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                        
                        if let (Value::Number(idx), Value::Array(mut arr)) = (index, array) {
                            let bound_idx = self.check_array_bounds(idx, arr.len())?;
                            arr[bound_idx] = value;
                            let array_value = Value::Array(arr);
                            if let Some(name) = self.current_env().iter().find_map(|(k, v)| 
                                if matches!(v, Value::Array(_)) { Some(k.clone()) } else { None }
                            ) {
                                self.current_env().insert(name, array_value);
                            }
                        } else {
                            return Err(VMError::TypeError {
                                message: "Invalid array assignment".to_string(),
                            });
                        }
                    }
                },
            }
            self.ip += 1;
        }
        
        if scope_depth != 0 {
            return Err(VMError::ExecutionError {
                message: format!("Unclosed scopes at end of execution: {}", scope_depth),
                line: 0, 
                position: 0,
            });
        }
        
        Ok(())
    }
}

pub fn compile(node: ASTNode) -> Vec<Instruction> {
    match node {
        ASTNode::Number(n) => vec![Instruction::Push(Value::Number(n))],
        ASTNode::String(s) => vec![Instruction::Push(Value::String(s))],
        ASTNode::BinOp { left, op, right } => {
            let mut instructions = compile(*left);
            instructions.extend(compile(*right));
            match op {
                Token::Plus => instructions.push(Instruction::Add),
                Token::Minus => instructions.push(Instruction::Sub),
                Token::Star => instructions.push(Instruction::Mul),
                Token::Slash => instructions.push(Instruction::Div),
                Token::Greater => instructions.push(Instruction::Greater),
                Token::Less => instructions.push(Instruction::Less),
                Token::Equal => instructions.push(Instruction::Equal),
                Token::NotEqual => instructions.push(Instruction::NotEqual),
                _ => panic!("Unsupported operation"),
            }
            instructions
        }
        ASTNode::If {
            condition,
            if_block,
            else_block,
        } => {
            let mut instructions = compile(*condition);
            let if_instructions: Vec<Instruction> =
                if_block.into_iter().flat_map(compile).collect();
            let else_instructions: Vec<Instruction> =
                else_block.into_iter().flat_map(compile).collect();

            let else_start = instructions.len() + if_instructions.len() + 2;
            instructions.push(Instruction::Jz(else_start));

            instructions.extend(if_instructions);

            let after_else = else_start + else_instructions.len();
            instructions.push(Instruction::Jmp(after_else));

            instructions.extend(else_instructions);
            instructions
        }
        ASTNode::While { condition, body } => {
            let mut instructions = Vec::new();
            // Record where condition check starts
            let condition_start = instructions.len();
            instructions.extend(compile(*condition));

            // Record where we'll put the Jz instruction
            let jz_placeholder_index = instructions.len();
            instructions.push(Instruction::Jz(0)); // Temporary placeholder

            let body_instructions: Vec<Instruction> = body.into_iter().flat_map(compile).collect();
            let body_len = body_instructions.len();
            instructions.extend(body_instructions);
            instructions.push(Instruction::Jmp(condition_start));

            let after_loop = jz_placeholder_index + 1 + body_len + 1;
            instructions[jz_placeholder_index] = Instruction::Jz(after_loop);

            instructions
        }
        ASTNode::VarDecl(name, value) => {
            let mut instructions = compile(*value);
            instructions.push(Instruction::Store(name));
            instructions
        }
        ASTNode::VarAssign(name, value) => {
            let mut instructions = compile(*value);
            instructions.push(Instruction::Store(name));
            instructions
        }
        ASTNode::VarRef(name) => vec![Instruction::Load(name)],
        ASTNode::Block(nodes) => {
            let mut instructions = vec![Instruction::BeginScope];
            instructions.extend(nodes.into_iter().flat_map(compile));
            instructions.push(Instruction::EndScope);
            instructions
        }
        ASTNode::Array(elements) => {
            let mut instructions = vec![Instruction::CreateArray];
            for element in elements {
                instructions.extend(compile(element));
                instructions.push(Instruction::ArrayOp(ArrayOperation::Push));
            }
            instructions
        }
        ASTNode::ArrayIndex { array, index } => {
            let mut instructions = compile(*array);
            instructions.extend(compile(*index));
            instructions.push(Instruction::ArrayOp(ArrayOperation::Get(0)));
            instructions
        }
        ASTNode::ArrayAssign { array, index, value } => {
            let mut instructions = compile(*array);
            instructions.extend(compile(*index));
            instructions.extend(compile(*value));
            instructions.push(Instruction::ArrayOp(ArrayOperation::Set(0)));
            instructions
        }
}
}

pub fn run_instructions(nodes: Vec<ASTNode>) -> Vec<Instruction> {
    let mut instr = Vec::new();
    let mut offset = 0;
    for node in nodes {
        let mut node_instructions = compile(node);
        for instruction in &mut node_instructions {
            match instruction {
                Instruction::Jmp(target) => *target += offset,
                Instruction::Jz(target) => *target += offset,
                _ => {}
            }
        }
        offset += node_instructions.len();
        instr.extend(node_instructions);
    }
    instr
}
