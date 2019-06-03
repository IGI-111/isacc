mod expression;
mod label;
mod variable;

pub use self::expression::Expression;
use self::label::LabelGenerator;
use self::variable::VariableMap;
use std::io::{self, Write};

pub fn codegen(program: &[Function], stream: &mut impl Write) -> io::Result<()> {
    writeln!(stream, ".intel_syntax noprefix")?;

    let mut labels = LabelGenerator::new();
    let mut vars = VariableMap::empty();
    for function in program {
        function.generate(stream, &mut labels, &mut vars)?;
    }

    Ok(())
}

#[derive(Debug)]
pub struct Function {
    name: String,
    statements: Vec<Statement>,
}

impl Function {
    pub fn new(name: String, statements: Vec<Statement>) -> Self {
        Self { name, statements }
    }

    pub fn generate(
        &self,
        stream: &mut impl Write,
        labels: &mut LabelGenerator,
        vars: &mut VariableMap,
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
            s.generate(stream, labels, vars)?;
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

#[derive(Debug, Clone)]
pub enum Type {
    Int,
}

pub type Identifier = String;

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
    Declaration(Type, Identifier, Option<Expression>),
    Expression(Expression),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    Compound(Vec<Statement>),
}

impl Statement {
    pub fn generate(
        &self,
        stream: &mut impl Write,
        labels: &mut LabelGenerator,
        vars: &mut VariableMap,
    ) -> io::Result<()> {
        match self {
            Statement::Compound(stms) => {
                let mut inner_vars = VariableMap::extend(&vars);
                for stm in stms {
                    stm.generate(stream, labels, &mut inner_vars)?;
                }
            }
            Statement::If(cond, stm, alt) => match alt {
                Some(alt) => {
                    let alt_label = labels.unique_label();
                    let post_conditional = labels.unique_label();

                    cond.generate(stream, labels, vars)?;
                    writeln!(
                        stream,
                        "cmp rax, 0\n\
                         je {}",
                        alt_label
                    )?;
                    stm.generate(stream, labels, vars)?;
                    writeln!(
                        stream,
                        "jmp {}\n\
                         {}:",
                        post_conditional, alt_label
                    )?;
                    alt.generate(stream, labels, vars)?;
                    writeln!(stream, "{}:", post_conditional)?;
                }
                None => {
                    let post_conditional = labels.unique_label();

                    cond.generate(stream, labels, vars)?;
                    writeln!(
                        stream,
                        "cmp rax, 0\n\
                         je {}",
                        post_conditional
                    )?;
                    stm.generate(stream, labels, vars)?;
                    writeln!(stream, "{}:", post_conditional)?;
                }
            },
            Statement::Declaration(t, id, expr) => {
                vars.declare(id.clone(), (*t).clone());
                if let Some(e) = expr {
                    e.generate(stream, labels, vars)?;
                    writeln!(stream, "mov [rbp{}], rax", vars.offset_of(&id))?;
                }
            }
            Statement::Expression(e) => {
                e.generate(stream, labels, vars)?;
            }
            Statement::Return(e) => {
                e.generate(stream, labels, vars)?;
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
