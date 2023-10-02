#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Tokens{
    HLT = 0x0,      // | 0  |  HLT                              | Halts the program
    PSH = 0x1,      // | 1  |  PSH val                          | Push val to stack
    POP = 0x2,      // | 2  |  POP reg                          | Remove from stack and reg to the value
    SET = 0x3,      // | 3  |  SET reg val                      | Set a register
    MOV = 0x4,      // | 4  |  MOV reg_a reg_b                  | Moves value from register a to register b
    SLT = 0x5,      // | 5  |  SLT reg_a reg_b reg_c            | Set reg_a to 1 if reg_b < reg_c
    ADD = 0x6,      // | 6  |  ADD reg_a reg_b reg_c            | Adds two registers, storing the result in a register [reg_a = reg_b + reg_c]
    SUB = 0x7,      // | 7  |  SUB reg_a reg_b reg_c            | Subtracts two registers, storing the result in a register [reg_a = reg_b - reg_c]
    MUL = 0x8,      // | 8  |  MUL reg_a reg_b reg_c            | Multiplies two registers, storing the result in a register [reg_a = reg_b * reg_c]
    DIV = 0x9,      // | 9  |  DIV reg_a reg_b reg_c            | Divides two registers, storing the result in a register [reg_a = reg_b / reg_c]
    ADDI = 0xA,     // | 10 |  ADDI reg_a reg_b val             | Adds a register and a value, storing the result in a register [reg_a = reg_b + val]
    SUBI = 0xB,     // | 11 |  SUBI reg_a reg_b val             | Subtracts a register and a value, storing the result in a register [reg_a = reg_b - val]
    MULI = 0xC,     // | 12 |  MULI reg_a reg_b val             | Multiplies a register and a value, storing the result in a register [reg_a = reg_b * val]
    DIVI = 0xD,     // | 13 |  DIVI reg_a reg_b val             | Divides a register and a value, storing the result in a register [reg_a = reg_b / val]
    AND = 0xE,      // | 14 |  AND reg_a reg_b reg_c            | Performs an AND operation on two registers, storing the result in a register [reg_a = reg_b & reg_c]
    OR = 0xF,       // | 15 |  OR reg_a reg_b reg_c             | Performs an OR operation on two registers, storing the result in a register [reg_a = reg_b | reg_c]
    SL = 0x10,      // | 16 |  SL reg_a reg_b reg_c             | Performs an SHIFT LEFT operation on two registers, storing the result in a register [reg_a = reg_b << reg_c]
    SR = 0x11,      // | 17 |  SR reg_a reg_b reg_c             | Performs an SHIFT RIGHT operation on two registers, storing the result in a register [reg_a = reg_b >> reg_c]
    SLI = 0x12,     // | 18 |  SLI reg_a reg_b val              | Performs an SHIFT LEFT operation on a register and value, storing the result in a register [reg_a = reg_b << val]
    SRI = 0x13,     // | 19 |  SRI reg_a reg_b val              | Performs an SHIFT RIGHT operation on a register and value, storing the result in a register [reg_a = reg_b >> val]
    SAL = 0x14,     // | 20 |  SAL reg                          | Performs an SHIFT LEFT operation on a register, storing the result in a register [reg <<= 1]
    SAR = 0x15,     // | 21 |  SAR reg                          | Performs an SHIFT RIGHT operation on a register, storing the result in a register [reg >>= 1]
    LD = 0x16,      // | 22 |  LD reg mem_addr                  | Loads data (uint32_t) from a memory address (uint16_t), storing in a register
    SD = 0x17,      // | 23 |  SD mem_addr reg                  | Stores data (uint32_t) from a register into memory  at the given address (uint16_t)
    SDR = 0x18,     // | 24  |  SDR reg_a reg_b                       | Stores the value of reg_b (uint32_t) into the memory address defined by reg_a (uint16_t)
    LDR = 0x19,     // | 25  |  LDR reg_a reg_b                       | Loads the value of the memory address defined by reg_b (uint16_t) into reg_a (uint32_t)
    IF = 0x1A,      // | 26 |  IF reg val addr                  | If reg == val, goto addr
    IFN = 0x1B,     // | 27 |  IFN reg val addr                 | If reg != val, goto addr
    JMP = 0x1C,     // | 28 |  JMP addr                         | Relative jump to addr
    JNZ = 0x1D,     // | 29 |  JNZ reg addr                     | Jumps to addr if reg is not 0
    IFR = 0x1E,     // | 30 |  IFR reg val rel_addr             | Relative jump to rel_addr if reg == val
    IFNR = 0x1F,    // | 31 |  IFNR reg val rel_addr            | Relative jump to rel_addr if reg != val
    JMPR = 0x20,    // | 32 |  JMPR rel_addr                    | Relative jump to addr
    JNZR = 0x21,    // | 33 |  JNZR reg rel_addr                | Relative jump to addr if reg is not 0
    RET = 0x22,     // | 34 |  RET                              | Return from a jump
    SETF = 0x23,    // | 35 |  SETF f_reg val                   | Set a fp reg to val (also an f) (FP operation)
    MOVF = 0x24,    // | 36 |  MOVF f_reg_a f_reg_b             | Move f_reg_a to f_reg_b (FP operation)
    MOVFI = 0x25,   // | 37 |  MOVFI f_reg_a reg_b              | Move f_reg_a to reg_b (warning - this will round the float to the nearest integer) (FP operation)
    MOVIF = 0x26,   // | 38 |  MOVIF reg_a f_reg_b              | Move reg_a to f_reg_b (FP operation)
    ADDF = 0x27,    // | 39 |  ADDF f_reg_a f_reg_b f_reg_c     | add f_reg_b to f_reg_c, storing the result in f_reg_a (FP operation)
    SUBF = 0x28,    // | 40 |  SUBF f_reg_a f_reg_b f_reg_c     | subtract f_reg_b to f_reg_c, storing the result in f_reg_a (FP operation)
    MULF = 0x29,    // | 41 |  MULF f_reg_a f_reg_b f_reg_c     | multiply f_reg_b to f_reg_c, storing the result in f_reg_a (FP operation)
    DIVF = 0x2A,    // | 42 |  DIVF f_reg_a f_reg_b f_reg_c     | divide f_reg_b to f_reg_c, storing the result in f_reg_a (FP operation)
    ADDFI = 0x2B,   // | 43 |  ADDFI f_reg_a f_reg_b value      | add f_reg_b to value, storing the result in f_reg_a (FP operation)
    SUBFI = 0x2C,   // | 44 |  SUBFI f_reg_a f_reg_b value      | subtract f_reg_b by value, storing the result in f_reg_a (FP operation)
    MULFI = 0x2D,   // | 45 |  MULFI f_reg_a f_reg_b value      | multiply f_reg_b by value, storing the result in f_reg_a (FP operation)
    DIVFI = 0x2E,   // | 46 |  DIVFI f_reg_a f_reg_b value      | divide f_reg_b by value, storing the result in f_reg_a (FP operation)
    LDF = 0x2F,     // | 47 |  LDWF f_reg mem_addr              | Loads data (f16) from a memory address (uint16_t), storing in a register (FP operation)
    SDF = 0x30,     // | 48 |  SDWF mem_addr f_reg              | Stores data (f16) from a register into memory  at the given address (uint16_t) (FP operation)
    IFF = 0x31,     // | 49 |  IFF f_reg val addr               | If reg == val, goto addr (FP operation)
    IFNF = 0x32,    // | 50 |  IFNF f_reg val addr              | If reg != val, goto addr (FP operation)
    JNZF = 0x33,    // | 51 |  JNZF f_reg addr                  | Jumps to addr if reg is not 0 (FP operation)
    IFRF = 0x34,    // | 52 |  IFRF f_reg val rel_addr          | Relative jump to rel_addr if reg == val (FP operation)
    IFNRF = 0x35,   // | 53 |  IFNRF f_reg val rel_addr         | Relative jump to rel_addr if reg != val (FP operation)
    JNZRF = 0x36,   // | 54 |  JNZRF f_reg rel_addr             | Relative jump to addr if reg is not 0 (FP operation)
    EQ = 0x37,      // | 55 |  EQ reg_a reg_b reg_c             | set reg_a to 1 if reg_b == reg_c, else 0
    NEQ = 0x38,     // | 56 |  NEQ reg_a reg_b reg_c            | set reg_a to 1 if reg_b != reg_c, else 0
    LEQ = 0x39,     // | 57 |  LEQ reg_a reg_b reg_c            | set reg_a to 1 if reg_b <= reg_c, else 0
    GEQ = 0x3A,     // | 58 |  GEQ reg_a reg_b reg_c            | set reg_a to 1 if reg_b >= reg_c, else 0
    LT = 0x3B,      // | 59 |  LT reg_a reg_b reg_c             | set reg_a to 1 if reg_b < reg_c, else 0
    GT = 0x3C,      // | 60 |  GT reg_a reg_b reg_c             | set reg_a to 1 if reg_b > reg_c, else 0
    EQF = 0x3D,     // | 61 |  EQF f_reg_a f_reg_b f_reg_c      | set reg_a to 1 if reg_b == reg_c, else 0 (FP operation)
    NEQF = 0x3E,    // | 62 |  NEQF f_reg_a f_reg_b f_reg_c     | set reg_a to 1 if reg_b != reg_c, else 0 (FP operation)
    LEQF = 0x3F,    // | 63 |  LEQF f_reg_a f_reg_b f_reg_c     | set reg_a to 1 if reg_b <= reg_c, else 0 (FP operation)
    GEQF = 0x40,    // | 64 |  GEQF f_reg_a f_reg_b f_reg_c     | set reg_a to 1 if reg_b >= reg_c, else 0 (FP operation)
    LTF = 0x41,     // | 65 |  LTF f_reg_a f_reg_b f_reg_c      | set reg_a to 1 if reg_b < reg_c, else 0 (FP operation)
    GTF = 0x42,     // | 66 |  GTF f_reg_a f_reg_b f_reg_c      | set reg_a to 1 if reg_b > reg_c, else 0 (FP operation)
    PSHR = 0x43,    // | 67  |  PSH val                         | Push the register's value to stack
    REC = 0x44,     // | 68  |  REC f_reg_a f_reg_b                  | stores the reciprocal of reg_b into reg_a
    SQRT = 0x45,    // | 69  |  SQRT f_reg_a f_reg_b                 | stores the square root of reg_b into reg_a
    RND = 0x46,     // | 70  |  RND f_reg_a f_reg_b                  | stores the rounded value of reg_b into reg_a
    SIN = 0x47,     // | 71  |  SIN f_reg_a f_reg_b                  | stores the sine of reg_b into reg_a
    COS = 0x48,     // | 72  |  SIN f_reg_a f_reg_b                  | stores the cosine of reg_b into reg_a
    TAN = 0x49,     // | 73  |  TAN f_reg_a f_reg_b                  | stores the tangent of reg_b into reg_a
    ASIN = 0x4A,    // | 74  |  ASIN f_reg_a f_reg_b                 | stores the arcsine of reg_b into reg_a
    ACOS = 0x4B,    // | 75  |  ASIN f_reg_a f_reg_b                 | stores the arccosine of reg_b into reg_a
    ATAN = 0x4C,    // | 76  |  ATAN f_reg_a f_reg_b                 | stores the arctangent of reg_b into reg_a
    SEX = 0x4D,     // | 77  |  SEX flag addr                         | If the exception defined by the flag is triggered, go to the handler (addr)

    // Other Token Types
    Register(String), // can be integer or float
    Value(u32), // always an integer, but can be an integer representation of a float
    Address(String), // Address in memory. Always 16 bits
    Label(String), // a label, used for branching
}



// Convert from a regular opcode integer into an enum
impl From<u32> for Tokens {
    fn from(val: u32) -> Self {
        match val {
            0x0 => Self::HLT,
            0x1 => Self::PSH,
            0x2 => Self::POP,
            0x3 => Self::SET,
            0x4 => Self::MOV,
            0x5 => Self::SLT,
            0x6 => Self::ADD,
            0x7 => Self::SUB,
            0x8 => Self::MUL,
            0x9 => Self::DIV,
            0xA => Self::ADDI,
            0xB => Self::SUBI,
            0xC => Self::MULI,
            0xD => Self::DIVI,
            0xE => Self::AND,
            0xF => Self::OR,
            0x10 => Self::SL,
            0x11 => Self::SR,
            0x12 => Self::SLI,
            0x13 => Self::SRI,
            0x14 => Self::SAL,
            0x15 => Self::SAR,
            0x16 => Self::LD,
            0x17 => Self::SD,
            0x18 => Self::SDR,
            0x19 => Self::LDR,
            0x1A => Self::IF,
            0x1B => Self::IFN,
            0x1C => Self::JMP,
            0x1D => Self::JNZ,
            0x1E => Self::IFR,
            0x1F => Self::IFNR,
            0x20 => Self::JMPR,
            0x21 => Self::JNZR,
            0x22 => Self::RET,
            0x23 => Self::SETF,
            0x24 => Self::MOVF,
            0x25 => Self::MOVFI,
            0x26 => Self::MOVIF,
            0x27 => Self::ADDF,
            0x28 => Self::SUBF,
            0x29 => Self::MULF,
            0x2A => Self::DIVF,
            0x2B => Self::ADDFI,
            0x2C => Self::SUBFI,
            0x2D => Self::MULFI,
            0x2E => Self::DIVFI,
            0x2F => Self::LDF,
            0x30 => Self::SDF,
            0x31 => Self::IFF,
            0x32 => Self::IFNF,
            0x33 => Self::JNZF,
            0x34 => Self::IFRF,
            0x35 => Self::IFNRF,
            0x36 => Self::JNZRF,
            0x37 => Self::EQ,
            0x38 => Self::NEQ,
            0x39 => Self::LEQ,
            0x3A => Self::GEQ,
            0x3B => Self::LT,
            0x3C => Self::GT,
            0x3D => Self::EQF,
            0x3E => Self::NEQF,
            0x3F => Self::LEQF,
            0x40 => Self::GEQF,
            0x41 => Self::LTF,
            0x42 => Self::GTF,
            0x43 => Self::PSHR,
            0x44 => Self::REC,
            0x45 => Self::SQRT,
            0x46 => Self::RND,
            0x47 => Self::SIN,
            0x48 => Self::COS,
            0x49 => Self::TAN,
            0x4A => Self::ASIN,
            0x4B => Self::ACOS,
            0x4C => Self::ATAN,
            0x4D => Self::SEX,
            _ => panic!("Error - value {} does not correspond to an instruction!", val),
        }
    }
}


impl From<Tokens> for u32 {
    fn from(inst: Tokens) -> Self {
        match inst {
            Tokens::HLT => 0x0,
            Tokens::PSH => 0x1,
            Tokens::POP => 0x2,
            Tokens::SET => 0x3,
            Tokens::MOV => 0x4,
            Tokens::SLT => 0x5,
            Tokens::ADD => 0x6,
            Tokens::SUB => 0x7,
            Tokens::MUL => 0x8,
            Tokens::DIV => 0x9,
            Tokens::ADDI => 0xA,
            Tokens::SUBI => 0xB,
            Tokens::MULI => 0xC,
            Tokens::DIVI => 0xD,
            Tokens::AND => 0xE,
            Tokens::OR => 0xF,
            Tokens::SL => 0x10,
            Tokens::SR => 0x11,
            Tokens::SLI => 0x12,
            Tokens::SRI => 0x13,
            Tokens::SAL => 0x14,
            Tokens::SAR => 0x15,
            Tokens::LD => 0x16,
            Tokens::SD => 0x17,
            Tokens::SDR => 0x18,
            Tokens::LDR => 0x19,
            Tokens::IF => 0x1A,
            Tokens::IFN => 0x1B,
            Tokens::JMP => 0x1C,
            Tokens::JNZ => 0x1D,
            Tokens::IFR => 0x1E,
            Tokens::IFNR => 0x1F,
            Tokens::JMPR => 0x20,
            Tokens::JNZR => 0x21,
            Tokens::RET => 0x22,
            Tokens::SETF => 0x23,
            Tokens::MOVF => 0x24,
            Tokens::MOVFI => 0x25,
            Tokens::MOVIF => 0x26,
            Tokens::ADDF => 0x27,
            Tokens::SUBF => 0x28,
            Tokens::MULF => 0x29,
            Tokens::DIVF => 0x2A,
            Tokens::ADDFI => 0x2B,
            Tokens::SUBFI => 0x2C,
            Tokens::MULFI => 0x2D,
            Tokens::DIVFI => 0x2E,
            Tokens::LDF => 0x2F,
            Tokens::SDF => 0x30,
            Tokens::IFF => 0x31,
            Tokens::IFNF => 0x32,
            Tokens::JNZF => 0x33,
            Tokens::IFRF => 0x34,
            Tokens::IFNRF => 0x35,
            Tokens::JNZRF => 0x36,
            Tokens::EQ => 0x37,
            Tokens::NEQ => 0x38,
            Tokens::LEQ => 0x39,
            Tokens::GEQ => 0x3A,
            Tokens::LT => 0x3B,
            Tokens::GT => 0x3C,
            Tokens::EQF => 0x3D,
            Tokens::NEQF => 0x3E,
            Tokens::LEQF => 0x3F,
            Tokens::GEQF => 0x40,
            Tokens::LTF => 0x41,
            Tokens::GTF => 0x42,
            Tokens::PSHR => 0x43,
            Tokens::REC => 0x44,
            Tokens::SQRT => 0x45,
            Tokens::RND => 0x46,
            Tokens::SIN => 0x47,
            Tokens::COS => 0x48,
            Tokens::TAN => 0x49,
            Tokens::ASIN => 0x4A,
            Tokens::ACOS => 0x4B,
            Tokens::ATAN => 0x4C,
            Tokens::SEX => 0x4D,

            _ => panic!("Error - instruction {:?} does not correspond to a value!", inst),
        }
    }
}


// String representation of the instruction
impl From<Tokens> for String {
    fn from(val: Tokens) -> Self {
        match val {
            Tokens::HLT => "HLT".to_string(),
            Tokens::PSH => "PSH".to_string(),
            Tokens::POP => "POP".to_string(),
            Tokens::SET => "SET".to_string(),
            Tokens::MOV => "MOV".to_string(),
            Tokens::SLT => "SLT".to_string(),
            Tokens::ADD => "ADD".to_string(),
            Tokens::SUB => "SUB".to_string(),
            Tokens::MUL => "MUL".to_string(),
            Tokens::DIV => "DIV".to_string(),
            Tokens::ADDI => "ADDI".to_string(),
            Tokens::SUBI => "SUBI".to_string(),
            Tokens::MULI => "MULI".to_string(),
            Tokens::DIVI => "DIVI".to_string(),
            Tokens::AND => "AND".to_string(),
            Tokens::OR => "OR".to_string(),
            Tokens::SL => "SL".to_string(),
            Tokens::SR => "SR".to_string(),
            Tokens::SLI => "SLI".to_string(),
            Tokens::SRI => "SRI".to_string(),
            Tokens::SAL => "SAL".to_string(),
            Tokens::SAR => "SAR".to_string(),
            Tokens::LD => "LD".to_string(),
            Tokens::SD => "SD".to_string(),
            Tokens::SDR => "SDR".to_string(),
            Tokens::LDR => "LDR".to_string(),
            Tokens::IF => "IF".to_string(),
            Tokens::IFN => "IFN".to_string(),
            Tokens::JMP => "JMP".to_string(),
            Tokens::JNZ => "JNZ".to_string(),
            Tokens::IFR => "IFR".to_string(),
            Tokens::IFNR => "IFNR".to_string(),
            Tokens::JMPR => "JMPR".to_string(),
            Tokens::JNZR => "JNZR".to_string(),
            Tokens::RET => "RET".to_string(),
            Tokens::SETF => "SETF".to_string(),
            Tokens::MOVF => "MOVF".to_string(),
            Tokens::MOVFI => "MOVFI".to_string(),
            Tokens::MOVIF => "MOVIF".to_string(),
            Tokens::ADDF => "ADDF".to_string(),
            Tokens::SUBF => "SUBF".to_string(),
            Tokens::MULF => "MULF".to_string(),
            Tokens::DIVF => "DIVF".to_string(),
            Tokens::ADDFI => "ADDFI".to_string(),
            Tokens::SUBFI => "SUBFI".to_string(),
            Tokens::MULFI => "MULFI".to_string(),
            Tokens::DIVFI => "DIVFI".to_string(),
            Tokens::LDF => "LDF".to_string(),
            Tokens::SDF => "SDF".to_string(),
            Tokens::IFF => "IFF".to_string(),
            Tokens::IFNF => "IFNF".to_string(),
            Tokens::JNZF => "JNZF".to_string(),
            Tokens::IFRF => "IFRF".to_string(),
            Tokens::IFNRF => "IFNRF".to_string(),
            Tokens::JNZRF => "JNZRF".to_string(),
            Tokens::EQ => "EQ".to_string(),
            Tokens::NEQ => "NEQ".to_string(),
            Tokens::LEQ => "LEQ".to_string(),
            Tokens::GEQ => "GEQ".to_string(),
            Tokens::LT => "LT".to_string(),
            Tokens::GT => "GT".to_string(),
            Tokens::EQF => "EQF".to_string(),
            Tokens::NEQF => "NEQF".to_string(),
            Tokens::LEQF => "LEQF".to_string(),
            Tokens::GEQF => "GEQF".to_string(),
            Tokens::LTF => "LTF".to_string(),
            Tokens::GTF => "GTF".to_string(),
            Tokens::PSHR => "PSHR".to_string(),
            Tokens::REC => "REC".to_string(),
            Tokens::SQRT => "SQRT".to_string(),
            Tokens::RND => "RND".to_string(),
            Tokens::SIN => "SIN".to_string(),
            Tokens::COS => "COS".to_string(),
            Tokens::TAN => "TAN".to_string(),
            Tokens::ASIN => "ASIN".to_string(),
            Tokens::ACOS => "ACOS".to_string(),
            Tokens::ATAN => "ATAN".to_string(),
            Tokens::SEX => "SEX".to_string(),


            Tokens::Register(val) => val,
            Tokens::Value(val) => val.to_string(),
            Tokens::Address(val) => val.to_string(),
            Tokens::Label(val) => val,
        }
    }
}

// Now from string to instruction
use std::convert::TryFrom;

impl TryFrom<String> for Tokens {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "HLT" => Ok(Tokens::HLT),
            "PSH" => Ok(Tokens::PSH),
            "POP" => Ok(Tokens::POP),
            "SET" => Ok(Tokens::SET),
            "MOV" => Ok(Tokens::MOV),
            "SLT" => Ok(Tokens::SLT),
            "ADD" => Ok(Tokens::ADD),
            "SUB" => Ok(Tokens::SUB),
            "MUL" => Ok(Tokens::MUL),
            "DIV" => Ok(Tokens::DIV),
            "ADDI" => Ok(Tokens::ADDI),
            "SUBI" => Ok(Tokens::SUBI),
            "MULI" => Ok(Tokens::MULI),
            "DIVI" => Ok(Tokens::DIVI),
            "AND" => Ok(Tokens::AND),
            "OR" => Ok(Tokens::OR),
            "SL" => Ok(Tokens::SL),
            "SR" => Ok(Tokens::SR),
            "SLI" => Ok(Tokens::SLI),
            "SRI" => Ok(Tokens::SRI),
            "SAL" => Ok(Tokens::SAL),
            "SAR" => Ok(Tokens::SAR),
            "LD" => Ok(Tokens::LD),
            "SD" => Ok(Tokens::SD),
            "IF" => Ok(Tokens::IF),
            "IFN" => Ok(Tokens::IFN),
            "JMP" => Ok(Tokens::JMP),
            "JNZ" => Ok(Tokens::JNZ),
            "IFR" => Ok(Tokens::IFR),
            "IFNR" => Ok(Tokens::IFNR),
            "JMPR" => Ok(Tokens::JMPR),
            "JNZR" => Ok(Tokens::JNZR),
            "RET" => Ok(Tokens::RET),
            "SETF" => Ok(Tokens::SETF),
            "MOVF" => Ok(Tokens::MOVF),
            "MOVFI" => Ok(Tokens::MOVFI),
            "MOVIF" => Ok(Tokens::MOVIF),
            "ADDF" => Ok(Tokens::ADDF),
            "SUBF" => Ok(Tokens::SUBF),
            "MULF" => Ok(Tokens::MULF),
            "DIVF" => Ok(Tokens::DIVF),
            "ADDFI" => Ok(Tokens::ADDFI),
            "SUBFI" => Ok(Tokens::SUBFI),
            "MULFI" => Ok(Tokens::MULFI),
            "DIVFI" => Ok(Tokens::DIVFI),
            "IFF" => Ok(Tokens::IFF),
            "IFNF" => Ok(Tokens::IFNF),
            "IFRF" => Ok(Tokens::IFRF),
            "IFNRF" => Ok(Tokens::IFNRF),
            "JNZF" => Ok(Tokens::JNZF),
            "JNZRF" => Ok(Tokens::JNZRF),
            "EQ" => Ok(Tokens::EQ),
            "NEQ" => Ok(Tokens::NEQ),
            "LEQ" => Ok(Tokens::LEQ),
            "GEQ" => Ok(Tokens::GEQ),
            "LT" => Ok(Tokens::LT),
            "GT" => Ok(Tokens::GT),
            "EQF" => Ok(Tokens::EQF),
            "NEQF" => Ok(Tokens::NEQF),
            "LEQF" => Ok(Tokens::LEQF),
            "GEQF" => Ok(Tokens::GEQF),
            "LTF" => Ok(Tokens::LTF),
            "GTF" => Ok(Tokens::GTF),
            "PSHR" => Ok(Tokens::PSHR),
            "REC" => Ok(Tokens::REC),
            "SQRT" => Ok(Tokens::SQRT),
            "RND" => Ok(Tokens::RND),
            "SIN" => Ok(Tokens::SIN),
            "COS" => Ok(Tokens::COS),
            "TAN" => Ok(Tokens::TAN),
            "ASIN" => Ok(Tokens::ASIN),
            "ACOS" => Ok(Tokens::ACOS),
            "ATAN" => Ok(Tokens::ATAN),
            "SEX" => Ok(Tokens::SEX),
            "SDR" => Ok(Tokens::SDR),
            "LDR" => Ok(Tokens::LDR),
            "SDF" => Ok(Tokens::SDF),
            "LDF" => Ok(Tokens::LDF),
            _ => {
                return Err("Error - value does not correspond to an instruction!")           
            }
        }
    }
}


pub const OPCODE_TABLE: [(Tokens, u8); 78] = [
    (Tokens::HLT, 0),
    (Tokens::PSH, 4),
    (Tokens::POP, 1),
    (Tokens::SET, 5),
    (Tokens::MOV, 2),
    (Tokens::SLT, 3),

    (Tokens::ADD, 3),
    (Tokens::SUB, 3),
    (Tokens::MUL, 3),
    (Tokens::DIV, 3),

    (Tokens::ADDI, 6),
    (Tokens::SUBI, 6),
    (Tokens::MULI, 6),
    (Tokens::DIVI, 6),

    (Tokens::AND, 3),
    (Tokens::OR, 3),
    (Tokens::SL, 3),
    (Tokens::SR, 3),
    (Tokens::SLI, 6),
    (Tokens::SRI, 6),
    (Tokens::SAL, 1),
    (Tokens::SAR, 1),

    (Tokens::LD, 3),
    (Tokens::SD, 3),
    (Tokens::SDR, 2),
    (Tokens::LDR, 2),

    (Tokens::IF, 7),
    (Tokens::IFN, 7),
    (Tokens::JMP, 2),
    (Tokens::JNZ, 3),

    (Tokens::IFR, 6),
    (Tokens::IFNR, 6),
    (Tokens::JMPR, 1),
    (Tokens::JNZR, 2),

    (Tokens::RET, 0),

    (Tokens::SETF, 5),

    (Tokens::MOVF, 2),
    (Tokens::MOVFI, 2),
    (Tokens::MOVIF, 2),

    
    (Tokens::ADDF, 3),
    (Tokens::SUBF, 3),
    (Tokens::MULF, 3),
    (Tokens::DIVF, 3),

    (Tokens::ADDFI, 6),
    (Tokens::SUBFI, 6),
    (Tokens::MULFI, 6),
    (Tokens::DIVFI, 6),

    (Tokens::LDF, 5),
    (Tokens::SDF, 5),

    (Tokens::IFF, 7),
    (Tokens::IFNF, 7),
    (Tokens::JNZF, 3),

    (Tokens::IFRF, 6),
    (Tokens::IFNRF, 6),
    (Tokens::JNZRF, 2),

    (Tokens::EQ, 3),
    (Tokens::NEQ, 3),
    (Tokens::LEQ, 3),
    (Tokens::GEQ, 3),
    (Tokens::LT, 3),
    (Tokens::GT, 3),

    (Tokens::EQF, 3),
    (Tokens::NEQF, 3),
    (Tokens::LEQF, 3),
    (Tokens::GEQF, 3),
    (Tokens::LTF, 3),
    (Tokens::GTF, 3),

    (Tokens::PSHR, 1),

    (Tokens::REC, 2),
    (Tokens::SQRT, 2),
    (Tokens::RND, 2),

    (Tokens::SIN, 2),
    (Tokens::COS, 2),
    (Tokens::TAN, 2),

    (Tokens::ASIN, 2),
    (Tokens::ACOS, 2),
    (Tokens::ATAN, 2),

    (Tokens::SEX, 6),
];
