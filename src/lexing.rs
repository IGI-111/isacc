use crate::error::CompilerError;
use combine::char::{alpha_num, digit, spaces, string};
use combine::{attempt, choice, eof, many1, sep_end_by1, token, Parser};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Semicolon,
    Int,
    Return,
    Identifier(String),
    Integer(usize),
    Minus,
    BinaryNot,
    LogicalNot,
    Add,
    Multiply,
    Divide,
    And,
    Or,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Assign,
    Increment,
    Decrement,
}

pub fn lex(text: &str) -> Result<Vec<Token>, CompilerError> {
    let mut lexer = sep_end_by1::<Vec<_>, _, _>(
        choice((
            attempt(choice((
                string("&&").map(|_| Token::And),
                string("||").map(|_| Token::Or),
                string("==").map(|_| Token::Equal),
                string("!=").map(|_| Token::NotEqual),
                string("<=").map(|_| Token::LessThanOrEqual),
                string(">=").map(|_| Token::GreaterThanOrEqual),
                string("int").map(|_| Token::Int),
                string("return").map(|_| Token::Return),
                string("++").map(|_| Token::Increment),
                string("--").map(|_| Token::Decrement),
            ))),
            choice((
                token('<').map(|_| Token::LessThan),
                token('>').map(|_| Token::GreaterThan),
                token('{').map(|_| Token::OpenBrace),
                token('}').map(|_| Token::CloseBrace),
                token('(').map(|_| Token::OpenParen),
                token(')').map(|_| Token::CloseParen),
                token(';').map(|_| Token::Semicolon),
                token('-').map(|_| Token::Minus),
                token('!').map(|_| Token::LogicalNot),
                token('~').map(|_| Token::BinaryNot),
                token('+').map(|_| Token::Add),
                token('*').map(|_| Token::Multiply),
                token('/').map(|_| Token::Divide),
                token('=').map(|_| Token::Assign),
            )),
            many1::<String, _>(digit()).map(|i| Token::Integer(i.parse().unwrap())),
            many1::<String, _>(alpha_num()).map(|id| Token::Identifier(id)),
        )),
        spaces(),
    )
    .skip(eof());

    match lexer.easy_parse(text) {
        Ok(tokens) => Ok(tokens.0),
        Err(e) => Err(CompilerError::LexError(format!("{:?}", e))),
    }
}
