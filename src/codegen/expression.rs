use super::{Context, Generator, Constant, CALLER_REGS};
use crate::ast::*;
use std::io::{self, Write};

impl Generator for Expression {
    fn generate(&self, stream: &mut impl Write, ctx: &mut Context) -> io::Result<()> {
        match self {
            Expression::FunCall(id, args) => {
                let mut to_restore = Vec::new();
                for (i, arg) in args.iter().take(6).enumerate() {
                    arg.generate(stream, ctx)?;
                    writeln!(
                        stream,
                        "push {}\n\
                         mov {}, rax",
                        CALLER_REGS[i], CALLER_REGS[i]
                    )?;
                    to_restore.push(CALLER_REGS[i]);
                }

                let mut stacked = 0;
                for arg in args.iter().skip(6).rev() {
                    arg.generate(stream, ctx)?;
                    writeln!(stream, "push rax")?;
                    stacked += 1;
                }
                writeln!(
                    stream,
                    "call {}\n\
                     add rsp, {}",
                    id,
                    stacked * 8 // 64 bit offsetting
                )?;

                // restore register arguments
                for reg in to_restore.iter().rev() {
                    writeln!(stream, "pop {}", reg)?;
                }
            }
            Expression::Conditional(cond, exp, alt) => {
                let alt_label = ctx.unique_label();
                let post_conditional = ctx.unique_label();

                cond.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "cmp rax, 0\n\
                     je {}",
                    alt_label
                )?;
                exp.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "jmp {}\n\
                     {}:",
                    post_conditional, alt_label
                )?;
                alt.generate(stream, ctx)?;
                writeln!(stream, "{}:", post_conditional)?;
            }
            Expression::PreIncrement(id) => {
                let var = ctx.resolve(&id);
                writeln!(
                    stream,
                    "add {}, 1\n\
                     mov rax, {}",
                    var, var
                )?;
            }
            Expression::PreDecrement(id) => {
                let var = ctx.resolve(&id);
                writeln!(
                    stream,
                    "sub {}, 1\n\
                     mov rax, {}",
                    var, var
                )?;
            }
            Expression::PostIncrement(id) => {
                let var = ctx.resolve(&id);
                writeln!(
                    stream,
                    "mov rax, {}\n\
                     add {}, 1",
                    var, var
                )?;
            }
            Expression::PostDecrement(id) => {
                let var = ctx.resolve(&id);
                writeln!(
                    stream,
                    "mov rax, {}\n\
                     sub {}, 1",
                    var, var
                )?;
            }
            Expression::Identifier(id) => {
                writeln!(stream, "mov rax, {}", ctx.resolve(&id))?;
            }
            Expression::Assignment(id, e) => {
                e.generate(stream, ctx)?;
                writeln!(stream, "mov {}, rax", ctx.resolve(&id))?;
            }
            Expression::Literal(i) => {
                writeln!(stream, "mov rax, {}", i)?;
            }
            Expression::Minus(e) => {
                e.generate(stream, ctx)?;
                writeln!(stream, "neg rax")?;
            }
            Expression::BinaryNot(e) => {
                e.generate(stream, ctx)?;
                writeln!(stream, "not rax")?;
            }
            Expression::LogicalNot(e) => {
                e.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "cmp rax, 0\n\
                     mov rax, 0\n\
                     sete al"
                )?;
            }
            Expression::Subtract(e1, e2) => {
                e2.generate(stream, ctx)?;
                writeln!(stream, "push rax")?;
                e1.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "pop rcx\n\
                     sub rax, rcx"
                )?;
            }
            Expression::Add(e1, e2) => {
                e1.generate(stream, ctx)?;
                writeln!(stream, "push rax")?;
                e2.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "pop rcx\n\
                     add rax, rcx"
                )?;
            }
            Expression::Multiply(e1, e2) => {
                e1.generate(stream, ctx)?;
                writeln!(stream, "push rax")?;
                e2.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "pop rcx\n\
                     imul rax, rcx"
                )?;
            }
            Expression::Divide(e1, e2) => {
                e1.generate(stream, ctx)?;
                writeln!(stream, "push rax")?;
                e2.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "mov rdx, 0\n\
                     mov rcx, rax\n\
                     pop rax\n\
                     idiv rcx"
                )?;
            }
            Expression::And(e1, e2) => {
                let end = ctx.unique_label();
                let second_clause = ctx.unique_label();
                e1.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "cmp rax, 0\n\
                     jne {}\n\
                     jmp {}\n\
                     {}:",
                    second_clause, end, second_clause
                )?;
                e2.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "cmp rax, 0\n\
                     mov rax, 0\n\
                     setne al\n\
                     {}:",
                    end
                )?;
            }
            Expression::Or(e1, e2) => {
                let end = ctx.unique_label();
                let second_clause = ctx.unique_label();
                e1.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "cmp rax, 0\n\
                     je {}\n\
                     mov rax, 1\n\
                     jmp {}\n\
                     {}:",
                    second_clause, end, second_clause
                )?;
                e2.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "cmp rax, 0\n\
                     mov rax, 0\n\
                     setne al\n\
                     {}:",
                    end
                )?;
            }
            Expression::Equal(e1, e2) => {
                e1.generate(stream, ctx)?;
                writeln!(stream, "push rax")?;
                e2.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "pop rcx\n\
                     cmp rcx, rax\n\
                     mov rax, 0\n\
                     sete al"
                )?;
            }
            Expression::NotEqual(e1, e2) => {
                e1.generate(stream, ctx)?;
                writeln!(stream, "push rax")?;
                e2.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "pop rcx\n\
                     cmp rcx, rax\n\
                     mov rax, 0\n\
                     setne al"
                )?;
            }
            Expression::LessThan(e1, e2) => {
                e1.generate(stream, ctx)?;
                writeln!(stream, "push rax")?;
                e2.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "pop rcx\n\
                     cmp rcx, rax\n\
                     mov rax, 0\n\
                     setl al"
                )?;
            }
            Expression::LessThanOrEqual(e1, e2) => {
                e1.generate(stream, ctx)?;
                writeln!(stream, "push rax")?;
                e2.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "pop rcx\n\
                     cmp rcx, rax\n\
                     mov rax, 0\n\
                     setle al"
                )?;
            }
            Expression::GreaterThan(e1, e2) => {
                e1.generate(stream, ctx)?;
                writeln!(stream, "push rax")?;
                e2.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "pop rcx\n\
                     cmp rcx, rax\n\
                     mov rax, 0\n\
                     setg al"
                )?;
            }
            Expression::GreaterThanOrEqual(e1, e2) => {
                e1.generate(stream, ctx)?;
                writeln!(stream, "push rax")?;
                e2.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "pop rcx\n\
                     cmp rcx, rax\n\
                     mov rax, 0\n\
                     setge al"
                )?;
            }
        }
        Ok(())
    }
}

impl Constant for Expression {
    fn eval<T>(&self) -> T {
    }
}
