use crate::error::CompilerError;
use combine::char::{alpha_num, digit, spaces, string};
use combine::{attempt, optional, choice, eof, many1, sep_end_by1, token, Parser};
use combine::stream::state::State;

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
    NotEqual, LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Assign,
    Increment,
    Decrement,
    AssignAdd,
    AssignMinus,
    AssignMultiply,
    AssignDivide,
    If,
    Else,
    Colon,
    QuestionMark,
    For,
    While,
    Do,
    Break,
    Continue,
    Comma,
}

pub fn lex(text: &str) -> Result<Vec<Token>, CompilerError> {
    let mut lexer = optional(spaces())
        .with(sep_end_by1::<Vec<_>, _, _>(
            choice((
                choice((
                    attempt(string("&&").map(|_| Token::And)),
                    attempt(string("||").map(|_| Token::Or)),
                    attempt(string("==").map(|_| Token::Equal)),
                    attempt(string("!=").map(|_| Token::NotEqual)),
                    attempt(string("<=").map(|_| Token::LessThanOrEqual)),
                    attempt(string(">=").map(|_| Token::GreaterThanOrEqual)),
                    attempt(string("int").map(|_| Token::Int)),
                    attempt(string("return").map(|_| Token::Return)),
                    attempt(string("if").map(|_| Token::If)),
                    attempt(string("else").map(|_| Token::Else)),
                    attempt(string("for").map(|_| Token::For)),
                    attempt(string("while").map(|_| Token::While)),
                    attempt(string("do").map(|_| Token::Do)),
                    attempt(string("break").map(|_| Token::Break)),
                    attempt(string("continue").map(|_| Token::Continue)),
                    attempt(string("++").map(|_| Token::Increment)),
                    attempt(string("--").map(|_| Token::Decrement)),
                    attempt(string("+=").map(|_| Token::AssignAdd)),
                    attempt(string("-=").map(|_| Token::AssignMinus)),
                    attempt(string("*=").map(|_| Token::AssignMultiply)),
                    attempt(string("/=").map(|_| Token::AssignDivide)),
                )),
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
                    token(':').map(|_| Token::Colon),
                    token(',').map(|_| Token::Comma),
                    token('?').map(|_| Token::QuestionMark),
                )),
                many1::<String, _>(digit()).map(|i| Token::Integer(i.parse().unwrap())),
                many1::<String, _>(alpha_num()).map(|id| Token::Identifier(id)),
            )),
            spaces(),
        ))
        .skip(eof());

    match lexer.easy_parse(State::new(text)) {
        Ok(tokens) => Ok(tokens.0),
        Err(e) => Err(CompilerError::Lexer(format!("{:?}", e.errors), e.position)),
    }
}
