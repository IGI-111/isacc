extern crate pest;
#[macro_use]
extern crate pest_derive;

mod codegen;
mod parser;

use codegen::generate;
use parser::parse;
use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    for file in args {
        let text = read_to_string(&file).expect(&format!("Can't open {}", file));
        let ast = match parse(&text) {
            Ok(r) => r,
            Err(e) => panic!(format!("{}", e)),
        };
        generate(ast);
    }
}
