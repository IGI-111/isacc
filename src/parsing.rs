use crate::codegen::*;
use combine::char::{alpha_num, digit, spaces, string};
use combine::easy::Errors;
use combine::stream::PointerOffset;
use combine::{
    any, between, choice, eof, many, many1, sep_end_by1, token, ParseError, Parser, Stream,
};

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
            token('+').map(|_| Token::Add),
            token('*').map(|_| Token::Multiply),
            token('/').map(|_| Token::Divide),
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

fn factor<I>() -> impl Parser<Input = I, Output = Expression>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let literal = any().map(|t| match t {
        Token::Integer(i) => Expression::Literal(i),
        _ => panic!("Invalid expression"),
    });

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
        _ => panic!("Invalid unary operator"),
    });

    choice((unary_op, literal))
}

fn term<I>() -> impl Parser<Input = I, Output = Expression>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    factor()
        .and(many::<Vec<_>, _>(
            choice((token(Token::Multiply), token(Token::Divide))).and(factor()),
        ))
        .map(|(first, remainder)| {
            remainder
                .into_iter()
                .fold(first, |prev, (op, next)| match op {
                    Token::Multiply => Expression::Multiply(Box::new(prev), Box::new(next)),
                    Token::Divide => Expression::Divide(Box::new(prev), Box::new(next)),
                    _ => panic!("Invalid binary operator"),
                })
        })
}

parser! { fn expression[I]()(I) -> Expression where [I: Stream<Item = Token>] { expression_() }}
fn expression_<I>() -> impl Parser<Input = I, Output = Expression>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    term()
        .and(many::<Vec<_>, _>(
            choice((token(Token::Add), token(Token::Minus))).and(term()),
        ))
        .map(|(first, remainder)| {
            remainder
                .into_iter()
                .fold(first, |prev, (op, next)| match op {
                    Token::Add => Expression::Add(Box::new(prev), Box::new(next)),
                    Token::Minus => Expression::Subtract(Box::new(prev), Box::new(next)),
                    _ => panic!("Invalid binary operator"),
                })
        })
}
