// Separate the imports for better clarity
use crate::Instructions;
use crate::Token;
use crate::utils::{cleanup_line, string_to_u32, register_to_byte};
use crate::instructions::InstructionMode;

/// Represents a Lexer with labels and clean lines.
pub struct Lexer {
    pub labels: Vec<(String, u32)>, // Label name, index
    clean_lines: Vec<Vec<String>>,  // Clean lines generated in the prepass
}

impl Lexer {
    /// Creates a new Lexer.
    pub fn new() -> Self {
        Lexer {
            labels: Vec::new(),
            clean_lines: Vec::new(),
        }
    }

    /// Preprocesses the input to identify and store labels.
    pub fn label_prepass(&mut self, input: String) {
        let mut current_index = 0;
        for line in input.lines() {
            let mut clean_line = cleanup_line(line);
            if self.is_label_line(&clean_line) {
                self.handle_label_line(&mut clean_line, &mut current_index);
            } else {
                self.increment_index_for_non_label_line(&clean_line, &mut current_index);
            }

            println!("Read Line: {:?}", clean_line);
            self.add_clean_line(clean_line);
        }
    }

    /// Determines if the given line is a label line.
    fn is_label_line(&self, clean_line: &[String]) -> bool {
        !clean_line.is_empty() && clean_line[0].contains(":")
    }

    /// Handles label lines by storing the label and updating the index.
    fn handle_label_line(&mut self, clean_line: &mut Vec<String>, current_index: &mut u32) {
        let token = clean_line[0].replace(":", "");
        self.labels.push((token.to_uppercase(), *current_index));
        clean_line.remove(0);
    }

    /// Increments the index for non-label lines.
    fn increment_index_for_non_label_line(&mut self, clean_line: &[String], current_index: &mut u32) {
        if !clean_line.is_empty() {
            *current_index += 4; // 4 bytes per instruction
        }
    }

    fn add_clean_line(&mut self, clean_line: Vec<String>) {
        if !clean_line.is_empty() {
            self.clean_lines.push(clean_line);
        }
    }
}


impl Lexer{
    fn process_register(&self, item: &str) -> Option<u32> {
        let item = item.replace("R", "");
        register_to_byte(&item)
    }
    
    fn process_memory_address(&self, last_value: &str) -> Option<Token> {
        let last_value = last_value.replace("[", "").replace("]", "");
    
        if last_value.contains("+") {
            let split_value: Vec<&str> = last_value.split("+").collect();
            let register = self.process_register(split_value[0])?;
            let offset = string_to_u32(split_value[1])?;
            Some(Token::BaseOffset(register, offset))
        } else {
            let register = self.process_register(&last_value)?;
            Some(Token::Register(register))
        }
    }

    pub fn tokenize(&mut self, input: String) -> Vec<Vec<Token>> {
        let mut tokens = Vec::new();
        
        // Prepass to find all the labels
        self.label_prepass(input);

        for line in &self.clean_lines {
            let mut line_tokens = Vec::new();

            let len = line.len();


            if len > 4{
                // Too many tokens
                panic!("Too many tokens in line: {} -- {:?}", line.join(" "), line);
            }

            // The first token in the line is the instruction
            // We need to convert this to an OpCode. Check src/instructions.rs for more info
            let instruction = match Instructions::try_from(line[0].clone()){
                Ok(op_code) => op_code,
                Err(_) => panic!("Invalid instruction: {}", line[0]),
            };

            line_tokens.push(Token::OpCode(instruction));

            // The rest of the tokens are parameters. 
            // There are different modes,
            // so we need to check which mode we are in. This will be determined
            // by the parameters themselves. The last parameter
            // will always determine the mode.

            // Example instruction:
            // ADD R1, R2, R3 -> Register mode
            // ADD R1, R2, 0x00000001 -> Immediate mode
            // ADD R1, R2, [R3] -> Register Indirect mode
            // ADD R1, R2, [R3 + 0x00000001] -> Base Offset mode
            //
            // It's key to note the first parameter is always the destination register
            // The second parameter is always the first source register. This can't
            // be a value or memory address
            // The third parameter is always the second source register, value or memory address
            // Note - not every instruction has 3 parameters. Some have 2, some have 1
            // However, each instruction's last parameter will always determine the mode

            if len == 1{
                line_tokens.push(Token::Mode(InstructionMode::Register));
                tokens.push(line_tokens);
                continue;
            }
            
            for item in &line[1..len-1] {
                if let Some(register) = self.process_register(item) {
                    line_tokens.push(Token::Register(register));
                } else {
                    panic!("Invalid register: {}", item);
                }
            }
        
            let last_value = &line[len - 1];
            if last_value.contains("[") && last_value.contains("]") {
                match self.process_memory_address(&last_value) {
                    Some(Token::BaseOffset(register, offset)) => {
                        line_tokens.push(Token::BaseOffset(register, offset));
                        line_tokens.insert(1, Token::Mode(InstructionMode::BaseOffset));
                    },
                    Some(Token::Register(register)) => {
                        line_tokens.push(Token::Register(register));
                        line_tokens.insert(1, Token::Mode(InstructionMode::RegisterIndirect));
                    },
                    _ => panic!("Invalid memory address: {}", last_value),
                }
            } else {
                match string_to_u32(&last_value) {
                    Some(value) => {
                        line_tokens.push(Token::Value(value));
                        line_tokens.insert(1, Token::Mode(InstructionMode::Immediate));
                    },
                    None => {
                        let check_reg = last_value.replace("R", "");
                        match register_to_byte(&check_reg) {
                            Some(register) => {
                                line_tokens.push(Token::Register(register));
                                line_tokens.insert(1, Token::Mode(InstructionMode::Register));
                            },
                            None => {
                                println!("Assuming {} is a label", last_value);
                                line_tokens.push(Token::Label(last_value.to_string()));
                                line_tokens.insert(1, Token::Mode(InstructionMode::Immediate));
                            },
                        }
                    }
                }
            }

            tokens.push(line_tokens);
        }


        tokens
    }

    pub fn get_label_address(&self, label: &str) -> Option<u32>{
        let label = label.to_uppercase();
        // Iterate through the labels and find the one we want
        for (label_name, label_address) in &self.labels{
            println!("Comparing {} to {}", label_name, label);
            if label_name == &label{
                return Some(*label_address);
            }
        }

        None
    }
}
