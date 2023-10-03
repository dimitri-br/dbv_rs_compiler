mod instructions;
mod lexer;
mod utils;
mod parser;
mod generator;
mod token;

use std::{fs::{File, read_to_string}, io::Write, env};

use instructions::*;
use generator::*;
use lexer::*;
use parser::*;
use token::*;



fn main() {
    // Input params - first is the input file, second is the output file
    let args: Vec<String> = env::args().collect();

    let input = read_to_string(&args[1]).expect("Failed to read file");

    let mut lexer = Lexer::new();
    let tokens = lexer.tokenize(input);

    println!("Tokens: {:?}", tokens);

    let parser = parser::Parser::new(&lexer);

    let mut nodes = Vec::new();
    for token in tokens {
        if let Some(node) = parser.parse_line(token) {
            nodes.push(node);
        }
    }

    let gen_code = CodeGenerator::generate(nodes);

    // Convert the vector of u32 to a vector of bytes
    let gen_code: Vec<u8> = gen_code.iter().map(|x| {
        let bytes = x.to_be_bytes();
        bytes.to_vec()
    }).flatten().collect();

    println!("Generated code: {:?}", gen_code);

    // Now to finalize the compilation, save it as binary to a file
    let mut file = File::create(&args[2]).unwrap();
    file.write_all(&gen_code).unwrap();
}

