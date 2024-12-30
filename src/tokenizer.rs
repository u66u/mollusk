use crate::error::VMError;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
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
#[derive(Debug, Clone, PartialEq)]
pub struct Tokenizer {
    pub input: String,
    pub position: usize,
    pub line: usize,          
    pub line_position: usize, 
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        Tokenizer {
            input,
            position: 0,
            line: 1,
            line_position: 1,
        }
    }

        pub fn next_token(&mut self) -> Result<Token, VMError> {
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
                            self.line_position += 1;
                        }
                        return Ok(Token::Number(num));
                    }
    
                    
                    '+' | '-' | '*' | '/' | '(' | ')' | '{' | '}' | '>' | '<' | '=' | '!' => {
                        let (token, advance) = match c {
                            '+' => (Token::Plus, 1),
                            '-' => (Token::Minus, 1),
                            '*' => (Token::Star, 1),
                            '/' => (Token::Slash, 1),
                            '(' => (Token::LParen, 1),
                            ')' => (Token::RParen, 1),
                            '{' => (Token::LBrace, 1),
                            '}' => (Token::RBrace, 1),
                            '>' => (Token::Greater, 1),
                            '<' => (Token::Less, 1),
                            '=' => {
                                if self.input[self.position..].starts_with("==") {
                                    (Token::Equal, 2)
                                } else {
                                    (Token::Ident("=".to_string()), 1)
                                }
                            }
                            '!' => {
                                if self.input[self.position..].starts_with("!=") {
                                    (Token::NotEqual, 2)
                                } else {
                                    return Err(VMError::TokenizationError {
                                        message: "Unexpected token: !".to_string(),
                                        line: self.line,
                                        position: self.line_position,
                                    });
                                }
                            }
                            _ => unreachable!(),
                        };
                        self.position += advance;
                        self.line_position += advance;
                        return Ok(token);
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
                            self.line_position += 1;
                        }
                        match ident.as_str() {
                            "if" => return Ok(Token::If),
                            "else" => return Ok(Token::Else),
                            "while" => return Ok(Token::While),
                            _ => return Ok(Token::Ident(ident)),
                        }
                    }
    
                    
                    ' ' | '\t' => {
                        self.position += 1;
                        self.line_position += 1;
                    }
    
                    
                    '\n' => {
                        self.position += 1;
                        self.line += 1;
                        self.line_position = 1;
                    }
    
                    
                    _ => {
                        return Err(VMError::TokenizationError {
                            message: format!("Unexpected character: {}", c),
                            line: self.line,
                            position: self.line_position,
                        });
                    }
                }
            }
            Ok(Token::EOF)
        }
    }