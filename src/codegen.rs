use std::io::{self, Write};

pub fn codegen(program: &[Function], stream: &mut impl Write) -> io::Result<()> {
    for function in program {
        function.generate(stream)?;
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

    pub fn generate(&self, stream: &mut impl Write) -> io::Result<()> {
        writeln!(
            stream,
            ".globl {}\n\
             {}:",
            self.name, self.name
        )?;
        for s in self.statements.iter() {
            s.generate(stream)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

impl Statement {
    pub fn generate(&self, stream: &mut impl Write) -> io::Result<()> {
        match self {
            Statement::Return(e) => {
                e.generate(stream)?;
                writeln!(stream, "ret")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Expression {
    Literal(usize),
    Minus(Box<Expression>),
    BinaryNot(Box<Expression>),
    LogicalNot(Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn generate(&self, stream: &mut impl Write) -> io::Result<()> {
        match self {
            Expression::Literal(i) => {
                writeln!(stream, "movq ${}, %rax", i)?;
            }
            Expression::Minus(e) => {
                e.generate(stream)?;
                writeln!(stream, "neg %rax")?;
            }
            Expression::BinaryNot(e) => {
                e.generate(stream)?;
                writeln!(stream, "not %rax")?;
            }
            Expression::LogicalNot(e) => {
                e.generate(stream)?;
                writeln!(
                    stream,
                    "cmpq $0, %rax\n\
                     movq $0, %rax\n\
                     sete %al"
                )?;
            }
            Expression::Subtract(e1, e2) => {
                e1.generate(stream)?;
                writeln!(stream, "pushq %rax")?;
                e2.generate(stream)?;
                writeln!(
                    stream,
                    "popq %rcx\n\
                     subq %rcx, %rax"
                )?;
            }
            Expression::Add(e1, e2) => {
                e1.generate(stream)?;
                writeln!(stream, "pushq %rax")?;
                e2.generate(stream)?;
                writeln!(
                    stream,
                    "popq %rcx\n\
                     addq %rcx, %rax"
                )?;
            }
            Expression::Multiply(e1, e2) => {
                e1.generate(stream)?;
                writeln!(stream, "pushq %rax")?;
                e2.generate(stream)?;
                writeln!(
                    stream,
                    "popq %rcx\n\
                     imulq %rcx, %rax"
                )?;
            }
            Expression::Divide(e1, e2) => {
                e1.generate(stream)?;
                writeln!(stream, "pushq %rax")?;
                e2.generate(stream)?;
                writeln!(
                    stream,
                    "movq $0, %rdx\n\
                     movq %rax, %rcx\n\
                     popq %rax\n\
                     idivq %rcx"
                )?;
            }
        }
        Ok(())
    }
}
