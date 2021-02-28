use std::collections::HashMap;
use std::iter::repeat;
use std::fs;

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

fn score_string(s: String) -> f32 {
    let expected: HashMap<char, f32> = [
        ('E', 11.1607), ('A', 8.4966), ('R', 7.5809), ('I', 7.5448),
        ('O', 7.1635), ('T', 6.9509), ('N', 6.6544), ('S', 5.7351),
        ('L', 5.4893), ('C', 4.5388), ('U', 3.6308), ('D', 3.3844),
        ('P', 3.1671), ('M', 3.0129), ('H', 3.0034), ('G', 2.4705),
        ('B', 2.0720), ('F', 1.8121), ('Y', 1.7779), ('W', 1.2899),
        ('K', 1.1016), ('V', 1.0074), ('X', 0.2902), ('Z', 0.2722),
        ('J', 0.1965), ('Q', 0.1962),
    ].iter().cloned().collect();

    let mut char_counts: HashMap<char, i32> = HashMap::new();

    let mut non_ascii = 0;

    for c in s.chars() {
        if c.is_ascii_alphanumeric() || c.is_ascii_whitespace() {
            let key = c.to_ascii_uppercase();
            match char_counts.get_mut(&key) {
                Some(count) => *count += 1,
                None => {
                    char_counts.insert(key, 1);
                    ()
                },
            }
        } else {
            non_ascii += 1;
        }
    }

    let diff: f32 = expected
        .iter()
        .map(|(c, expectation)| {
            let actual = char_counts.get(c).unwrap_or(&0);
            let rate = 100.0 * *actual as f32 / s.len() as f32;
            (rate - expectation).abs()
        })
        .sum();

    diff + non_ascii as f32
}

fn best_string(strings: Vec<Vec<u8>>) -> Vec<u8> {
    strings
        .iter()
        .min_by(|a, b| {
            let x = String::from_utf8(a.to_vec());
            let y = String::from_utf8(b.to_vec());
            match (x, y) {
                (Ok(s1), Ok(s2)) => {
                    match score_string(s1).partial_cmp(&score_string(s2)) {
                        Some(o) => o,
                        _ => core::cmp::Ordering::Equal,
                    }
                },
                (Ok(_), _) => core::cmp::Ordering::Less,
                (_, Ok(_)) => core::cmp::Ordering::Greater,
                _ => core::cmp::Ordering::Equal,
            }
        }
    )
        .unwrap()
        .clone()
}

fn s1c1() {
    // Set 1 - Challenge 1
    let s = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    println!("{}", hex_to_base64(s));
}

fn s1c2() {
    // Set 1 - Challenge 2
    let s = parse_hex_str(String::from("1c0111001f010100061a024b53535009181c"));
    let k = parse_hex_str(String::from("686974207468652062756c6c277320657965"));
    println!("{}", format_as_hex(fixed_xor(s, k)));
}

fn s1c3() {
    // Set 1 - Challenge 3
    let result = (65..90).map(|k: u8| {
        let s: Vec<u8> = parse_hex_str(String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"));
        let len = s.len();
        let key: Vec<u8> = repeat(k).take(len).collect();
        let result = String::from_utf8(fixed_xor(s, key));
        match result {
            Ok(r) => {
                let score = score_string(r.clone());
                (r, score)
            },
            _ => (String::new(), f32::MAX)
        }
    }).min_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap()).unwrap().0;
    println!("{}", result);
}

fn s1c4() {
    // Set 1 - Challenge 4
    let input = fs::read_to_string("resources/4.txt")
        .expect("Failed to read 4.txt");
    let results = input
        .lines()
        .map(|line| {
            let parsed = parse_hex_str(String::from(line));
            let mut results = vec![];
            for k in 1..123 {
                let key = repeat(k).take(60).map(|c| c as u8).collect();
                results.push(fixed_xor(parsed.clone(), key));
            }
            results
        })
        .flatten()
        .collect();
    println!("{}", String::from_utf8(best_string(results)).unwrap());
}

fn main() {
    s1c1();
    s1c2();
    s1c3();
    s1c4();
}
