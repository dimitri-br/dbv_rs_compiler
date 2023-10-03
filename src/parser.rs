use crate::{utils::register_to_byte, token::Token, instructions::InstructionMode};
use super::{Instructions, Lexer, utils};
    
#[derive(Debug)]
pub struct ASTNode {
    pub op_code: Instructions,
    pub mode: InstructionMode,
    pub args: Vec<u32>,
    pub token_repr: String,
}

impl ASTNode {
    pub fn new(op_code: Instructions, mode: InstructionMode, args: Vec<u32>) -> Self {
        let mut token_repr = String::from(op_code);
        
        Self {
            op_code,
            mode,
            args,
            token_repr,
        }
    }
}

pub struct Parser<'a> {
    lexer: &'a Lexer,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a Lexer) -> Self {
        Self { lexer }
    }
    
    pub fn parse_line(&self, tokens: Vec<Token>) -> Option<ASTNode> {
        println!("{:?}", tokens);

        // The opcode is always the first token
        let op_code = match tokens[0] {
            Token::OpCode(op_code) => op_code,
            _ => panic!("Invalid token"),
        };

        // The mode is always the second token
        let mode = match tokens[1] {
            Token::Mode(mode) => mode,
            _ => panic!("Invalid token"),
        };

        // The rest of the tokens are parameters. 
        // If it's a label, we need to get the memory address
        // from the lexer. Otherwise, we can pull the value

        let mut args: Vec<u32> = Vec::new();
        for token in &tokens[2..] {
            match token {
                Token::Label(label) => {
                    let label = label.to_uppercase();
                    let label_address = self.lexer.get_label_address(&label).unwrap();
                    args.push(label_address);
                },
                Token::Register(register) => {
                    args.push(*register);
                },
                Token::Value(value) => {
                    args.push(*value);
                },
                _ => panic!("Invalid token"),
            }
        }

        // return
        return Some(ASTNode::new(op_code, mode, args));
    }
}