mod token;
mod lexer;
mod utils;
mod parser;
mod generator;

use std::{fs::{File, read_to_string}, io::Write, env};

use generator::CodeGenerator;
use token::{Tokens, OPCODE_TABLE};
use lexer::Lexer;

fn main() {
    // Input params - first is the input file, second is the output file
    let args: Vec<String> = env::args().collect();

    let input = read_to_string(&args[1]).expect("Failed to read file");

    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize();

    let parser = parser::Parser::new(&lexer);

    let mut nodes = Vec::new();
    for token in tokens {
        if let Some(node) = parser.parse_line(token) {
            nodes.push(node);
        }
    }

    let gen_code = CodeGenerator::generate(nodes);

    // Now to finalize the compilation, save it as binary to a file
    let mut file = File::create(&args[2]).unwrap();
    file.write_all(&gen_code).unwrap();
}

