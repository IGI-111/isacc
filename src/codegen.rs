use crate::parser::Rule;
use pest::iterators::Pair;

pub fn generate(program: Pair<Rule>) {
    // TODO: better error handling
    for function in program
        .into_inner()
        .filter(|p| p.as_rule() == Rule::function)
    {
        generate_function(function);
    }
}

fn generate_function(function: Pair<Rule>) {
    let mut inner_iter = function.into_inner();

    let _type_specifier = inner_iter.next().unwrap();
    let identifier = inner_iter.next().unwrap();

    let identifier_str = identifier.as_str();
    println!(".globl {}\n\n{}:", identifier_str, identifier_str);

    for statement in inner_iter {
        generate_statement(statement);
    }
}

fn generate_statement(statement: Pair<Rule>) {
    match statement.as_rule() {
        Rule::return_statement => {
            let mut inner_iter = statement.into_inner();
            let expression = inner_iter.next().unwrap();
            match expression.as_rule() {
                Rule::integer => {
                    let integer = expression.as_str();
                    println!("movl ${}, %eax\nret", integer);
                }
                _ => panic!("Invalid expression in return statement"),
            }
        }
        _ => panic!("Unknown statement type"),
    }
}
