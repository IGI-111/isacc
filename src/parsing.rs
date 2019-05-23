use crate::codegen::*;
use combine::char::{alpha_num, digit, spaces, string};
use combine::easy::Errors;
use combine::stream::PointerOffset;
use combine::{any, between, choice, eof, many1, sep_end_by1, token, Parser, Stream, ParseError};

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
}

pub fn lex(text: &str) -> Result<Vec<Token>, Errors<char, &str, PointerOffset>> {
    let mut lexer = sep_end_by1::<Vec<_>, _, _>(
        choice((
            token('{').map(|_| Token::OpenBrace),
            token('}').map(|_| Token::CloseBrace),
            token('(').map(|_| Token::OpenParen),
            token(')').map(|_| Token::CloseParen),
            token(';').map(|_| Token::Semicolon),
            token('-').map(|_| Token::Minus),
            token('!').map(|_| Token::LogicalNot),
            token('~').map(|_| Token::BinaryNot),
            string("int").map(|_| Token::Int),
            string("return").map(|_| Token::Return),
            many1::<String, _>(digit()).map(|i| Token::Integer(i.parse().unwrap())),
            many1::<String, _>(alpha_num()).map(|id| Token::Identifier(id)),
        )),
        spaces(),
    )
    .skip(eof());
    let tokens = lexer.easy_parse(text)?.0;
    Ok(tokens)
}

pub fn parse(
    tokens: &[Token],
) -> Result<Vec<Function>, Errors<Token, &[Token], combine::stream::PointerOffset>> {
    let statement = token(Token::Return)
        .with(expression())
        .map(|e| Statement::Return(e));

    let function = token(Token::Int)
        .with(any())
        .skip(token(Token::OpenParen))
        .skip(token(Token::CloseParen))
        .and(between(
            token(Token::OpenBrace),
            token(Token::CloseBrace),
            sep_end_by1::<Vec<_>, _, _>(statement, token(Token::Semicolon)),
        ))
        .map(|(id, statements)| match id {
            Token::Identifier(name) => Function::new(name, statements),
            _ => panic!("Invalid identifier"),
        });

    let mut program = many1::<Vec<_>, _>(function);

    let ast = program.easy_parse(tokens)?;
    Ok(ast.0)
}

parser! { fn expression[I]()(I) -> Expression where [I: Stream<Item = Token>] { expression_() }}
fn expression_<I>() -> impl Parser<Input = I, Output = Expression>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let unary_op = choice((
        token(Token::Minus),
        token(Token::BinaryNot),
        token(Token::LogicalNot),
    ))
    .and(expression())
    .map(|(op, e)| match op {
        Token::Minus => Expression::Minus(Box::new(e)),
        Token::BinaryNot => Expression::BinaryNot(Box::new(e)),
        Token::LogicalNot => Expression::LogicalNot(Box::new(e)),
        _ => panic!("Invalid operator"),
    });

    let literal = any().map(|t| match t {
        Token::Integer(i) => Expression::Literal(i),
        _ => panic!("Invalid expression"),
    });

    choice((unary_op, literal))
}
