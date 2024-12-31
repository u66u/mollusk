#[derive(Debug)]
pub enum VMError {
    ParseError { message: String, line: usize, position: usize },
    TokenizationError { message: String, line: usize, position: usize },
    ExecutionError { message: String, line: usize, position: usize },
    TypeError { message: String },
    IndexError { index: i32, len: usize },
    NotAnArray,
    UndefinedVariable { name: String },
    StackUnderflow,
    DivisionByZero,
    NoScopeToEnd,
}

impl std::fmt::Display for VMError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VMError::ParseError {
                message,
                line,
                position,
            } => write!(f, "Parse error at line {}:{} - {}", line, position, message),
            VMError::TokenizationError {
                message,
                line,
                position,
            } => write!(
                f,
                "Tokenization error at line {}:{} - {}",
                line, position, message
            ),
            VMError::ExecutionError {
                message,
                line,
                position,
            } => write!(
                f,
                "Execution error at line {}:{} - {}",
                line, position, message
            ),
            VMError::TypeError { message } => write!(f, "Type error: {}", message),
            VMError::UndefinedVariable { name } => write!(f, "Undefined variable: {}", name),
            VMError::StackUnderflow => write!(f, "Stack underflow"),
            VMError::DivisionByZero => write!(f, "Division by zero"),
            VMError::IndexError { index, len } => write!(f, "Index error: index {} out of bounds for array of length {}", index, len),
            VMError::NotAnArray => write!(f, "Value is not an array"),
            VMError::NoScopeToEnd => write!(f, "No scope to end"),
        }
    }
}

impl std::error::Error for VMError {}
