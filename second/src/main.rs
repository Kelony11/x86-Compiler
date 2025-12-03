use std::env;
use std::fs;

mod ast;
mod scanner;
mod parser;
mod codegen;

use crate::scanner::scan_source;
use crate::parser::Parser;
use crate::codegen::generate_program_x86;

fn main() {
    // Expect exactly one argument: the .rucomp file
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: rucompiler-x86-second <input.rucomp>");
        return;
    }

    let input_file = &args[1];

    // Derive output filename: *.rucomp -> *.s
    let output_file = if let Some(stripped) = input_file.strip_suffix(".rucomp") {
        format!("{}.s", stripped)
    } else {
        format!("{}.s", input_file)
    };

    // Read the source program
    let source = fs::read_to_string(input_file)
        .expect("could not read input file");

    // 1) Scan source into tokens
    let tokens = scan_source(&source);

    // 2) Parse the full program (args, vars, statements, return)
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();

    // 3) Generate x86-64 assembly for the full program
    let asm = generate_program_x86(&program);

    // 4) Write the .s file
    fs::write(&output_file, asm)
        .expect("failed to write output file");
}
