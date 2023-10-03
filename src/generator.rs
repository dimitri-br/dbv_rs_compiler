use crate::instructions::{Instructions, InstructionMode};

use super::parser::ASTNode;

pub struct CodeGenerator;

impl CodeGenerator {
    pub fn generate(nodes: Vec<ASTNode>) -> Vec<u32> {
        let nodes: Vec<u32> = nodes.iter().flat_map(|node| {
            let bytes = encode_instruction(node.op_code, node.mode, &node.args);
            // If extension is equal to 0, remove it
            if bytes[1] == 0 {
                vec![bytes[0]]
            }else{
                bytes
            }
        }).collect();

        for node in nodes.iter() {
            println!("{0:8X}", node);
        }

        nodes
    }
}

pub fn encode_instruction(instruction: Instructions, mode: InstructionMode, arguments: &[u32]) -> Vec<u32>{
    let mut raw_instruction: u32 = 0;
    let mut extension = 0;

    let args_len = arguments.len();

    // Format:
    // | 0000_0000 | 0000 | 0000_0000_0000_0000_0000 | -> 32 bits
    // |  Opcode   | Mode |         Arguments        |

    // Encode the opcode
    raw_instruction |= (instruction as u32) << 24; // 0b1111_1111_0000_0000_0000_0000_0000_0000

    // Encode the mode
    match mode{
        InstructionMode::Register => {
            raw_instruction |= 0x00000000; // 0b0000_0000_0000_0000_0000_0000_0000_0000
        },
        InstructionMode::Immediate => {
            raw_instruction |= 0x400000; // 0b0000_0000_0100_0000_0000_0000_0000_0000
        },
        InstructionMode::RegisterIndirect => {
            raw_instruction |= 0x800000; // 0b0000_0000_1000_0000_0000_0000_0000_0000
        },
        InstructionMode::BaseOffset => {
            raw_instruction |= 0xC00000; // 0b0000_0000_1100_0000_0000_0000_0000_0000
        },
    }

    // Encode the arguments
    match mode{
        InstructionMode::Register => {
            // As a loop (up to 4 arguments)
            for (i, arg) in arguments.iter().enumerate() {
                raw_instruction |= (arg & 0xF) << (4 * (3 - i));
            }
        },
        InstructionMode::Immediate => {
             for (i, arg) in arguments.iter().enumerate() {
                if i == args_len - 1 {
                    // If the argument is greater than 0xF, we need to set the extension bit
                    if arg > &0xF {
                        println!("Extended: {:#X} for {:#X}", arg, instruction as u32);
                        raw_instruction |= 0x1; // 0b0000_0000_0000_0000_0000_0000_0000_0001
                        extension = *arg;
                    }else{
                        raw_instruction |= (arg & 0xF) << 4; // We fit in a nibble so we can just set the bits
                    }
                }else{
                    raw_instruction |= (arg & 0xF) << (4 * (3 - i));
                }
            }
        },
        InstructionMode::RegisterIndirect => {
            // As a loop (up to 4 arguments)
            for (i, arg) in arguments.iter().enumerate() {
                raw_instruction |= (arg & 0xF) << (4 * (3 - i));
            }
        },
        InstructionMode::BaseOffset => {
            // As a loop (up to 4 arguments)
            for (i, arg) in arguments.iter().enumerate() {
                raw_instruction |= (arg & 0xF) << (4 * (3 - i));
            }
        },
    }

    vec![raw_instruction, extension]
}