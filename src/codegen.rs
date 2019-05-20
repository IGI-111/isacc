use crate::parser::Rule;
use pest::iterators::Pair;
use std::io::{self, Write};

pub fn generate(program: Pair<Rule>, stream: &mut impl Write) -> io::Result<()> {
    assert!(program.as_rule() == Rule::program);
    for function in program
        .into_inner()
        .filter(|p| p.as_rule() == Rule::function)
    {
        generate_function(function, stream)?;
    }
    Ok(())
}

fn generate_function(function: Pair<Rule>, stream: &mut impl Write) -> io::Result<()> {
    assert!(function.as_rule() == Rule::function);
    let mut inner_iter = function.into_inner();

    let _type_specifier = inner_iter.next().unwrap();
    let identifier = inner_iter.next().unwrap();

    let identifier_str = identifier.as_str();
    writeln!(stream, ".globl {}\n\n{}:", identifier_str, identifier_str)?;

    for statement in inner_iter {
        generate_statement(statement, stream)?;
    }
    Ok(())
}

fn generate_statement(statement: Pair<Rule>, stream: &mut impl Write) -> io::Result<()> {
    assert!(statement.as_rule() == Rule::statement);

    let mut inner_iter = statement.into_inner();
    let inner = inner_iter.next().unwrap();

    match inner.as_rule() {
        Rule::return_statement => {
            let mut inner_iter = inner.into_inner();
            let expression = inner_iter.next().unwrap();
            generate_expression(expression, stream)?;
            writeln!(stream, "ret")?;
        }
        _ => panic!("Unknown statement type"),
    }
    Ok(())
}

fn generate_expression(expression: Pair<Rule>, stream: &mut impl Write) -> io::Result<()> {
    assert!(expression.as_rule() == Rule::expression);
    let mut inner_iter = expression.into_inner();
    let inner_first = inner_iter.next().expect("Empty expression");

    match inner_first.as_rule() {
        Rule::literal => {
            let literal = inner_first.as_str();
            writeln!(stream, "movl ${}, %eax", literal)?;
        }
        Rule::unary_operator => {
            let operator = inner_first.as_str();
            let inner_second = inner_iter.next().expect("Unary operator without operand");
            generate_expression(inner_second, stream)?;
            match operator {
                "-" => writeln!(stream, "neg %eax")?,
                "~" => writeln!(stream, "not %eax")?,
                "!" => writeln!(stream, "cmpl $0, %eax\nmovl $0, %eax\nsete %al")?,
                _ => panic!("Unknown operator"),
            }
        }
        _ => panic!("Invalid expression in return statement"),
    }

    Ok(())
}
