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
}

pub fn lex(text: &str) -> Result<Vec<Token>, CompilerError> {
    let mut lexer = sep_end_by1::<Vec<_>, _, _>(
        choice((
            attempt(string("&&")).map(|_| Token::And),
            attempt(string("||")).map(|_| Token::Or),
            attempt(string("==")).map(|_| Token::Equal),
            attempt(string("!=")).map(|_| Token::NotEqual),
            attempt(string("<=")).map(|_| Token::LessThanOrEqual),
            attempt(string(">=")).map(|_| Token::GreaterThanOrEqual),
            attempt(string("int")).map(|_| Token::Int),
            attempt(string("return")).map(|_| Token::Return),
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
