use std::error::Error;
use std::fmt;
use combine::stream::state::SourcePosition;

#[derive(Debug)]
pub enum CompilerError {
    Parser(String, usize),
    Lexer(String, SourcePosition),
    Validation(String),
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompilerError::Parser(e, pos) => write!(f, "Parser Error line {}: {}", pos, e),
            CompilerError::Lexer(e, pos) => write!(f, "Lexer Error position {}: {}", pos.line, e),
            CompilerError::Validation(e) => write!(f, "Validation Error: {}", e),
        }
    }
}

impl Error for CompilerError {}
