use std::collections::HashMap;
mod ast;
mod error;
mod parser;
mod tokenizer;
mod types;

use crate::ast::ASTNode;
use crate::parser::Parser;
use crate::tokenizer::{Token, Tokenizer};

// #[derive(Debug, Clone)]
// enum Instruction {
//     Push(i32),
//     Pop,
//     Add,
//     Sub,
//     Mul,
//     Div,
//     Greater,
//     Less,
//     Equal,
//     NotEqual,
//     Jmp(usize),
//     Jz(usize),
//     Label(String),
//     Store(String),
//     Load(String),
//     BeginScope,
//     EndScope,
//     CreateArray(usize),
//     ArrayPush,
//     ArrayPop,
//     ArrayGet,
//     ArraySet,
// }

// struct VM {
//     stack: Vec<i32>,
//     ip: usize,
//     env_stack: Vec<HashMap<String, i32>>,
// }

// impl VM {
//     fn new() -> Self {
//         VM {
//             stack: Vec::new(),
//             ip: 0,
//             env_stack: vec![HashMap::new()], // Start with global scope
//         }
//     }

//     fn current_env(&mut self) -> &mut HashMap<String, i32> {
//         self.env_stack.last_mut().expect("No environment on stack")
//     }

//     fn get_var(&self, name: &str) -> Option<i32> {
//         for env in self.env_stack.iter().rev() {
//             if let Some(value) = env.get(name) {
//                 return Some(*value);
//             }
//         }
//         None
//     }

//     fn execute(&mut self, instructions: &[Instruction]) {
//         while self.ip < instructions.len() {
//             match &instructions[self.ip] {
//                 Instruction::Push(value) => self.stack.push(*value),
//                 Instruction::Pop => {
//                     self.stack.pop().expect("Stack underflow");
//                 }
//                 Instruction::Add => {
//                     let b = self.stack.pop().expect("Stack underflow");
//                     let a = self.stack.pop().expect("Stack underflow");
//                     self.stack.push(a + b);
//                 }
//                 Instruction::Sub => {
//                     let b = self.stack.pop().expect("Stack underflow");
//                     let a = self.stack.pop().expect("Stack underflow");
//                     self.stack.push(a - b);
//                 }
//                 Instruction::Mul => {
//                     let b = self.stack.pop().expect("Stack underflow");
//                     let a = self.stack.pop().expect("Stack underflow");
//                     self.stack.push(a * b);
//                 }
//                 Instruction::Div => {
//                     let b = self.stack.pop().expect("Stack underflow");
//                     let a = self.stack.pop().expect("Stack underflow");
//                     self.stack.push(a / b);
//                 }
//                 Instruction::Greater => {
//                     let b = self.stack.pop().expect("Stack underflow");
//                     let a = self.stack.pop().expect("Stack underflow");
//                     self.stack.push(if a > b { 1 } else { 0 });
//                 }
//                 Instruction::Less => {
//                     let b = self.stack.pop().expect("Stack underflow");
//                     let a = self.stack.pop().expect("Stack underflow");
//                     self.stack.push(if a < b { 1 } else { 0 });
//                 }
//                 Instruction::Equal => {
//                     let b = self.stack.pop().expect("Stack underflow");
//                     let a = self.stack.pop().expect("Stack underflow");
//                     self.stack.push(if a == b { 1 } else { 0 });
//                 }
//                 Instruction::NotEqual => {
//                     let b = self.stack.pop().expect("Stack underflow");
//                     let a = self.stack.pop().expect("Stack underflow");
//                     self.stack.push(if a != b { 1 } else { 0 });
//                 }
//                 Instruction::Jmp(target) => {
//                     self.ip = *target;
//                     continue;
//                 }
//                 Instruction::Jz(target) => {
//                     let condition = self.stack.pop().expect("Stack underflow");
//                     if condition == 0 {
//                         self.ip = *target;
//                         continue;
//                     }
//                 }
//                 Instruction::Label(_) => {}
//                 Instruction::Store(name) => {
//                     let value = self.stack.pop().expect("Stack underflow");
//                     self.current_env().insert(name.clone(), value);
//                 }
//                 Instruction::Load(name) => {
//                     let value = self.get_var(name).expect("Variable not found");
//                     self.stack.push(value);
//                 }
//                 Instruction::BeginScope => {
//                     self.env_stack.push(HashMap::new());
//                 }
//                 Instruction::EndScope => {
//                     self.env_stack.pop().expect("No scope to end");
//                 }
//             }
//             self.ip += 1;
//         }
//     }
// }

// fn compile(node: ASTNode) -> Vec<Instruction> {
//     match node {
//         ASTNode::Number(n) => vec![Instruction::Push(n)],
//         ASTNode::BinOp { left, op, right } => {
//             let mut instructions = compile(*left);
//             instructions.extend(compile(*right));
//             match op {
//                 Token::Plus => instructions.push(Instruction::Add),
//                 Token::Minus => instructions.push(Instruction::Sub),
//                 Token::Star => instructions.push(Instruction::Mul),
//                 Token::Slash => instructions.push(Instruction::Div),
//                 Token::Greater => instructions.push(Instruction::Greater),
//                 Token::Less => instructions.push(Instruction::Less),
//                 Token::Equal => instructions.push(Instruction::Equal),
//                 Token::NotEqual => instructions.push(Instruction::NotEqual),
//                 _ => panic!("Unsupported operation"),
//             }
//             instructions
//         }
//         ASTNode::If {
//             condition,
//             if_block,
//             else_block,
//         } => {
//             let mut instructions = compile(*condition);
//             let if_instructions: Vec<Instruction> =
//                 if_block.into_iter().flat_map(compile).collect();
//             let else_instructions: Vec<Instruction> =
//                 else_block.into_iter().flat_map(compile).collect();

//             // Calculate the position where else block starts:
//             let else_start = instructions.len() + 1 + if_instructions.len() + 1;
//             instructions.push(Instruction::Jz(else_start));

//             instructions.extend(if_instructions);

//             // Calculate the position after the entire if-else block:
//             let after_else = instructions.len() + 1 + else_instructions.len();
//             instructions.push(Instruction::Jmp(after_else));

//             instructions.extend(else_instructions);

//             instructions
//         }
//         ASTNode::While { condition, body } => {
//             let mut instructions = Vec::new();
//             // Record where condition check starts
//             let condition_start = instructions.len();
//             instructions.extend(compile(*condition));

//             // Record where we'll put the Jz instruction
//             let jz_placeholder_index = instructions.len();
//             instructions.push(Instruction::Jz(0)); // Temporary placeholder

//             let body_instructions: Vec<Instruction> = body.into_iter().flat_map(compile).collect();
//             let body_len = body_instructions.len();
//             instructions.extend(body_instructions);
//             instructions.push(Instruction::Jmp(condition_start));

//             let after_loop = jz_placeholder_index + 1 + body_len + 1;
//             instructions[jz_placeholder_index] = Instruction::Jz(after_loop);

//             instructions
//         }
//         ASTNode::VarDecl(name, value) => {
//             let mut instructions = compile(*value);
//             instructions.push(Instruction::Store(name));
//             instructions
//         }
//         ASTNode::VarAssign(name, value) => {
//             let mut instructions = compile(*value);
//             instructions.push(Instruction::Store(name));
//             instructions
//         }
//         ASTNode::VarRef(name) => {
//             vec![Instruction::Load(name)]
//         }
//         ASTNode::Block(nodes) => {
//             let mut instructions = vec![Instruction::BeginScope];
//             instructions.extend(nodes.into_iter().flat_map(compile));
//             instructions.push(Instruction::EndScope);
//             instructions
//         }
//     }
// }

// fn run_instructions(nodes: Vec<ASTNode>) -> Vec<Instruction> {
//     let mut instr = Vec::new();
//     let mut offset = 0;

//     for node in nodes {
//         let mut node_instructions = compile(node);

//         for instruction in &mut node_instructions {
//             match instruction {
//                 Instruction::Jmp(target) => *target += offset,
//                 Instruction::Jz(target) => *target += offset,
//                 _ => {}
//             }
//         }

//         offset += node_instructions.len();
//         instr.extend(node_instructions);
//     }
//     instr
// }

fn main() {
    let program = r#"
let x = 5
while (x > 0) {
    x = x - 1
}
   y + 5
    "#
    .to_string();

    let tokenizer = Tokenizer::new(program);
    let mut parser = Parser::new(tokenizer);
    let ast_nodes = parser.parse_program();
    println!("AST: {:?}\n", ast_nodes);
    // let instructions = run_instructions(ast_nodes);

    // println!(
    //     "Instructions: {:?}, len: {:?})",
    //     instructions,
    //     instructions.len()
    // );

    // let mut vm = VM::new();
    // vm.execute(&instructions);

    // println!("Stack: {:?}", vm.stack);
    // println!("Variables: {:?}", vm.env_stack);
}
