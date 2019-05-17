extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs::read_to_string;
use std::env;

#[derive(Parser)]
#[grammar = "syntax.pest"]
struct SyntaxParser;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    for file in args {
        parse(&file);
    }


}

fn parse(filename: &str) {
    println!("{}", filename);
    let text = read_to_string(filename).expect(&format!("Can't open {}", filename));
    match SyntaxParser::parse(Rule::program, &text) {
        Ok(program) => println!("{:#?}", program),
        Err(e) => eprintln!("{}", e),
    }
}
