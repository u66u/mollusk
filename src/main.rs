mod ast;
mod error;
mod parser;
mod tokenizer;
mod types;
mod vm;

use crate::parser::Parser;
use crate::tokenizer::{Token, Tokenizer};
use crate::vm::{run_instructions, VM};

fn main() {
    let program = r#"
    x = [1,2,3]
     x[0] = 10
     y = x[0]
     y
    z = "hi"
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
        match vm.execute(&instructions) {
            Ok(_) => {
                println!("VM stack: {:?}", vm.stack);
                println!("VM env: {:?}", vm.env_stack);
            }
            Err(e) => println!("Runtime error: {}", e),
        }
    }
}
