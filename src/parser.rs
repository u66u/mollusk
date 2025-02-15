use crate::ast::ASTNode;
use crate::error::VMError;
use crate::tokenizer::{Token, Tokenizer};

#[derive(Debug)]
pub struct Parser {
    pub tokenizer: Tokenizer,
    pub current_token: Token,
    token_start: usize,
}

impl Parser {
    pub fn new(mut tokenizer: Tokenizer) -> Self {
        let token_start = tokenizer.position;
        let current_token = tokenizer.next_token().unwrap();
        Parser {
            tokenizer,
            current_token,
            token_start,
        }
    }

    fn create_error(&self, message: String) -> VMError {
        VMError::parse_error(
            self.tokenizer.input.clone(),
            message,
            self.token_start,
            self.tokenizer.position - self.token_start
        )
    }

    fn eat(&mut self, token: Token) -> Result<(), VMError> {
        if self.current_token == token {
            self.token_start = self.tokenizer.position;
            self.current_token = self.tokenizer.next_token()?;
            Ok(())
        } else {
            Err(self.create_error(
                format!("Expected: {:?}, Got: {:?}", token, self.current_token)
            ))
        }
    }

    fn error(&self, message: &str) -> VMError {
        self.create_error(format!("{}", message))
    }

    fn factor(&mut self) -> Result<ASTNode, VMError> {
        match &self.current_token {
            Token::Number(n) => {
                let num = *n;
                self.eat(Token::Number(num))?;
                Ok(ASTNode::Number(num))
            }
            Token::String(s) => {
                let str = s.clone();
                self.eat(Token::String(str.clone()))?;
                Ok(ASTNode::String(str))
            }
            Token::LParen => {
                self.eat(Token::LParen)?;
                let node = self.comparison()?;
                self.eat(Token::RParen)?;
                Ok(node)
            }
            Token::Ident(name) => {
                let var_name = name.clone();
                self.eat(Token::Ident(var_name.clone()))?;
                if self.current_token == Token::LBracket {
                    self.array_index(ASTNode::VarRef(var_name))
                } else {
                    Ok(ASTNode::VarRef(var_name))
                }
            }
            Token::LBracket => self.array_literal(),

            _ => Err(self.error("Expected number, string, identifier, or '('")),

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
        let body = if self.current_token == Token::LBrace {
            self.block()?
        } else {
            vec![self.statement()?]
        };
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
            return Err(self.error("Expected variable name"));
        };
        self.eat(Token::Ident(var_name.clone()))?;
    
        if self.current_token == Token::Assignment {
            self.eat(Token::Assignment)?;
            let value = self.expr()?;
            Ok(ASTNode::VarDecl(var_name, Box::new(value)))
        } else if self.current_token == Token::LBracket {
            let array_index = self.array_index(ASTNode::VarRef(var_name.clone()))?;
            if self.current_token == Token::Assignment {
                self.eat(Token::Assignment)?;
                let value = self.expr()?;
                if let ASTNode::ArrayIndex { array, index } = array_index {
                    Ok(ASTNode::ArrayAssign {
                        array,
                        index,
                        value: Box::new(value),
                    })
                } else {
                    Err(self.error("Expected array index"))
                }
            } else {
                Ok(array_index)
            }
        } else {
            Ok(ASTNode::VarRef(var_name))
        }
    }

    fn array_literal(&mut self) -> Result<ASTNode, VMError> {
        self.eat(Token::LBracket)?;
        let mut elements = Vec::new();
        
        if self.current_token != Token::RBracket {
            elements.push(self.expr()?);
            while self.current_token == Token::Comma {
                self.eat(Token::Comma)?;
                if self.current_token == Token::RBracket {
                    break; // Allow trailing comma
                }
                elements.push(self.expr()?);
            }
        }
        
        self.eat(Token::RBracket).map_err(|_| self.error("Expected closing bracket ']'"))?;
        Ok(ASTNode::Array(elements))
    }
    
    fn array_index(&mut self, array: ASTNode) -> Result<ASTNode, VMError> {
        self.eat(Token::LBracket)?;
        let index = self.expr()?;
        self.eat(Token::RBracket).map_err(|_| self.error("Expected closing bracket ']'"))?;
        
        Ok(ASTNode::ArrayIndex {
            array: Box::new(array),
            index: Box::new(index),
        })
    }

    pub fn parse_program(&mut self) -> Result<Vec<ASTNode>, VMError> {
        let mut statements = Vec::new();
        while self.current_token != Token::EOF {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }
}
