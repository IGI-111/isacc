use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CompilerError {
    ParseError(String),
    LexError(String),
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompilerError::ParseError(e) => write!(f, "Parse Error: {}", e),
            CompilerError::LexError(e) => write!(f, "Lexing Error: {}", e),
        }
    }
}

impl Error for CompilerError {}
