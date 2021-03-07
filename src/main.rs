use std::collections::HashMap;
use std::fs;
use std::iter::repeat;

mod aes;
mod base64;
mod hex;
mod util;

use base64::{from_base64, to_base64};
use hex::{from_hex, to_hex};
use util::{fixed_xor, hamming_distance, transpose};

// TODO
// Should probably try to improve performance of score_string.
// Might be a good idea to figure out how to profile Rust.

fn score_string(s: &[u8]) -> usize {
    let expected: HashMap<u8, i32> = [
        (b' ', 130000),
        (b'E', 111607), (b'A', 84966), (b'R', 75809), (b'I', 75448),
        (b'O', 71635), (b'T', 69509), (b'N', 66544), (b'S', 57351),
        (b'L', 54893), (b'C', 45388), (b'U', 36308), (b'D', 33844),
        (b'P', 31671), (b'M', 30129), (b'H', 30034), (b'G', 24705),
        (b'B', 20720), (b'F', 18121), (b'Y', 17779), (b'W', 12899),
        (b'K', 11016), (b'V', 10074), (b'X', 02902), (b'Z', 02722),
        (b'J', 01965), (b'Q', 01962),
    ].iter().cloned().collect();

    let mut char_counts: HashMap<u8, usize> = HashMap::new();

    let mut non_ascii = 0;

    for c in s {
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

    let diff: i32 = expected
        .iter()
        .map(|(c, expectation)| {
            let &actual = char_counts.get(c).unwrap_or(&0);
            let rate: i32 = 100 * actual as i32 / s.len() as i32;
            (rate - expectation).abs()
        })
        .sum();

    diff as usize * 100 + non_ascii * 100
}

fn best_string<'a>(strings: &'a Vec<Vec<u8>>) -> &'a[u8] {
    strings
        .iter()
        .min_by_key(|s| score_string(s))
        .unwrap()
}

fn s1c1() {
    // Set 1 - Challenge 1
    let s = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("1-1: {}", to_base64(&from_hex(s)));
}

fn s1c2() {
    // Set 1 - Challenge 2
    let s = from_hex("1c0111001f010100061a024b53535009181c");
    let k = from_hex("686974207468652062756c6c277320657965");
    println!("1-2: {}", to_hex(&fixed_xor(&s, &k)));
}

fn s1c3() {
    // Set 1 - Challenge 3
    let results = (b'0'..b'Z').map(|k: u8| {
        let s: Vec<u8> = from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        let len = s.len();
        let key: Vec<u8> = repeat(k).take(len).collect();
        fixed_xor(&s, &key)
    }).collect();

    let result = best_string(&results);
    println!("1-3: {}", String::from_utf8(result.to_vec()).unwrap());
}

fn s1c4() {
    // Set 1 - Challenge 4
    let input = fs::read_to_string("resources/4.txt")
        .expect("Failed to read 4.txt");
    let keys: Vec<Vec<u8>> = (b'0'..b'Z')
        .map(move |k| repeat(k).take(60).collect())
        .collect();
    let results = input
        .lines()
        .map(from_hex)
        .map(|line| keys
             .iter()
             .map(|key| fixed_xor(&line, key))
             .collect::<Vec<Vec<u8>>>()
        )
        .flatten()
        .collect();
    let result = best_string(&results);
    println!("1-4: {}", String::from_utf8(result.to_vec()).unwrap().trim());
}

fn s1c5() {
    // Set 1 - Challenge 5
    let input = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key: Vec<u8> = "ICE".bytes().cycle().take(input.len()).collect();
    let result = fixed_xor(input, &key);
    println!("1-5: {}", to_hex(&result));
}

fn s1c6() {
    // Set 1 - Challenge 6
    let raw_input = fs::read_to_string("resources/6.txt")
        .expect("Failed to read 6.txt");
    let input = from_base64(&raw_input);

    let key_size: usize = (2..41)
        .min_by_key(|&key_size| {
            let mut i = 0;
            let mut offset = 0;
            let mut total_distance = 0;
            while offset < input.len() - 2*key_size {
                let chunk_1 = &input[0..(0+key_size)];
                let chunk_2 = &input[(offset+key_size)..(offset+2*key_size)];
                total_distance += 10000*hamming_distance(chunk_1, chunk_2);
                i += 1;
                offset += key_size;
            }
            let r = total_distance / key_size / i;
            r
        }).expect("Found no best key size");

    let chunks: Vec<Vec<u8>> = input
        .chunks(key_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    let key: Vec<u8> = transpose(chunks, b' ')
        .iter()
        .map(|chunk| {
            let mut results = vec![];
            for k in 0..128 {
                let key: Vec<u8> = repeat(k as u8)
                    .take(chunk.len())
                    .collect();
                results.push((k, fixed_xor(&chunk, &key)));
            }
            results.iter()
                .min_by_key(|(_, result)| score_string(result))
                .unwrap()
                .0
        })
        .collect();

    let repeated_key: Vec<u8> = key
        .iter()
        .cycle()
        .take(input.len())
        .map(|&x| x)
        .collect();

    let decrypted = fixed_xor(&input, &repeated_key);
    println!("1-6: {}", String::from_utf8(decrypted).unwrap());
}

fn main() {
    s1c1();
    s1c2();
    s1c3();
    s1c4();
    s1c5();
    s1c6();
}
