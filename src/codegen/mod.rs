mod expression;
mod function;
mod statement;
mod context;

pub use self::expression::Expression;
pub use self::function::Function;
pub use self::statement::Statement;

use self::context::Context;
use std::io::{self, Write};

pub fn codegen(program: &[Function], stream: &mut impl Write) -> io::Result<()> {
    writeln!(stream, ".intel_syntax noprefix")?;

    for function in program {
        function.generate(stream, &mut Context::empty())?;
    }

    Ok(())
}

trait Generator {
    fn generate(&self, stream: &mut impl Write, ctx: &mut Context) -> io::Result<()>;
}


#[derive(Debug, Clone)]
pub enum Type {
    Int,
}

pub type Identifier = String;
