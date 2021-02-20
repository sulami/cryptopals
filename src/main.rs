fn parse_hex(c: char) -> u8 {
    u8::from_str_radix(c.to_string().as_str(), 16).unwrap()
}

fn format_as_base64(input: Vec<u8>) -> String {
    let base64_table
        = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
        .as_bytes();
    input
        .iter()
        .map(|n| {
            base64_table[usize::from(*n)] as char
        })
        .collect::<String>()
}

fn hex_to_base64(input: String) -> String {
    let bs: Vec<u8> = input
        .chars()
        .map(parse_hex)
        .collect::<Vec<u8>>()
        .chunks(3)
        .map(|chunk| {
            let first = (chunk[0] << 2) | (chunk[1] >> 2);
            let second = ((chunk[1] & 0b00000011) << 4) | chunk[2];
            vec![first % 64, second % 64]
        })
        .flatten()
        .collect();
    format_as_base64(bs)
}

fn main() {
    // Set 1 - Challenge 1
    let s = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    println!("{}", hex_to_base64(s));
}
