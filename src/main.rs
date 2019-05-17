extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use std::env;
use std::fs::read_to_string;

#[derive(Parser)]
#[grammar = "syntax.pest"]
struct SyntaxParser;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    for file in args {
        println!("{}", file);
        let text = read_to_string(&file).expect(&format!("Can't open {}", file));
        let ast = parse(&text).unwrap_or_else(|e| {
            eprintln!("{}", e);
            panic!(e);
        });
        println!("{:#?}", ast);
        codegen(ast);
    }
}

fn parse(text: &str) -> Result<Pair<Rule>, Error<Rule>> {
    let mut pairs = SyntaxParser::parse(Rule::program, &text)?;
    Ok(pairs.next().unwrap())
}

// TODO: better error handling
fn codegen(program: Pair<Rule>) {
    for function in program
        .into_inner()
        .filter(|p| p.as_rule() == Rule::function)
    {
        let mut inner_iter = function.into_inner();

        let _type_specifier = inner_iter.next().unwrap();
        let identifier = inner_iter.next().unwrap();

        let identifier_str = identifier.as_str();
        println!(".globl {}\n\n{}:\n", identifier_str, identifier_str);

        for statement in inner_iter {
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
    }
}
