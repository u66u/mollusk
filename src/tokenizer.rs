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
    LBracket,
    RBracket,
    Comma,
    If,
    Else,
    While,
    EOF,
    Greater,
    Less,
    Equal,
    NotEqual,
    Ident(String),
    String(String),
    Assignment,
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
            let input_slice = &self.input[self.position..];
            let c = input_slice.chars().next().unwrap();
            
            match c {
                '0'..='9' => {
                    let mut num = 0;
                    let mut start = self.position;
                    while start < self.input.len() && self.input[start..].chars().next().unwrap().is_ascii_digit() {
                        num = num * 10 + (self.input[start..].chars().next().unwrap() as i32 - '0' as i32);
                        start += 1;
                    }
                    self.line_position += start - self.position;
                    self.position = start;
                    return Ok(Token::Number(num));
                }
                '"' => {
                    self.position += 1; // Skip opening quote
                    let mut string = String::new();
                    let mut escaped = false;
                    
                    while self.position < self.input.len() {
                        let c = self.input[self.position..].chars().next().unwrap();
                        self.position += 1;
                        
                        if escaped {
                            string.push(match c {
                                'n' => '\n',
                                't' => '\t',
                                'r' => '\r',
                                '"' => '"',
                                '\\' => '\\',
                                _ => return Err(VMError::TokenizationError {
                                    message: format!("Invalid escape sequence: \\{}", c),
                                    line: self.line,
                                    position: self.line_position,
                                }),
                            });
                            escaped = false;
                        } else if c == '\\' {
                            escaped = true;
                        } else if c == '"' {
                            return Ok(Token::String(string));
                        } else {
                            string.push(c);
                        }
                    }
                    return Err(VMError::TokenizationError {
                        message: "Unterminated string literal".to_string(),
                        line: self.line,
                        position: self.line_position,
                    });
                }
                '+' | '-' | '*' | '/' | '(' | ')' | '{' | '}' | '>' | '<' | '!' | '[' | ']' | ',' | '=' => {
                    let (token, advance) = match c {
                        '+' => (Token::Plus, 1),
                        '-' => (Token::Minus, 1),
                        '*' => (Token::Star, 1),
                        '/' => (Token::Slash, 1),
                        '(' => (Token::LParen, 1),
                        ')' => (Token::RParen, 1),
                        '{' => (Token::LBrace, 1),
                        '}' => (Token::RBrace, 1),
                        '[' => (Token::LBracket, 1),
                        ']' => (Token::RBracket, 1),
                        ',' => (Token::Comma, 1),
                        '>' => (Token::Greater, 1),
                        '<' => (Token::Less, 1),
                        '=' => {
                            if input_slice.starts_with("==") {
                                (Token::Equal, 2)
                            } else {
                                (Token::Assignment, 1)
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
