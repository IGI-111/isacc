mod context;
mod expression;
mod function;
mod statement;

use crate::ast::Program;
use self::context::Context;
use std::io::{self, Write};

pub fn codegen(program: &Program, stream: &mut impl Write) -> io::Result<()> {
    program.generate(stream, &mut Context::empty())
}

trait Generator {
    fn generate(&self, stream: &mut impl Write, ctx: &mut Context) -> io::Result<()>;
}

impl Generator for Program {
    fn generate(&self, stream: &mut impl Write, ctx: &mut Context) -> io::Result<()> {
        writeln!(stream, ".intel_syntax noprefix")?;
        for function in self.funs.iter() {
            function.generate(stream, ctx)?;
        }
        Ok(())
    }
}

const CALLER_REGS: [&str; 6] = [ "rdi", "rsi", "rdx", "rcx", "r8", "r9"];
