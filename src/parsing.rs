use crate::codegen::*;
use crate::error::CompilerError;
use crate::lexing::*;
use combine::{
    attempt,
    between, choice, many, many1, optional, satisfy, sep_end_by1, token, ParseError, Parser, Stream,
};

pub fn parse(tokens: &[Token]) -> Result<Vec<Function>, CompilerError> {
    let mut program = many1::<Vec<_>, _>(function());

    match program.easy_parse(tokens) {
        Ok(ast) => Ok(ast.0),
        Err(e) => Err(CompilerError::ParseError(format!("{:?}", e))),
    }
}

fn function<I>() -> impl Parser<Input = I, Output = Function>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    token(Token::Int)
        .with(identifier())
        .skip(token(Token::OpenParen))
        .skip(token(Token::CloseParen))
        .and(between(
            token(Token::OpenBrace),
            token(Token::CloseBrace),
            sep_end_by1::<Vec<_>, _, _>(statement(), token(Token::Semicolon)),
        ))
        .map(|(name, statements)| Function::new(name, statements))
}

fn statement<I>() -> impl Parser<Input = I, Output = Statement>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let return_statement = token(Token::Return)
        .with(expression())
        .map(|e| Statement::Return(e));

    let expression_statement = expression().map(|e| Statement::Expression(e));

    let declaration_statement = typename()
        .and(identifier())
        .and(optional(token(Token::Assign).with(expression())))
        .map(|((t, id), expr)| Statement::Declaration(t, id, expr));

    choice((
        return_statement,
        expression_statement,
        declaration_statement,
    ))
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

    choice((
        unary_op,
        literal(),
        between(
            token(Token::OpenParen),
            token(Token::CloseParen),
            expression(),
        ),
        identifier().map(|id| Expression::Identifier(id)),
    ))
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

fn additive_exp<I>() -> impl Parser<Input = I, Output = Expression>
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
                    _ => unreachable!(),
                })
        })
}

fn relational_exp<I>() -> impl Parser<Input = I, Output = Expression>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    additive_exp()
        .and(many::<Vec<_>, _>(
            choice((
                token(Token::LessThan),
                token(Token::LessThanOrEqual),
                token(Token::GreaterThan),
                token(Token::GreaterThanOrEqual),
            ))
            .and(additive_exp()),
        ))
        .map(|(first, remainder)| {
            remainder
                .into_iter()
                .fold(first, |prev, (op, next)| match op {
                    Token::LessThan => Expression::LessThan(Box::new(prev), Box::new(next)),
                    Token::LessThanOrEqual => {
                        Expression::LessThanOrEqual(Box::new(prev), Box::new(next))
                    }
                    Token::GreaterThan => Expression::GreaterThan(Box::new(prev), Box::new(next)),
                    Token::GreaterThanOrEqual => {
                        Expression::GreaterThanOrEqual(Box::new(prev), Box::new(next))
                    }
                    _ => unreachable!(),
                })
        })
}

fn equality_exp<I>() -> impl Parser<Input = I, Output = Expression>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    relational_exp()
        .and(many::<Vec<_>, _>(
            choice((token(Token::Equal), token(Token::NotEqual))).and(relational_exp()),
        ))
        .map(|(first, remainder)| {
            remainder
                .into_iter()
                .fold(first, |prev, (op, next)| match op {
                    Token::Equal => Expression::Equal(Box::new(prev), Box::new(next)),
                    Token::NotEqual => Expression::NotEqual(Box::new(prev), Box::new(next)),
                    _ => unreachable!(),
                })
        })
}

fn logical_and_exp<I>() -> impl Parser<Input = I, Output = Expression>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    equality_exp()
        .and(many::<Vec<_>, _>(token(Token::And).and(equality_exp())))
        .map(|(first, remainder)| {
            remainder
                .into_iter()
                .fold(first, |prev, (op, next)| match op {
                    Token::And => Expression::And(Box::new(prev), Box::new(next)),
                    _ => unreachable!(),
                })
        })
}

fn logical_or_exp<I>() -> impl Parser<Input = I, Output = Expression>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    logical_and_exp()
        .and(many::<Vec<_>, _>(token(Token::Or).and(logical_and_exp())))
        .map(|(first, remainder)| {
            remainder
                .into_iter()
                .fold(first, |prev, (op, next)| match op {
                    Token::Or => Expression::Or(Box::new(prev), Box::new(next)),
                    _ => unreachable!(),
                })
        })
}

parser! { fn expression[I]()(I) -> Expression where [I: Stream<Item = Token>] { expression_() }}
fn expression_<I>() -> impl Parser<Input = I, Output = Expression>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(
            identifier()
                .skip(token(Token::Assign))
                .and(expression())
                .map(|(id, expr)| Expression::Assignment(id, Box::new(expr))),
        ),
        logical_or_exp(),
    ))
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

fn identifier<I>() -> impl Parser<Input = I, Output = Identifier>
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

fn typename<I>() -> impl Parser<Input = I, Output = Type>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(|t| match t {
        Token::Int => true,
        _ => false,
    })
    .map(|t| match t {
        Token::Int => Type::Int,
        _ => unreachable!(),
    })
}
