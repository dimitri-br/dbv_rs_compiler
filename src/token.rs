use crate::instructions::{Instructions, InstructionMode};


#[derive(Debug)]
pub enum Token{
    OpCode(Instructions),
    Mode(InstructionMode),
    Register(u32),
    BaseOffset(u32, u32),
    Value(u32),
    //Memory(u32),
    Label(String),
    None,
}