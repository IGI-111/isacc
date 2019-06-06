use crate::codegen::*;
use crate::error::CompilerError;
use crate::lexing::*;
use combine::{
    attempt, between, choice, many, many1, optional, satisfy, sep_by, token, ParseError, Parser,
    Stream,
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
    typename()
        .with(identifier())
        .and(between(
            token(Token::OpenParen),
            token(Token::CloseParen),
            sep_by::<Vec<_>, _, _>(typename().and(identifier()), token(Token::Comma)),
        ))
        .and(choice((
            between(
                token(Token::OpenBrace),
                token(Token::CloseBrace),
                many::<Vec<_>, _>(block_item()),
            ).map(|args| Some(args)) ,
            token(Token::Semicolon).map(|_| None),
        )))
        .map(|((name, args), statements)| Function::new(name, args, statements))
}

fn block_item<I>() -> impl Parser<Input = I, Output = Statement>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((declaration(), statement()))
}

fn declaration<I>() -> impl Parser<Input = I, Output = Statement>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    typename()
        .and(identifier())
        .and(optional(token(Token::Assign).with(expression())))
        .skip(token(Token::Semicolon))
        .map(|((t, id), expr)| Statement::Declaration(t, id, expr))
}

parser! { fn statement[I]()(I) -> Statement where [I: Stream<Item = Token>] { statement_() }}
fn statement_<I>() -> impl Parser<Input = I, Output = Statement>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let return_statement = token(Token::Return)
        .with(expression())
        .skip(token(Token::Semicolon))
        .map(|e| Statement::Return(e));

    let expression_statement = optional(expression())
        .map(|e| Statement::Expression(e))
        .skip(token(Token::Semicolon));

    let if_statement = token(Token::If)
        .with(between(
            token(Token::OpenParen),
            token(Token::CloseParen),
            expression(),
        ))
        .and(statement())
        .and(optional(token(Token::Else).with(statement())))
        .map(|((cond, stm), alt)| Statement::If(cond, Box::new(stm), alt.map(Box::new)));

    let compound_statement = between(
        token(Token::OpenBrace),
        token(Token::CloseBrace),
        many::<Vec<_>, _>(block_item()),
    )
    .map(|stms| Statement::Compound(stms));

    let for_statement = token(Token::For)
        .with(between(
            token(Token::OpenParen),
            token(Token::CloseParen),
            optional(expression())
                .skip(token(Token::Semicolon))
                .and(optional(expression()))
                .skip(token(Token::Semicolon))
                .and(optional(expression())),
        ))
        .and(statement())
        .map(|(((init, cond), iter), body)| {
            Statement::For(
                init,
                cond.unwrap_or(Expression::Literal(1)),
                iter,
                Box::new(body),
            )
        });

    let for_decl_statement = token(Token::For)
        .with(between(
            token(Token::OpenParen),
            token(Token::CloseParen),
            declaration()
                .and(optional(expression()))
                .skip(token(Token::Semicolon))
                .and(optional(expression())),
        ))
        .and(statement())
        .map(|(((init, cond), iter), body)| {
            Statement::ForDecl(
                Box::new(init),
                cond.unwrap_or(Expression::Literal(1)),
                iter,
                Box::new(body),
            )
        });

    let while_statement = token(Token::While)
        .with(between(
            token(Token::OpenParen),
            token(Token::CloseParen),
            expression(),
        ))
        .and(statement())
        .map(|(cond, body)| Statement::While(cond, Box::new(body)));

    let do_statement = token(Token::Do)
        .with(statement())
        .skip(token(Token::While))
        .and(between(
            token(Token::OpenParen),
            token(Token::CloseParen),
            expression(),
        ))
        .skip(token(Token::Semicolon))
        .map(|(body, cond)| Statement::Do(Box::new(body), cond));

    let break_statement = token(Token::Break)
        .skip(token(Token::Semicolon))
        .map(|_| Statement::Break);

    let continue_statement = token(Token::Continue)
        .skip(token(Token::Semicolon))
        .map(|_| Statement::Continue);

    choice((
        compound_statement,
        return_statement,
        if_statement,
        attempt(for_decl_statement),
        for_statement,
        while_statement,
        do_statement,
        expression_statement,
        break_statement,
        continue_statement,
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
        _ => unreachable!(),
    });

    let unary_lvalue_pre = choice((token(Token::Increment), token(Token::Decrement)))
        .and(identifier())
        .map(|(op, id)| match op {
            Token::Increment => Expression::PreIncrement(id),
            Token::Decrement => Expression::PreDecrement(id),
            _ => unreachable!(),
        });

    let unary_lvalue_post = identifier()
        .and(choice((token(Token::Increment), token(Token::Decrement))))
        .map(|(id, op)| match op {
            Token::Increment => Expression::PostIncrement(id),
            Token::Decrement => Expression::PostDecrement(id),
            _ => unreachable!(),
        });

    choice((
        attempt(unary_lvalue_pre),
        attempt(unary_lvalue_post),
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
                    _ => unreachable!(),
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

parser! { fn conditional_exp[I]()(I) -> Expression where [I: Stream<Item = Token>] { conditional_exp_() }}
fn conditional_exp_<I>() -> impl Parser<Input = I, Output = Expression>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    logical_or_exp()
        .and(optional(
            token(Token::QuestionMark)
                .with(expression())
                .skip(token(Token::Colon))
                .and(conditional_exp()),
        ))
        .map(|(cond, remainder)| match remainder {
            None => cond,
            Some((exp, alt)) => {
                Expression::Conditional(Box::new(cond), Box::new(exp), Box::new(alt))
            }
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
                .and(choice((
                    token(Token::Assign),
                    token(Token::AssignAdd),
                    token(Token::AssignMinus),
                    token(Token::AssignMultiply),
                    token(Token::AssignDivide),
                )))
                .and(expression())
                .map(|((id, op), expr)| match op {
                    Token::Assign => Expression::Assignment(id, Box::new(expr)),
                    Token::AssignAdd => Expression::Assignment(
                        id.clone(),
                        Box::new(Expression::Add(
                            Box::new(Expression::Identifier(id)),
                            Box::new(expr),
                        )),
                    ),
                    Token::AssignMinus => Expression::Assignment(
                        id.clone(),
                        Box::new(Expression::Subtract(
                            Box::new(Expression::Identifier(id)),
                            Box::new(expr),
                        )),
                    ),
                    Token::AssignMultiply => Expression::Assignment(
                        id.clone(),
                        Box::new(Expression::Multiply(
                            Box::new(Expression::Identifier(id)),
                            Box::new(expr),
                        )),
                    ),
                    Token::AssignDivide => Expression::Assignment(
                        id.clone(),
                        Box::new(Expression::Divide(
                            Box::new(Expression::Identifier(id)),
                            Box::new(expr),
                        )),
                    ),
                    _ => unreachable!(),
                }),
        ),
        conditional_exp(),
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
