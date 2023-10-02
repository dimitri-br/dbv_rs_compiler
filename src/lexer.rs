use crate::Tokens;

pub struct Lexer {
    input: String,
    pub labels: Vec<(String, usize)>, // Label name, index
}

impl Lexer {
    // Constructor
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.to_string(),
            labels: Vec::new(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Vec<Tokens>> {
        let mut tokens = Vec::new();
        let mut current_index = 0;

        for line in self.input.lines() {
            let mut line_vec = Vec::new();
            let words: Vec<&str> = line.split_whitespace().collect();
            
            for word in words {
                if word.starts_with(";") {
                    break;
                }

                let word = if word.ends_with(",") {
                    &word[..word.len()-1]
                } else {
                    word
                };

                match word.to_uppercase().as_str() {
                    w if Tokens::try_from(w.to_string()).is_ok() => {
                        line_vec.push(Tokens::try_from(w.to_string()).unwrap());
                        current_index += 1;
                    },
                    w if w.starts_with("R_") => line_vec.push(Tokens::Register(w[2..].to_string())),
                    w if w.starts_with("F_R_") => line_vec.push(Tokens::Register(w[4..].to_string())),
                    w if w.ends_with(":") => {
                        let label = w.replace(":", "");
                        if self.labels.iter().any(|(lbl, _)| lbl == &label) {
                            panic!("Label {} already exists.", label);
                        }
                        self.labels.push((label.to_string(), current_index));
                    },
                    w => {
                        // Address logic
                        if line_vec.len() == 3 && (
                            matches!(&line_vec[0], &Tokens::IF | &Tokens::IFF | &Tokens::IFN | &Tokens::IFNF | &Tokens::IFR | &Tokens::IFRF | &Tokens::IFNR | &Tokens::IFNRF)) {
                            line_vec.push(Tokens::Address(w.to_string()));
                        } else if line_vec.len() == 2 && matches!(&line_vec[0], &Tokens::SEX) {
                            line_vec.push(Tokens::Address(w.to_string()));
                        } else if line_vec.len() == 1 && matches!(&line_vec[0], &Tokens::JMP | &Tokens::JMPR | &Tokens::JNZR | &Tokens::JNZF | &Tokens::JNZRF) {
                            line_vec.push(Tokens::Address(w.to_string()));
                        } else if line_vec.len() == 2 && matches!(&line_vec[0], &Tokens::JNZ) {
                            line_vec.push(Tokens::Address(w.to_string()));
                        } else if line_vec.len() == 1 && matches!(line_vec[0], Tokens::SD) {
                            line_vec.push(Tokens::Address(w.to_string()));
                        } else if line_vec.len() == 2 && matches!(line_vec[0], Tokens::LD) {
                            line_vec.push(Tokens::Address(w.to_string()));
                        } 
                        // Value logic
                        else if w.starts_with("0X") {
                            if let Ok(val) = u32::from_str_radix(&w[2..], 16) {
                                line_vec.push(Tokens::Value(val));
                            } else {
                                panic!("Err - Invalid hex value");
                            }
                        } else if w.starts_with("0B") {
                            if let Ok(val) = u32::from_str_radix(&w[2..], 2) {
                                line_vec.push(Tokens::Value(val));
                            } else {
                                panic!("Err - Invalid binary value");
                            }
                        } else if let Ok(val) = w.parse::<u32>() {
                            line_vec.push(Tokens::Value(val));
                        } else if w.contains(".") {
                            // Float -> check that it's a valid float
                            println!("Float: {}", w);
                            if let Ok(val) = w.parse::<f32>() {
                                line_vec.push(Tokens::Value(val.to_bits() as u32));
                            } else {
                                panic!("Err - Invalid float value");
                            }
                        }
                    },                    
                }
            }

            if !line_vec.is_empty() {
                tokens.push(line_vec);
            }
        }

        tokens
    }
}
