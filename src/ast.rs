use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub enum ASTNode {
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
