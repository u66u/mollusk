use std::collections::HashMap;
use crate::ast::ASTNode;
use crate::tokenizer::Token;
use crate::types::{Value, VMBinaryOp, VMCompare, VMArray};
use crate::error::VMError;

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
}


impl VM {
    pub fn new() -> Self {
        VM {
            stack: Vec::new(),
            ip: 0,
            env_stack: vec![HashMap::new()], // Start with global scope
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

    pub fn execute(&mut self, instructions: &[Instruction]) -> Result<(), VMError> {
        while self.ip < instructions.len() {
            // println!("node: {:?}", instructions[self.ip]);
            match &instructions[self.ip] {
                Instruction::Push(value) => {
                    self.stack.push(value.clone());
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
                    self.ip = *target;
                    continue;
                }
                Instruction::Jz(target) => {
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
                        return Err(VMError::UndefinedVariable {
                            name: name.clone()
                        });
                    }
                }
                Instruction::BeginScope => {
                    self.env_stack.push(HashMap::new());
                }
                Instruction::EndScope => {
                        self.env_stack.pop().ok_or(VMError::NoScopeToEnd)?;
                }
                Instruction::CreateArray => {
                    self.stack.push(Value::Array(Vec::new()));
                }
                Instruction::ArrayOp(op) => {
                    match op {
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
                        ArrayOperation::Get(index) => {
                            let array = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                            let value = array.get(Some(*index as i32))?;
                            self.stack.push(array);
                            self.stack.push(value);
                        }
                        ArrayOperation::Set(index) => {
                            let value = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                            let mut array = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                            array.set(Some(*index as i32), value)?;
                            self.stack.push(array);
                        }
                    }
                }
            }
            self.ip += 1;
        }
        Ok(())
    }
}

pub fn compile(node: ASTNode) -> Vec<Instruction> {
    match node {
        ASTNode::Number(n) => vec![Instruction::Push(Value::Number(n))],
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
        ASTNode::If { condition, if_block, else_block } => {
            let mut instructions = compile(*condition);
            let if_instructions: Vec<Instruction> = if_block.into_iter().flat_map(compile).collect();
            let else_instructions: Vec<Instruction> = else_block.into_iter().flat_map(compile).collect();
        
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
        ASTNode::VarRef(name) => {
            vec![Instruction::Load(name)]
        }
        ASTNode::Block(nodes) => {
            let mut instructions = vec![Instruction::BeginScope];
            instructions.extend(nodes.into_iter().flat_map(compile));
            instructions.push(Instruction::EndScope);
            instructions
        }
    }
}

pub fn run_instructions(nodes: Vec<ASTNode>) -> Vec<Instruction> {
    let mut instr = Vec::new();
    let mut offset = 0;

    for node in nodes {
        let mut node_instructions = compile(node);
        
        let mut scope_count = 0;
        for instruction in &node_instructions {
            match instruction {
                Instruction::BeginScope => scope_count += 1,
                Instruction::EndScope => scope_count -= 1,
                _ => {}
            }
        }

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