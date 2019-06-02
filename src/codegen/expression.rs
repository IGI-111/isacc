use super::label::LabelGenerator;
use super::variable::VariableMap;
use super::Identifier;
use std::io::{self, Write};

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    Literal(usize),
    Minus(Box<Expression>),
    BinaryNot(Box<Expression>),
    LogicalNot(Box<Expression>),
    PreIncrement(Identifier),
    PreDecrement(Identifier),
    PostIncrement(Identifier),
    PostDecrement(Identifier),
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
    Conditional(Box<Expression>, Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn generate(
        &self,
        stream: &mut impl Write,
        labels: &mut LabelGenerator,
        vars: &mut VariableMap,
    ) -> io::Result<()> {
        match self {
            Expression::Conditional(cond, exp, alt) => {
                let alt_label = labels.unique_label();
                let post_conditional = labels.unique_label();

                cond.generate(stream, labels, vars)?;
                writeln!(
                    stream,
                    "cmp rax, 0\n\
                     je {}",
                    alt_label
                )?;
                exp.generate(stream, labels, vars)?;
                writeln!(
                    stream,
                    "jmp {}\n\
                     {}:",
                    post_conditional,
                    alt_label
                )?;
                alt.generate(stream, labels, vars)?;
                writeln!(stream, "{}:", post_conditional)?;
            }
            Expression::PreIncrement(id) => {
                let offset = vars.offset_of(&id);
                writeln!(
                    stream,
                    "add QWORD PTR [rbp{}], 1\n\
                     mov rax, [rbp{}]",
                    offset, offset
                )?;
            }
            Expression::PreDecrement(id) => {
                let offset = vars.offset_of(&id);
                writeln!(
                    stream,
                    "sub QWORD PTR [rbp{}], 1\n\
                     mov rax, [rbp{}]",
                    offset, offset
                )?;
            }
            Expression::PostIncrement(id) => {
                let offset = vars.offset_of(&id);
                writeln!(
                    stream,
                    "mov rax, [rbp{}]\n\
                     add QWORD PTR [rbp{}], 1",
                    offset, offset
                )?;
            }
            Expression::PostDecrement(id) => {
                let offset = vars.offset_of(&id);
                writeln!(
                    stream,
                    "mov rax, [rbp{}]\n\
                     sub QWORD PTR [rbp{}], 1",
                    offset, offset
                )?;
            }
            Expression::Identifier(id) => {
                writeln!(stream, "mov rax, [rbp{}]", vars.offset_of(&id))?;
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
