use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CompilerError {
    Parser(String),
    Lexer(String),
    Validation(String),
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompilerError::Parser(e) => write!(f, "Parser Error: {}", e),
            CompilerError::Lexer(e) => write!(f, "Lexer Error: {}", e),
            CompilerError::Validation(e) => write!(f, "Validation Error: {}", e),
        }
    }
}

impl Error for CompilerError {}
