use super::Generator;
use super::Context;
use crate::ast::*;
use std::io::{self, Write};

impl Generator for Statement {
    fn generate(&self, stream: &mut impl Write, ctx: &mut Context) -> io::Result<()> {
        match self {
            Statement::Continue => {
                writeln!(
                    stream,
                    "jmp {}",
                    ctx.outer_loop().expect("No outer loop context").0
                )?; // FIXME: doesn't always jump over body
            }
            Statement::Break => {
                writeln!(
                    stream,
                    "jmp {}",
                    ctx.outer_loop().expect("No outer loop context").1
                )?;
            }
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
                body.generate(stream, &mut ctx.inner_loop(beg.clone(), end.clone()))?;
                writeln!(
                    stream,
                    "jmp {}\n\
                     {}:",
                    beg, end
                )?;
            }
            Statement::Do(body, cond) => {
                let beg = ctx.unique_label();
                let end = ctx.unique_label();

                writeln!(stream, "{}:", beg)?;
                body.generate(stream, &mut ctx.inner_loop(beg.clone(), end.clone()))?;
                cond.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "cmp rax, 0\n\
                     jne {}`\n\
                     {}:",
                    beg, end
                )?;
            }
            Statement::For(init, cond, iter, body) => {
                let beg = ctx.unique_label();
                let cont = ctx.unique_label();
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
                body.generate(stream, &mut ctx.inner_loop(cont.clone(), end.clone()))?;

                writeln!(stream, "{}:", cont)?;
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
                let cont = ctx.unique_label();
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
                body.generate(stream, &mut ctx.inner_loop(cont.clone(), end.clone()))?;

                writeln!(stream, "{}:", cont)?;
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
                    writeln!(stream, "sub rsp, 8")?;
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
