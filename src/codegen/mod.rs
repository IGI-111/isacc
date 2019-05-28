mod label;
mod variable;

use self::label::LabelGenerator;
use self::variable::VariableMap;
use std::io::{self, Write};

pub fn codegen(program: &[Function], stream: &mut impl Write) -> io::Result<()> {
    writeln!(stream, ".intel_syntax noprefix")?;

    let mut labels = LabelGenerator::new();
    let mut vars = VariableMap::new();
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
                ".globl _main\n\
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
}

impl Statement {
    pub fn generate(
        &self,
        stream: &mut impl Write,
        labels: &mut LabelGenerator,
        vars: &mut VariableMap,
    ) -> io::Result<()> {
        match self {
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

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    Literal(usize),
    Minus(Box<Expression>),
    BinaryNot(Box<Expression>),
    LogicalNot(Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Equal(Box<Expression>, Box<Expression>),
    NotEqual(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),
    LessThanOrEqual(Box<Expression>, Box<Expression>),
    GreaterThan(Box<Expression>, Box<Expression>),
    GreaterThanOrEqual(Box<Expression>, Box<Expression>),
    Assignment(Identifier, Box<Expression>),
}

impl Expression {
    pub fn generate(
        &self,
        stream: &mut impl Write,
        labels: &mut LabelGenerator,
        vars: &mut VariableMap,
    ) -> io::Result<()> {
        match self {
            Expression::Identifier(id) => {
                    writeln!(stream, "mov [rbp{}], rax", vars.offset_of(&id))?;
            }
            Expression::Assignment(id, e) => {
                e.generate(stream, labels, vars)?;
                    writeln!(stream, "mov [rbp{}], rax", vars.offset_of(&id))?;
            }
            Expression::Literal(i) => {
                writeln!(stream, "mov rax, {}", i)?;
            }
            Expression::Minus(e) => {
                e.generate(stream, labels, vars)?;
                writeln!(stream, "neg rax")?;
            }
            Expression::BinaryNot(e) => {
                e.generate(stream, labels, vars)?;
                writeln!(stream, "not rax")?;
            }
            Expression::LogicalNot(e) => {
                e.generate(stream, labels, vars)?;
                writeln!(
                    stream,
                    "cmp rax, 0\n\
                     mov rax, 0\n\
                     sete al"
                )?;
            }
            Expression::Subtract(e1, e2) => {
                e1.generate(stream, labels, vars)?;
                writeln!(stream, "push rax")?;
                e2.generate(stream, labels, vars)?;
                writeln!(
                    stream,
                    "pop rcx\n\
                     sub rax, rcx"
                )?;
            }
            Expression::Add(e1, e2) => {
                e1.generate(stream, labels, vars)?;
                writeln!(stream, "push rax")?;
                e2.generate(stream, labels, vars)?;
                writeln!(
                    stream,
                    "pop rcx\n\
                     add rax, rcx"
                )?;
            }
            Expression::Multiply(e1, e2) => {
                e1.generate(stream, labels, vars)?;
                writeln!(stream, "push rax")?;
                e2.generate(stream, labels, vars)?;
                writeln!(
                    stream,
                    "pop rcx\n\
                     imul rax, rcx"
                )?;
            }
            Expression::Divide(e1, e2) => {
                e1.generate(stream, labels, vars)?;
                writeln!(stream, "push rax")?;
                e2.generate(stream, labels, vars)?;
                writeln!(
                    stream,
                    "mov rdx, 0\n\
                     mov rcx, rax\n\
                     pop rax\n\
                     idiv rcx"
                )?;
            }
            Expression::And(e1, e2) => {
                let end = labels.unique_label();
                let second_clause = labels.unique_label();
                e1.generate(stream, labels, vars)?;
                writeln!(
                    stream,
                    "cmp rax, 0\n\
                     jne {}\n\
                     jmp {}\n\
                     {}:",
                    second_clause, end, second_clause
                )?;
                e2.generate(stream, labels, vars)?;
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
                let end = labels.unique_label();
                let second_clause = labels.unique_label();
                e1.generate(stream, labels, vars)?;
                writeln!(
                    stream,
                    "cmp rax, 0\n\
                     je {}\n\
                     mov rax, 1\n\
                     jmp {}\n\
                     {}:",
                    second_clause, end, second_clause
                )?;
                e2.generate(stream, labels, vars)?;
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
                e1.generate(stream, labels, vars)?;
                writeln!(stream, "push rax")?;
                e2.generate(stream, labels, vars)?;
                writeln!(
                    stream,
                    "pop rcx\n\
                     cmp rcx, rax\n\
                     mov rax, 0\n\
                     sete al"
                )?;
            }
            Expression::NotEqual(e1, e2) => {
                e1.generate(stream, labels, vars)?;
                writeln!(stream, "push rax")?;
                e2.generate(stream, labels, vars)?;
                writeln!(
                    stream,
                    "pop rcx\n\
                     cmp rcx, rax\n\
                     mov rax, 0\n\
                     setne al"
                )?;
            }
            Expression::LessThan(e1, e2) => {
                e1.generate(stream, labels, vars)?;
                writeln!(stream, "push rax")?;
                e2.generate(stream, labels, vars)?;
                writeln!(
                    stream,
                    "pop rcx\n\
                     cmp rcx, rax\n\
                     mov rax, 0\n\
                     setl al"
                )?;
            }
            Expression::LessThanOrEqual(e1, e2) => {
                e1.generate(stream, labels, vars)?;
                writeln!(stream, "push rax")?;
                e2.generate(stream, labels, vars)?;
                writeln!(
                    stream,
                    "pop rcx\n\
                     cmp rcx, rax\n\
                     mov rax, 0\n\
                     setle al"
                )?;
            }
            Expression::GreaterThan(e1, e2) => {
                e1.generate(stream, labels, vars)?;
                writeln!(stream, "push rax")?;
                e2.generate(stream, labels, vars)?;
                writeln!(
                    stream,
                    "pop rcx\n\
                     cmp rcx, rax\n\
                     mov rax, 0\n\
                     setg al"
                )?;
            }
            Expression::GreaterThanOrEqual(e1, e2) => {
                e1.generate(stream, labels, vars)?;
                writeln!(stream, "push rax")?;
                e2.generate(stream, labels, vars)?;
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
