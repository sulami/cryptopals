fn from_hex_char(c: char) -> u8 {
    u8::from_str_radix(c.to_string().as_str(), 16)
        .expect("Failed to parse hex char")
}

pub fn from_hex(s: &str) -> Vec<u8> {
    s.chars()
        .map(from_hex_char)
        .collect::<Vec<u8>>()
        .chunks(2)
        .map(|chunk| (chunk[0] << 4) | chunk[1])
        .collect()
}

pub fn to_hex(input: &[u8]) -> String {
    input.iter()
        .map(|c| format!("{:02x}", c))
        .collect()
}
