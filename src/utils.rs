pub fn register_to_byte(reg: &str) -> u8 {
    match reg {
        "A" => 0x0,
        "B" => 0x1,
        "C" => 0x2,
        "D" => 0x3,
        "E" => 0x4,
        "F" => 0x5,
        _ => panic!("Invalid register"),
    }
}

pub fn break_value_into_bytes(val: u32) -> Vec<u8> {
    vec![
        ((val & 0xFF000000) >> 24) as u8,
        ((val & 0x00FF0000) >> 16) as u8,
        ((val & 0x0000FF00) >> 8) as u8,
        (val & 0x000000FF) as u8
    ]
}

pub fn bytes_to_u32(bytes: &[u8]) -> u32 {
    let mut val: u32 = 0;
    for (i, byte) in bytes.iter().enumerate() {
        val |= (*byte as u32) << (8 * (3 - i));
    }
    val
}

pub fn parse_address(addr: &str) -> Vec<u8> {
    let val = if addr.starts_with("0X") {
        u16::from_str_radix(&addr[2..], 16)
    } else if addr.starts_with("0B") {
        u16::from_str_radix(&addr[2..], 2)
    } else {
        addr.parse::<u16>()
    }.expect("Err - Jump address invalid");

    vec![((val & 0xFF00) >> 8) as u8, (val & 0x00FF) as u8]
}

