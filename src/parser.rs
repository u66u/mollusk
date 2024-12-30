use crate::ast::ASTNode;
use crate::error::VMError;
use crate::tokenizer::{Token, Tokenizer};

#[derive(Debug)]
pub struct Parser {
    pub tokenizer: Tokenizer,
    pub current_token: Token,
}

impl Parser {
    pub fn new(mut tokenizer: Tokenizer) -> Self {
        let current_token = tokenizer.next_token().unwrap();
        Parser {
            tokenizer,
            current_token,
        }
    }

    fn eat(&mut self, token: Token) -> Result<(), VMError> {
        if self.current_token == token {
            self.current_token = self.tokenizer.next_token()?;
            Ok(())
        } else {
            Err(VMError::ParseError {
                message: format!("Expected: {:?}, Got: {:?}", token, self.current_token),
                line: self.tokenizer.line,
                position: self.tokenizer.line_position,
            })
        }
    }

    fn factor(&mut self) -> Result<ASTNode, VMError> {
        match self.current_token {
            Token::Number(n) => {
                self.eat(Token::Number(n))?;
                Ok(ASTNode::Number(n))
            }
            Token::LParen => {
                self.eat(Token::LParen)?;
                let node = self.comparison()?;
                self.eat(Token::RParen)?;
                Ok(node)
            }
            Token::Ident(_) => {
                let var_name = if let Token::Ident(name) = &self.current_token {
                    name.clone()
                } else {
                    return Err(VMError::ParseError {
                        message: "Expected identifier".to_string(),
                        line: self.tokenizer.line,
                        position: self.tokenizer.line_position,
                    });
                };
                self.eat(Token::Ident(var_name.clone()))?;
                Ok(ASTNode::VarRef(var_name))
            }
            _ => Err(VMError::ParseError {
                message: format!(
                    "Unexpected token in factor: expected Number, LParen, or Ident, found {:?}",
                    self.current_token
                ),
                line: self.tokenizer.line,
                position: self.tokenizer.line_position,
            }),
        }
    }

    fn term(&mut self) -> Result<ASTNode, VMError> {
        let mut node = self.factor()?;
        while matches!(self.current_token, Token::Star | Token::Slash) {
            let op = self.current_token.clone();
            match op {
                Token::Star => self.eat(Token::Star)?,
                Token::Slash => self.eat(Token::Slash)?,
                _ => unreachable!(),
            }
            node = ASTNode::BinOp {
                left: Box::new(node),
                op,
                right: Box::new(self.factor()?),
            };
        }
        Ok(node)
    }

    fn expr(&mut self) -> Result<ASTNode, VMError> {
        let mut node = self.term()?;
        while matches!(self.current_token, Token::Plus | Token::Minus) {
            let op = self.current_token.clone();
            match op {
                Token::Plus => self.eat(Token::Plus)?,
                Token::Minus => self.eat(Token::Minus)?,
                _ => unreachable!(),
            }
            node = ASTNode::BinOp {
                left: Box::new(node),
                op,
                right: Box::new(self.term()?),
            };
        }
        Ok(node)
    }

    fn comparison(&mut self) -> Result<ASTNode, VMError> {
        let mut node = self.expr()?;
        while matches!(
            self.current_token,
            Token::Greater | Token::Less | Token::Equal | Token::NotEqual
        ) {
            let op = self.current_token.clone();
            match op {
                Token::Greater => self.eat(Token::Greater)?,
                Token::Less => self.eat(Token::Less)?,
                Token::Equal => self.eat(Token::Equal)?,
                Token::NotEqual => self.eat(Token::NotEqual)?,
                _ => unreachable!(),
            }
            node = ASTNode::BinOp {
                left: Box::new(node),
                op,
                right: Box::new(self.expr()?),
            };
        }
        Ok(node)
    }

    fn if_statement(&mut self) -> Result<ASTNode, VMError> {
        self.eat(Token::If)?;
        self.eat(Token::LParen)?;
        let condition = self.comparison()?;
        self.eat(Token::RParen)?;
        let if_block = self.block()?;
        let else_block = if self.current_token == Token::Else {
            self.eat(Token::Else)?;
            self.block()?
        } else {
            Vec::new()
        };
        Ok(ASTNode::If {
            condition: Box::new(condition),
            if_block,
            else_block,
        })
    }

    fn while_loop(&mut self) -> Result<ASTNode, VMError> {
        self.eat(Token::While)?;
        self.eat(Token::LParen)?;
        let condition = self.comparison()?;
        self.eat(Token::RParen)?;
        let body = self.block()?;
        Ok(ASTNode::While {
            condition: Box::new(condition),
            body,
        })
    }

    fn block(&mut self) -> Result<Vec<ASTNode>, VMError> {
        self.eat(Token::LBrace)?;
        let mut nodes = Vec::new();
        while self.current_token != Token::RBrace {
            nodes.push(self.statement()?);
        }
        self.eat(Token::RBrace)?;
        Ok(nodes)
    }

    fn statement(&mut self) -> Result<ASTNode, VMError> {
        match self.current_token {
            Token::If => self.if_statement(),
            Token::While => self.while_loop(),
            Token::LBrace => Ok(ASTNode::Block(self.block()?)),
            Token::Ident(_) => self.var_statement(),
            _ => self.expr(),
        }
    }

    fn var_statement(&mut self) -> Result<ASTNode, VMError> {
        let var_name = if let Token::Ident(name) = &self.current_token {
            name.clone()
        } else {
            return Err(VMError::ParseError {
                message: "Expected identifier".to_string(),
                line: self.tokenizer.line,
                position: self.tokenizer.line_position,
            });
        };
        self.eat(Token::Ident(var_name.clone()))?;
        if self.current_token == Token::Ident("=".to_string()) {
            self.eat(Token::Ident("=".to_string()))?;
            let value = self.expr()?;
            Ok(ASTNode::VarAssign(var_name, Box::new(value)))
        } else {
            Ok(ASTNode::VarRef(var_name))
        }
    }

    pub fn parse_program(&mut self) -> Result<Vec<ASTNode>, VMError> {
        let mut statements = Vec::new();
        while self.current_token != Token::EOF {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }
}
