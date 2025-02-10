use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum VMError {
    #[error("Parse error: {message}")]
    #[diagnostic(code(vm::parse_error))]
    ParseError {
        #[source_code]
        src: String,
        message: String,
        #[label("here")]
        span: SourceSpan,
    },

    #[error("Tokenization error: {message}")]
    #[diagnostic(code(vm::tokenization_error))]
    TokenizationError {
        #[source_code]
        src: String,
        message: String,
        #[label("here")]
        span: SourceSpan,
    },

    #[error("Execution error: {message}")]
    #[diagnostic(code(vm::execution_error))]
    ExecutionError {
        message: String,
        line: usize,
        position: usize,
    },

    #[error("Type error: {message}")]
    TypeError {
        message: String,
    },

    #[error("Index {index} out of bounds for array of length {len}")]
    IndexError {
        index: i32,
        len: usize,
    },

    #[error("Value is not an array")]
    NotAnArray,

    #[error("Undefined variable: {name}")]
    UndefinedVariable {
        name: String,
    },

    #[error("Stack underflow")]
    StackUnderflow,

    #[error("Division by zero")]
    DivisionByZero,

    #[error("No scope to end")]
    NoScopeToEnd,

    #[error("Stack overflow")]
    StackOverflow,

    #[error("Invalid jump destination: {target} (max: {max})")]
    InvalidJump {
        target: usize,
        max: usize,
    },
}

impl VMError {
    pub fn tokenization_error(src: String, message: String, pos: usize, len: usize) -> Self {
        VMError::TokenizationError {
            src,
            message,
            span: (pos, len).into(),
        }
    }

    pub fn parse_error(src: String, message: String, pos: usize, len: usize) -> Self {
        VMError::ParseError {
            src,
            message,
            span: (pos, len).into(),
        }
    }
}
