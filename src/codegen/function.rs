use super::Context;
use super::Generator;
use crate::ast::*;
use std::io::{self, Write};

impl Generator for Function {
    fn generate(&self, stream: &mut impl Write, ctx: &mut Context) -> io::Result<()> {
        if let Some(statements) = &self.statements {
            if self.name == "main" {
                writeln!(
                    stream,
                    ".globl main\n\
                     .globl _main\n\
                     main:\n\
                     _main:"
                )?;
            } else {
                writeln!(
                    stream,
                    ".globl {}\n\
                     {}:",
                    self.name, self.name
                )?;
            }

            writeln!(
                stream,
                "push rbp\n\
                 mov rbp, rsp"
            )?;

            let mut fun_ctx = ctx.function_scope(&self);
            for s in statements.iter() {
                s.generate(stream, &mut fun_ctx)?;
            }

            writeln!(
                stream,
                "mov rsp, rbp\n\
                 pop rbp\n\
                 mov rax, 0\n\
                 ret"
            )?;
        }
        Ok(())
    }
}
