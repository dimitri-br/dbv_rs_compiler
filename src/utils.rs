#[macro_export]
macro_rules! enum_conv_gen {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<usize> for $name {
            type Error = ();

            fn try_from(v: usize) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as usize => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }

        impl std::convert::TryFrom<String> for $name {
            type Error = ();

            fn try_from(v: String) -> Result<Self, Self::Error> {
                match v.to_uppercase().as_str() {
                    $(x if x == stringify!($vname).to_uppercase() => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }

        impl std::convert::From<$name> for String {
            fn from(v: $name) -> Self {
                match v {
                    $($name::$vname => stringify!($vname).to_string(),)*
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Parameter{
    pub value: u32,
}

impl Parameter{
    pub fn get_value(&self) -> u32{
        self.value
    }
}

pub fn register_to_byte(reg: &str) -> Option<u32> {
    let value = reg.to_uppercase();
    match value.as_str() {
        "A" => Some(0x0),
        "B" => Some(0x1),
        "C" => Some(0x2),
        "D" => Some(0x3),
        "E" => Some(0x4),
        "F" => Some(0x5),
        "G" => Some(0x6),
        "H" => Some(0x7),
        "I" => Some(0x8),
        "J" => Some(0x9),
        "K" => Some(0xA),
        "L" => Some(0xB),
        "M" => Some(0xC),
        "N" => Some(0xD),
        "O" => Some(0xE),
        "P" => Some(0xF),
        _ => None,
    }
}

pub fn cleanup_line(line: &str) -> Vec<String> {
    let mut cleaned_line = String::new();
    // This is an asm thing. First thing to do, is to remove all comments
    // Read the line up until a comment is found (;)
    for c in line.chars() {
        if c == ';' {
            break;
        }
        cleaned_line.push(c);
    }

    // Now we need to remove all commas. They are not needed in the assembly,
    // but can help with readability
    cleaned_line = cleaned_line.replace(",", "");

    // Now split the line into tokens. We split the line by spaces
    // This will give us a vector of tokens
    let tokens: Vec<&str> = cleaned_line.split(" ").collect();

    // For each token, we need to remove all whitespace surrounding it
    // This is done by trimming the token
    let mut cleaned_tokens = Vec::new();
    for token in tokens {
        cleaned_tokens.push(token.trim().to_string());
    }

    // We should now have a vector of tokens, with no whitespace surrounding them
    // This is what we need to tokenize the line

    // Remove any empty items ("")
    cleaned_tokens.retain(|x| x != "");

    cleaned_tokens
}

pub fn string_to_u32(value: &str) -> Option<u32>{
    let value = value.to_lowercase();
    // Check hex
    if value.contains("0x"){
        let value = value.replace("0x", "");

        let value = match u32::from_str_radix(&value, 16){
            Ok(value) => Some(value),
            Err(_) => None,
        };

        value
    }else if value.contains("0b"){
        let value = value.replace("0b", "");

        let value = match u32::from_str_radix(&value, 2){
            Ok(value) => Some(value),
            Err(_) => None,
        };

        value
    }else{
        // Decimal
        let value = match value.parse::<u32>(){
            Ok(value) => Some(value),
            Err(_) => None,
        };

        value
    }
}
