use crate::utils::register_to_byte;
use super::{Tokens, Lexer, utils, OPCODE_TABLE};
    
#[derive(Debug)]
pub struct ASTNode {
    pub op_code: u8,
    pub args: Vec<u8>,
    pub token_repr: String,
}

impl ASTNode {
    pub fn new(op_code: u8, args: Vec<u8>) -> Self {
        let token_repr = Tokens::from(op_code as u32).into();
        assert_eq!(args.len() as u8, OPCODE_TABLE[op_code as usize].1, 
                   "Invalid number of arguments for opcode: {}. Expected: {}. Got: {}",
                   token_repr, OPCODE_TABLE[op_code as usize].1, args.len() as u8);
        
        Self {
            op_code,
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
    
    pub fn parse_line(&self, tokens: Vec<Tokens>) -> Option<ASTNode> {
        println!("{:?}", tokens);
        let mut iter = tokens.into_iter().peekable();
        let mut token_opcode: u32 = 0;
        let mut args: Vec<u8> = Vec::new();

        while let Some(token) = iter.next() {
            match token {
                Tokens::Value(val) => args.extend(utils::break_value_into_bytes(val)),
                Tokens::Register(reg) => args.push(register_to_byte(&reg)),
                Tokens::Address(addr) => args.extend(self.handle_address(&addr)),
                Tokens::Label(_) => return None,
                _ => token_opcode = u32::from(token),
            }
        }
        Some(ASTNode::new(token_opcode as u8, args))
    }

    fn handle_address(&self, addr: &str) -> Vec<u8> {
        if let Some(val) = self.lexer.labels.iter().find(|(label, _)| label == addr) {
            return vec![((val.1 & 0xFF00) >> 8) as u8, (val.1 & 0x00FF) as u8];
        }

        utils::parse_address(addr)
    }
}