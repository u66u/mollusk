use std::collections::HashMap;
mod ast;
mod error;
mod parser;
mod tokenizer;
mod types;
mod vm;

use crate::parser::Parser;
use crate::tokenizer::{Token, Tokenizer};
use crate::vm::{VM, run_instructions};

fn main() {
    let program = r#"
    x = 0
    if (x > 10) {
        y = x + 5
    } else {
        y = x - 5
    }
    while (x < 10) {
        x = x + 1
    }
    i = 10
    y = 5
    "#
    .to_string();


    let tokenizer = Tokenizer::new(program);
    let mut parser = Parser::new(tokenizer);
    let ast_nodes = parser.parse_program();
    println!("AST: {:?}\n", ast_nodes);
    if let Ok(nodes) = ast_nodes {
        let instructions = run_instructions(nodes);
        println!("Instructions: {:?}\n", instructions);
        let mut vm = VM::new();
        let _ = vm.execute(&instructions);
        println!("VM stack: {:?}", vm.stack);
        println!("VM env: {:?}", vm.env_stack);
    }

}
