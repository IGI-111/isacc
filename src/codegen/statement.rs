use super::{Context, Expression, Generator, Identifier, Type};
use std::io::{self, Write};

#[derive(Debug)]
pub enum Statement {
    Declaration(Type, Identifier, Option<Expression>),
    Return(Expression),
    Expression(Option<Expression>),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    Compound(Vec<Statement>),
    For(
        Option<Expression>,
        Expression,
        Option<Expression>,
        Box<Statement>,
    ),
    ForDecl(
        Box<Statement>,
        Expression,
        Option<Expression>,
        Box<Statement>,
    ),
    While(Expression, Box<Statement>),
    Do(Box<Statement>, Expression),
    Break,
    Continue,
}

impl Generator for Statement {
    fn generate(&self, stream: &mut impl Write, ctx: &mut Context) -> io::Result<()> {
        match self {
            Statement::Break => {}
            Statement::Continue => {}
            Statement::While(cond, body) => {
                let beg = ctx.unique_label();
                let end = ctx.unique_label();

                writeln!(stream, "{}:", beg)?;
                cond.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "cmp rax, 0\n\
                     je {}",
                    end
                )?;
                body.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "jmp {}\n\
                     {}:",
                    beg, end
                )?;
            }
            Statement::Do(body, cond) => {
                let beg = ctx.unique_label();

                writeln!(stream, "{}:", beg)?;
                body.generate(stream, ctx)?;
                cond.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "cmp rax, 0\n\
                     jne {}",
                    beg
                )?;
            }
            Statement::For(init, cond, iter, body) => {
                let beg = ctx.unique_label();
                let end = ctx.unique_label();

                if let Some(init) = init {
                    init.generate(stream, ctx)?;
                }
                writeln!(stream, "{}:", beg)?;
                cond.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "cmp rax, 0\n\
                     je {}",
                    end
                )?;
                body.generate(stream, ctx)?;
                if let Some(iter) = iter {
                    iter.generate(stream, ctx)?;
                }
                writeln!(
                    stream,
                    "jmp {}\n\
                     {}:",
                    beg, end
                )?;
            }
            Statement::ForDecl(init, cond, iter, body) => {
                let beg = ctx.unique_label();
                let end = ctx.unique_label();

                init.generate(stream, ctx)?;
                writeln!(stream, "{}:", beg)?;
                cond.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "cmp rax, 0\n\
                     je {}",
                    end
                )?;
                body.generate(stream, ctx)?;
                if let Some(iter) = iter {
                    iter.generate(stream, ctx)?;
                }
                writeln!(
                    stream,
                    "jmp {}\n\
                     {}:",
                    beg, end
                )?;
            }
            Statement::Compound(stms) => {
                for stm in stms {
                    stm.generate(stream, &mut ctx.inner_scope())?;
                }
            }
            Statement::If(cond, stm, alt) => match alt {
                Some(alt) => {
                    let alt_label = ctx.unique_label();
                    let post_conditional = ctx.unique_label();

                    cond.generate(stream, ctx)?;
                    writeln!(
                        stream,
                        "cmp rax, 0\n\
                         je {}",
                        alt_label
                    )?;
                    stm.generate(stream, ctx)?;
                    writeln!(
                        stream,
                        "jmp {}\n\
                         {}:",
                        post_conditional, alt_label
                    )?;
                    alt.generate(stream, ctx)?;
                    writeln!(stream, "{}:", post_conditional)?;
                }
                None => {
                    let post_conditional = ctx.unique_label();

                    cond.generate(stream, ctx)?;
                    writeln!(
                        stream,
                        "cmp rax, 0\n\
                         je {}",
                        post_conditional
                    )?;
                    stm.generate(stream, ctx)?;
                    writeln!(stream, "{}:", post_conditional)?;
                }
            },
            Statement::Declaration(t, id, expr) => {
                ctx.declare(id.clone(), (*t).clone());
                if let Some(e) = expr {
                    e.generate(stream, ctx)?;
                    writeln!(stream, "mov [rbp{}], rax", ctx.offset_of(&id))?;
                }
            }
            Statement::Expression(e) => {
                if let Some(e) = e {
                    e.generate(stream, ctx)?;
                }
            }
            Statement::Return(e) => {
                e.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "mov rsp, rbp\n\
                     pop rbp\n\
                     ret"
                )?;
            }
        }
        Ok(())
    }
}
