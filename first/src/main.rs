mod ast;
mod scanner;
mod parser;
mod codegen;

use std::env;
use std::fs;


use crate::parser::Parser;
use crate::codegen::generate_expr_code;
use crate::ast::ASTNode;
use crate::scanner::scan_source;

use std::cell::RefCell;
use std::rc::Rc;

// --------------------------
// Collect identifiers in FIRST-SEEN order
// --------------------------
fn collect_identifiers_in_order(
    node: &Rc<RefCell<ASTNode>>,
    out: &mut Vec<String>,
    seen: &mut std::collections::HashSet<String>,
) {
    use ast::ASTNode::*;

    match &*node.borrow() {
        Identifier(name) => {
            if !seen.contains(name) {
                seen.insert(name.clone());
                out.push(name.clone());
            }
        }
        Number(_) => {}

        Add(l, r) | Multiply(l, r) => {
            collect_identifiers_in_order(l, out, seen);
            collect_identifiers_in_order(r, out, seen);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: rucompiler-x86-first <file.exp>");
        return;
    }

    let input_file = &args[1];
    let output_file = input_file.replace(".exp", ".s");

    let source = fs::read_to_string(input_file)
        .expect("Failed to read input file");

    // 1. Tokenize using your real scanner.
    let tokens = scan_source(&source);

    // 2. Parse
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expression();

    // 3. Collect identifiers
    let mut ids = Vec::new();
    let mut seen = std::collections::HashSet::new();
    collect_identifiers_in_order(&ast, &mut ids, &mut seen);

    // 4. Generate x86-64 assembly
    let asm = generate_expr_code(&ast, &ids);

    // 5. Write output
    fs::write(&output_file, asm).expect("Failed to write output file");
}
