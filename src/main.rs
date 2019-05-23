#[macro_use]
extern crate combine;

mod codegen;
mod parsing;

use codegen::*;
use parsing::*;

fn main() {
    let text = "int main() { return -~10; }";
    let tokens = lex(text).unwrap();
    let ast = parse(&tokens).unwrap();
    println!("{:#?}", ast);
    codegen(&ast);
}
