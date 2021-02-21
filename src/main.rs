fn parse_hex(c: char) -> u8 {
    u8::from_str_radix(c.to_string().as_str(), 16)
        .expect("Failed to parse hex char")
}

/// Parses a string of hex-pairs into u8s.
fn parse_hex_str(s: String) -> Vec<u8> {
    s.chars()
        .map(parse_hex)
        .collect::<Vec<u8>>()
        .chunks(2)
        .map(|chunk| (chunk[0] << 4) | chunk[1])
        .collect()
}

fn to_base64(input: Vec<u8>) -> String {
    let base64_table
        = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
        .as_bytes();
    input
        .iter()
        .map(|n| {
            base64_table[*n as usize] as char
        })
        .collect()
}

fn format_as_hex(input: Vec<u8>) -> String {
    input.iter().map(|c| format!("{:x}", c)).collect()
}

fn hex_to_base64(input: String) -> String {
    let bs: Vec<u8> = parse_hex_str(input)
        .chunks(3)
        .map(|chunk| {
            let first = chunk[0] >> 2;
            let second = ((chunk[0] & 0b00000011) << 4) | (chunk[1] >> 4);
            let third = ((chunk[1] & 0b00001111) << 2) | (chunk[2] >> 6);
            let fourth = chunk[2] & 0b00111111;
            vec![first % 64, second % 64, third % 64, fourth % 64]
        })
        .flatten()
        .collect();
    to_base64(bs)
}

fn fixed_xor(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    a.iter()
        .zip(b)
        .map(|(x, y)| x ^ y)
        .collect()
}

fn main() {
    // Set 1 - Challenge 1
    let s = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    println!("{}", hex_to_base64(s));

    // Set 1 - Challenge 2
    let s = parse_hex_str(String::from("1c0111001f010100061a024b53535009181c"));
    let k = parse_hex_str(String::from("686974207468652062756c6c277320657965"));
    println!("{}", format_as_hex(fixed_xor(s, k)));
}
