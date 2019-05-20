use crate::parser::Rule;
use pest::iterators::Pair;
use std::io::{self, Write};

pub fn generate(program: Pair<Rule>, stream: &mut impl Write) -> io::Result<()> {
    // TODO: better error handling
    for function in program
        .into_inner()
        .filter(|p| p.as_rule() == Rule::function)
    {
        generate_function(function, stream)?;
    }
    Ok(())
}

fn generate_function(function: Pair<Rule>, stream: &mut impl Write) -> io::Result<()> {
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
    match statement.as_rule() {
        Rule::return_statement => {
            let mut inner_iter = statement.into_inner();
            let expression = inner_iter.next().unwrap();
            match expression.as_rule() {
                Rule::integer => {
                    let integer = expression.as_str();
                    writeln!(stream, "movl ${}, %eax\nret", integer)?;
                }
                _ => panic!("Invalid expression in return statement"),
            }
        }
        _ => panic!("Unknown statement type"),
    }
    Ok(())
}
