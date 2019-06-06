use super::{Context, Generator, Identifier, Statement, Type};
use std::io::{self, Write};

#[derive(Debug)]
pub struct Function {
    name: String,
    args: Vec<(Type, Identifier)>,
    statements: Option<Vec<Statement>>,
}

impl Function {
    pub fn new(
        name: String,
        args: Vec<(Type, Identifier)>,
        statements: Option<Vec<Statement>>,
    ) -> Self {
        Self {
            name,
            statements,
            args,
        }
    }
}
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

            for s in statements.iter() {
                s.generate(stream, ctx)?;
            }

            writeln!(
                stream,
                "mov rsp, rbp\n\
                 pop rbp\n\
                 mov rax, 0\n\
                 ret"
            )?;
        } else {
        }
        Ok(())
    }
}
