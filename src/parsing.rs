use crate::codegen::*;
use crate::error::CompilerError;
use crate::lexing::*;
use combine::{
    between, choice, many, many1, satisfy, sep_end_by1, token, ParseError, Parser, Stream,
};

pub fn parse(tokens: &[Token]) -> Result<Vec<Function>, CompilerError> {
    let statement = token(Token::Return)
        .with(expression())
        .map(|e| Statement::Return(e));

    let function = token(Token::Int)
        .with(identifier())
        .skip(token(Token::OpenParen))
        .skip(token(Token::CloseParen))
        .and(between(
            token(Token::OpenBrace),
            token(Token::CloseBrace),
            sep_end_by1::<Vec<_>, _, _>(statement, token(Token::Semicolon)),
        ))
        .map(|(name, statements)| Function::new(name, statements));

    let mut program = many1::<Vec<_>, _>(function);

    match program.easy_parse(tokens) {
        Ok(ast) => Ok(ast.0),
        Err(e) => Err(CompilerError::ParseError(format!("{:?}", e))),
    }
}

fn factor<I>() -> impl Parser<Input = I, Output = Expression>
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
        _ => panic!("Invalid unary operator"),
    });

    choice((unary_op, literal()))
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

fn literal<I>() -> impl Parser<Input = I, Output = Expression>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(|t| match t {
        Token::Integer(_) => true,
        _ => false,
    })
    .map(|t| match t {
        Token::Integer(i) => Expression::Literal(i),
        _ => unreachable!(),
    })
}

fn identifier<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(|t| match t {
        Token::Identifier(_) => true,
        _ => false,
    })
    .map(|t| match t {
        Token::Identifier(i) => i,
        _ => unreachable!(),
    })
}
