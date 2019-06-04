use super::{Context, Generator, Statement};
use std::io::{self, Write};

#[derive(Debug)]
pub struct Function {
    name: String,
    statements: Vec<Statement>,
}

impl Function {
    pub fn new(name: String, statements: Vec<Statement>) -> Self {
        Self { name, statements }
    }
}
impl Generator for Function {
    fn generate(
        &self,
        stream: &mut impl Write,
        ctx: &mut Context,
    ) -> io::Result<()> {
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

        for s in self.statements.iter() {
            s.generate(stream, ctx)?;
        }

        writeln!(
            stream,
            "mov rsp, rbp\n\
             pop rbp\n\
             mov rax, 0\n\
             ret"
        )?;
        Ok(())
    }
}
