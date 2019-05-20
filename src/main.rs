extern crate pest;
#[macro_use]
extern crate pest_derive;

mod codegen;
mod parser;

use codegen::generate;
use parser::parse;
use std::env;
use std::fs::read_to_string;
use std::fs::File;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    for file in args {
        let text = read_to_string(&file).expect(&format!("Can't open {}", file));
        let ast = match parse(&text) {
            Ok(r) => r,
            Err(e) => panic!(format!("{}", e)),
        };

        let output_path = format!(
            "{}.s",
            Path::new(&file)
                .file_stem()
                .expect("Can't open output file")
                .to_str()
                .unwrap()
        );
        let mut output_stream = File::create(&output_path).unwrap();

        generate(ast, &mut output_stream).unwrap();
    }
}
