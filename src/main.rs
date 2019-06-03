#[macro_use]
extern crate combine;
extern crate indexmap;

mod codegen;
mod error;
mod lexing;
mod parsing;

use codegen::*;
use lexing::*;
use parsing::*;
use std::env;
use std::error::Error;
use std::fs::read_to_string;
use std::fs::File;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    for file in args {
        let text = read_to_string(&file)?;
        let tokens: Vec<Token> = lex(&text)?;
        println!("{:#?}", tokens);
        let ast: Vec<Function> = parse(&tokens)?;
        println!("{:#?}", ast);
        let output_path = format!(
            "{}.s",
            Path::new(&file)
                .file_stem()
                .expect("Can't open output file")
                .to_str()
                .unwrap()
        );
        let mut output_stream = File::create(&output_path)?;
        codegen(&ast, &mut output_stream)?;
    }
    Ok(())
}
