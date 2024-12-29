use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    LBrace,
    RBrace,
    If,
    Else,
    While,
    EOF,
    Greater,
    Less,
    Equal,
    NotEqual,
    Ident(String),
}

#[derive(Debug, Clone)]
enum Instruction {
    Push(i32),
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
}

#[derive(Debug, Clone)]
enum ASTNode {
    Number(i32),
    BinOp {
        left: Box<ASTNode>,
        op: Token,
        right: Box<ASTNode>,
    },
    If {
        condition: Box<ASTNode>,
        if_block: Vec<ASTNode>,
        else_block: Vec<ASTNode>,
    },
    While {
        condition: Box<ASTNode>,
        body: Vec<ASTNode>,
    },
    VarDecl(String, Box<ASTNode>),
    VarAssign(String, Box<ASTNode>),
    VarRef(String),
    Block(Vec<ASTNode>),
}

struct VM {
    stack: Vec<i32>,
    ip: usize,
    env_stack: Vec<HashMap<String, i32>>,
}

impl VM {
    fn new() -> Self {
        VM {
            stack: Vec::new(),
            ip: 0,
            env_stack: vec![HashMap::new()], // Start with global scope
        }
    }

    fn current_env(&mut self) -> &mut HashMap<String, i32> {
        self.env_stack.last_mut().expect("No environment on stack")
    }

    fn get_var(&self, name: &str) -> Option<i32> {
        for env in self.env_stack.iter().rev() {
            if let Some(value) = env.get(name) {
                return Some(*value);
            }
        }
        None
    }

    fn execute(&mut self, instructions: &[Instruction]) {
        while self.ip < instructions.len() {
            match &instructions[self.ip] {
                Instruction::Push(value) => self.stack.push(*value),
                Instruction::Pop => {
                    self.stack.pop().expect("Stack underflow");
                }
                Instruction::Add => {
                    let b = self.stack.pop().expect("Stack underflow");
                    let a = self.stack.pop().expect("Stack underflow");
                    self.stack.push(a + b);
                }
                Instruction::Sub => {
                    let b = self.stack.pop().expect("Stack underflow");
                    let a = self.stack.pop().expect("Stack underflow");
                    self.stack.push(a - b);
                }
                Instruction::Mul => {
                    let b = self.stack.pop().expect("Stack underflow");
                    let a = self.stack.pop().expect("Stack underflow");
                    self.stack.push(a * b);
                }
                Instruction::Div => {
                    let b = self.stack.pop().expect("Stack underflow");
                    let a = self.stack.pop().expect("Stack underflow");
                    self.stack.push(a / b);
                }
                Instruction::Greater => {
                    let b = self.stack.pop().expect("Stack underflow");
                    let a = self.stack.pop().expect("Stack underflow");
                    self.stack.push(if a > b { 1 } else { 0 });
                }
                Instruction::Less => {
                    let b = self.stack.pop().expect("Stack underflow");
                    let a = self.stack.pop().expect("Stack underflow");
                    self.stack.push(if a < b { 1 } else { 0 });
                }
                Instruction::Equal => {
                    let b = self.stack.pop().expect("Stack underflow");
                    let a = self.stack.pop().expect("Stack underflow");
                    self.stack.push(if a == b { 1 } else { 0 });
                }
                Instruction::NotEqual => {
                    let b = self.stack.pop().expect("Stack underflow");
                    let a = self.stack.pop().expect("Stack underflow");
                    self.stack.push(if a != b { 1 } else { 0 });
                }
                Instruction::Jmp(target) => {
                    self.ip = *target;
                    continue;
                }
                Instruction::Jz(target) => {
                    let value = self.stack.pop().expect("Stack underflow");
                    if value == 0 {
                        self.ip = *target;
                        continue;
                    }
                }
                Instruction::Label(_) => {}
                Instruction::Store(name) => {
                    let value = self.stack.pop().expect("Stack underflow");
                    self.current_env().insert(name.clone(), value);
                }

                Instruction::Load(name) => {
                    if let Some(value) = self.get_var(name) {
                        self.stack.push(value);
                    } else {
                        panic!("Undefined variable: {}", name);
                    }
                }

                Instruction::BeginScope => {
                    self.env_stack.push(HashMap::new());
                }

                Instruction::EndScope => {
                    self.env_stack.pop().expect("No scope to pop");
                }
            }
            self.ip += 1;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Tokenizer {
    input: String,
    position: usize,
}

impl Tokenizer {
    fn new(input: String) -> Self {
        Tokenizer { input, position: 0 }
    }

    fn next_token(&mut self) -> Token {
        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();
            match c {
                '0'..='9' => {
                    let mut num = 0;
                    while self.position < self.input.len() {
                        let c = self.input.chars().nth(self.position).unwrap();
                        if !c.is_ascii_digit() {
                            break;
                        }
                        num = num * 10 + (c as i32 - '0' as i32);
                        self.position += 1;
                    }
                    return Token::Number(num);
                }
                '+' => {
                    self.position += 1;
                    return Token::Plus;
                }
                '-' => {
                    self.position += 1;
                    return Token::Minus;
                }
                '*' => {
                    self.position += 1;
                    return Token::Star;
                }
                '/' => {
                    self.position += 1;
                    return Token::Slash;
                }
                '(' => {
                    self.position += 1;
                    return Token::LParen;
                }
                ')' => {
                    self.position += 1;
                    return Token::RParen;
                }
                '{' => {
                    self.position += 1;
                    return Token::LBrace;
                }
                '}' => {
                    self.position += 1;
                    return Token::RBrace;
                }
                '>' => {
                    self.position += 1;
                    return Token::Greater;
                }
                '<' => {
                    self.position += 1;
                    return Token::Less;
                }
                '=' => {
                    if self.input[self.position..].starts_with("==") {
                        self.position += 2;
                        return Token::Equal;
                    } else {
                        self.position += 1;
                        return Token::Ident("=".to_string());
                    }
                }
                '!' => {
                    if self.input[self.position..].starts_with("!=") {
                        self.position += 2;
                        return Token::NotEqual;
                    } else {
                        panic!("Unexpected token: !");
                    }
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut ident = String::new();
                    while self.position < self.input.len() {
                        let c = self.input.chars().nth(self.position).unwrap();
                        if !c.is_ascii_alphanumeric() && c != '_' {
                            break;
                        }
                        ident.push(c);
                        self.position += 1;
                    }
                    match ident.as_str() {
                        "if" => return Token::If,
                        "else" => return Token::Else,
                        "while" => return Token::While,
                        _ => return Token::Ident(ident),
                    }
                }
                _ => {
                    self.position += 1;
                }
            }
        }
        Token::EOF
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            tokens.push(token.clone());
            if token == Token::EOF {
                break;
            }
        }
        tokens
    }
}

#[derive(Debug)]
struct Parser {
    tokenizer: Tokenizer,
    current_token: Token,
}

impl Parser {
    fn new(mut tokenizer: Tokenizer) -> Self {
        let current_token = tokenizer.next_token();
        Parser {
            tokenizer,
            current_token,
        }
    }

    fn parse_program(&mut self) -> Vec<ASTNode> {
        let mut statements = Vec::new();
        while self.current_token != Token::EOF {
            statements.push(self.statement());
        }
        statements
    }

    fn eat(&mut self, token: Token) {
        if self.current_token == token {
            self.current_token = self.tokenizer.next_token();
        } else {
            println!("Expected: {:?}, Got: {:?}", token, self.current_token);
            panic!("Unexpected token");
        }
    }

    fn factor(&mut self) -> ASTNode {
        match self.current_token {
            Token::Number(n) => {
                self.eat(Token::Number(n));
                ASTNode::Number(n)
            }
            Token::LParen => {
                self.eat(Token::LParen);
                let node = self.comparison();
                self.eat(Token::RParen);
                node
            }
            Token::Ident(_) => {
                let var_name = if let Token::Ident(name) = &self.current_token {
                    name.clone()
                } else {
                    panic!("Expected identifier");
                };
                self.eat(Token::Ident(var_name.clone()));
                ASTNode::VarRef(var_name)
            }
            _ => panic!(
                "Unexpected token in factor: expected Number, LParen, or Ident, found {:?}",
                self.current_token
            ),
        }
    }

    fn term(&mut self) -> ASTNode {
        let mut node = self.factor();
        while matches!(self.current_token, Token::Star | Token::Slash) {
            let op = self.current_token.clone();
            match op {
                Token::Star => self.eat(Token::Star),
                Token::Slash => self.eat(Token::Slash),
                _ => panic!("Unexpected token in term"),
            }
            node = ASTNode::BinOp {
                left: Box::new(node),
                op,
                right: Box::new(self.factor()),
            };
        }
        node
    }

    fn expr(&mut self) -> ASTNode {
        let mut node = self.term();
        while matches!(self.current_token, Token::Plus | Token::Minus) {
            let op = self.current_token.clone();
            match op {
                Token::Plus => self.eat(Token::Plus),
                Token::Minus => self.eat(Token::Minus),
                _ => panic!("Unexpected token in expr"),
            }
            node = ASTNode::BinOp {
                left: Box::new(node),
                op,
                right: Box::new(self.term()),
            };
        }
        node
    }

    fn comparison(&mut self) -> ASTNode {
        let mut node = self.expr();
        while matches!(
            self.current_token,
            Token::Greater | Token::Less | Token::Equal | Token::NotEqual
        ) {
            let op = self.current_token.clone();
            match op {
                Token::Greater => self.eat(Token::Greater),
                Token::Less => self.eat(Token::Less),
                Token::Equal => self.eat(Token::Equal),
                Token::NotEqual => self.eat(Token::NotEqual),
                _ => panic!("Unexpected token in comparison"),
            }
            node = ASTNode::BinOp {
                left: Box::new(node),
                op,
                right: Box::new(self.expr()),
            };
        }
        node
    }

    fn if_statement(&mut self) -> ASTNode {
        self.eat(Token::If);
        self.eat(Token::LParen);
        let condition = self.comparison();
        self.eat(Token::RParen);
        let if_block = self.block();
        let else_block = if self.current_token == Token::Else {
            self.eat(Token::Else);
            self.block()
        } else {
            Vec::new()
        };
        ASTNode::If {
            condition: Box::new(condition),
            if_block,
            else_block,
        }
    }

    fn while_loop(&mut self) -> ASTNode {
        self.eat(Token::While);
        self.eat(Token::LParen);
        let condition = self.comparison();
        self.eat(Token::RParen);
        let body = self.block();
        ASTNode::While {
            condition: Box::new(condition),
            body,
        }
    }

    fn block(&mut self) -> Vec<ASTNode> {
        self.eat(Token::LBrace);
        let mut nodes = Vec::new();
        while self.current_token != Token::RBrace {
            nodes.push(self.statement());
        }
        self.eat(Token::RBrace);
        nodes
    }

    fn statement(&mut self) -> ASTNode {
        match self.current_token {
            Token::If => self.if_statement(),
            Token::While => self.while_loop(),
            Token::LBrace => ASTNode::Block(self.block()),
            Token::Ident(_) => self.var_statement(),
            _ => self.expr(),
        }
    }

    fn var_statement(&mut self) -> ASTNode {
        let var_name = if let Token::Ident(name) = &self.current_token {
            name.clone()
        } else {
            panic!("Expected identifier");
        };
        self.eat(Token::Ident(var_name.clone()));
        if self.current_token == Token::Ident("=".to_string()) {
            self.eat(Token::Ident("=".to_string()));
            let value = self.expr();
            ASTNode::VarAssign(var_name, Box::new(value))
        } else {
            ASTNode::VarRef(var_name)
        }
    }
}

fn compile(node: ASTNode) -> Vec<Instruction> {
    match node {
        ASTNode::Number(n) => vec![Instruction::Push(n)],
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

            // Calculate the position where else block starts:
            let else_start = instructions.len() + 1 + if_instructions.len() + 1;
            instructions.push(Instruction::Jz(else_start));

            instructions.extend(if_instructions);

            // Calculate the position after the entire if-else block:
            let after_else = instructions.len() + 1 + else_instructions.len();
            instructions.push(Instruction::Jmp(after_else));

            instructions.extend(else_instructions);

            instructions
        }
        ASTNode::While { condition, body } => {
            let mut instructions = compile(*condition);
            let body_instructions: Vec<Instruction> = body.into_iter().flat_map(compile).collect();

            // Jz target: Jump to the end of the loop if the condition is false
            let jz_target = instructions.len() + body_instructions.len() + 2;
            instructions.push(Instruction::Jz(jz_target));

            instructions.extend(body_instructions.clone());

            // Jmp target: Jump back to the start of the loop
            let jmp_target = 0;
            instructions.push(Instruction::Jmp(jmp_target));

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

fn main() {
    let program = r#"
        x = 5
        { y = 5}
        y = 6 
        z = (y + 6)
        (z / y)
    "#
    .to_string();

    let tokenizer = Tokenizer::new(program);
    let mut parser = Parser::new(tokenizer);
    let ast_nodes = parser.parse_program();
    println!("AST: {:?}", ast_nodes);

    let instructions: Vec<Instruction> = ast_nodes.into_iter().flat_map(compile).collect();
    println!("Instructions: {:?}", instructions);

    let mut vm = VM::new();
    vm.execute(&instructions);

    println!("Stack: {:?}", vm.stack);
    println!("Variables: {:?}", vm.env_stack);
}
