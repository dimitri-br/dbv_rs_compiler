use crate::{Instructions, Token, utils::{cleanup_line, self, Parameter}, instructions::InstructionMode};


pub struct Lexer {
    pub labels: Vec<(String, u32)>, // Label name, index
    clean_lines: Vec<Vec<String>>, // we store the clean lines that we generate in the prepass so we don't have to do it again
}

impl Lexer {
    // Constructor
    pub fn new() -> Self {
        Lexer {
            labels: Vec::new(),
            clean_lines: Vec::new(),
        }
    }

    pub fn label_prepass(&mut self, input: String){
        // Iterate through the input and find all the labels
        // Store them as well as their index
        let mut current_index = 0;
        for line in input.lines(){
            // First step is to clean up the line. 
            // This function will remove all comments and unnecessary whitespace
            // and return each token as a string in a vector
            let mut clean_line = cleanup_line(line);


            // Check for a token. This is defined as "LABEL:"
            // this must exist as the only token on the line
            // as otherwise it won't be picked up properly
            let len = clean_line.len();

            // Check if the line is empty
            // this helps us avoid empty lines
            // If there is garbage in the line, it will get picked up
            // later   
            if clean_line.len() > 0{
                if clean_line[0].contains(":"){
                    // This is a token
                    // Remove the : from the token
                    let token = clean_line[0].replace(":", "");
                    // Store the token and the current index
                    self.labels.push((token.to_uppercase(), current_index));
                    // Remove the token from the line
                    clean_line.remove(0);
                }else{
                    current_index += 1 * 4; // 4 bytes per instruction. Only increment if the line isn't empty
                    // and isn't a label
                }
            }

            if clean_line.len() == 0{
                // This is an empty line
                // We don't want to increment the index
                continue;
            }

            // Add the clean line to the clean lines vector
            self.clean_lines.push(clean_line);
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

            let mode: InstructionMode = InstructionMode::Register; // temp - we will change this once
            // we know what mode we are in

            // add the instruction to the line tokens
            line_tokens.push(Token::OpCode(instruction));

            // add the mode to the line tokens
            line_tokens.push(Token::Mode(mode));


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

            // If we've only got one item,
            // it's just an instruction with no parameters
            if len > 1{
                // Go through every parameter between the first and last 
                for item in &line[1..len-1]{
                    // Anything in between the first and last parameter
                    // will be a register
    
                    // remove the R
                    let item = item.replace("R", ""); 
                    let register = match utils::register_to_byte(&item){
                        Some(register) => register,
                        None => panic!("Invalid register: {}", item),
                    };
    
                    line_tokens.push(Token::Register(register));
                }
            
                // We need to check if the last parameter is a register, value or memory address
                // If it's a register, then we are in register mode
                // If it's a value, then we are in immediate mode
                // If it's a memory address, then we need to check if it's a register indirect or base offset

                let mut last_value = line[line.len() - 1].clone();

                // Check if the last value is a memory address
                if last_value.contains("[") && last_value.contains("]"){
                    last_value = last_value.replace("[", "").replace("]", "");
                    // This is a memory address
                    // Check if it's a register indirect or base offset. Check for a + sign
                    if last_value.contains("+"){
                        // This is a base offset
                        let split_value: Vec<&str> = last_value.split("+").collect();
                        let register = split_value[0];
                        let offset = split_value[1];
                        // Convert the register to a byte
                        let register = register.replace("R", "");
                        let register = match utils::register_to_byte(&register){
                            Some(register) => register,
                            None => panic!("Invalid register: {}", register),
                        };
                        // Convert the offset to a u32
                        let offset = match utils::string_to_u32(offset){
                            Some(offset) => offset,
                            None => panic!("Invalid offset: {}", offset),
                        };

                        line_tokens.push(Token::BaseOffset(register, offset)); // we store the offset so we can add it to the base register later
                        
                        // Change the mode
                        line_tokens[1] = Token::Mode(InstructionMode::BaseOffset);
                    }else{
                        // This is a register indirect
                        last_value = last_value.replace("R", "");
                        let register = match utils::register_to_byte(&last_value){
                            Some(register) => register,
                            None => panic!("Invalid register: {}", last_value),
                        };
                        line_tokens.push(Token::Register(register));

                        // Change the mode
                        line_tokens[1] = Token::Mode(InstructionMode::RegisterIndirect);
                    }
                }else{
                    // Check it's a value. We can do this by attempting to convert it to a u32
                    match utils::string_to_u32(&last_value){
                        Some(value) =>  {
                            line_tokens.push(Token::Value(value));
                            // Change the mode
                            line_tokens[1] = Token::Mode(InstructionMode::Immediate);
                        },
                        None => {
                            // We'll check if it's a register by attemping to convert it to a byte
                            let check_reg = last_value.replace("R", "");
                            match utils::register_to_byte(&check_reg){
                                Some(register) => {
                                    line_tokens.push(Token::Register(register));
                                    // Change the mode
                                    line_tokens[1] = Token::Mode(InstructionMode::Register);
                                },
                                None => {
                                    // We'll assume it's a label
                                    println!("Assuming {} is a label", last_value);
                                    line_tokens.push(Token::Label(last_value));
                                    // Change the mode
                                    line_tokens[1] = Token::Mode(InstructionMode::Immediate);
                                },
                            }


                        }
                    };
                }
            }

            // We now have all the tokens for this line
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
