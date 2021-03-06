use std::collections::HashMap;
use std::fs;
use std::iter::repeat;

fn from_hex_char(c: char) -> u8 {
    u8::from_str_radix(c.to_string().as_str(), 16)
        .expect("Failed to parse hex char")
}

fn from_hex(s: &str) -> Vec<u8> {
    s.chars()
        .map(from_hex_char)
        .collect::<Vec<u8>>()
        .chunks(2)
        .map(|chunk| (chunk[0] << 4) | chunk[1])
        .collect()
}

fn to_hex(input: &[u8]) -> String {
    input.iter()
        .map(|c| format!("{:02x}", c))
        .collect()
}

const BASE64_TABLE: &[u8]
    = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn from_base64(s: &str) -> Vec<u8> {
    let mut padding = 0;
    let mut result: Vec<u8> = s
        .bytes()
        .filter(|&b| b != b' ')
        .filter(|&b| b != b'\n')
        .map(|b| {
            if b == b'=' {
                padding += 1;
                0
            } else {
                let mut idx = 0;
                while idx < BASE64_TABLE.len() {
                    if b == BASE64_TABLE[idx] {
                        break
                    }
                    idx += 1;
                }
                idx as u8
            }
        })
        .collect::<Vec<u8>>()
        .chunks(4)
        .map(|chunk| {
            // aaaaaabbbbbbccccccdddddd
            // aaaaaaaabbbbbbbbcccccccc
            let first = chunk[0] << 2 | chunk[1] >> 4;
            let second = chunk[1] << 4 | chunk[2] >> 2;
            let third = chunk[2] << 6 | chunk[3];
            vec![first, second, third]
        })
        .flatten()
        .collect();
    result.truncate(result.len() - padding);
    result
}

#[test]
fn from_base64_test() {
    assert_eq!("any carnal pleas", from_base64("YW55IGNhcm5hbCBwbGVhcw==")
               .iter()
               .map(|&b| b as char)
               .collect::<String>());
    assert_eq!("any carnal pleasu", from_base64("YW55IGNhcm5hbCBwbGVhc3U=")
               .iter()
               .map(|&b| b as char)
               .collect::<String>());
    assert_eq!("any carnal pleasur", from_base64("YW55IGNhcm5hbCBwbGVhc3Vy")
               .iter()
               .map(|&b| b as char)
               .collect::<String>());
    assert_eq!("any carnal pleasure", from_base64("YW55IGNhcm5hbCBwbGVhc3VyZQ==")
               .iter()
               .map(|&b| b as char)
               .collect::<String>());
    assert_eq!("any carnal pleasure.", from_base64("YW55IGNhcm5hbCBwbGVhc3VyZS4=")
               .iter()
               .map(|&b| b as char)
               .collect::<String>());
}

fn to_base64(input: &[u8]) -> String {
    let mut input = input.to_vec();
    let padding = input.len() % 3;
    // Pad the bytes so that we have a number divisible by 3. The last
    // chunk might have some zeroes.
    if 0 < padding {
        for _ in padding..3 {
            input.push(0);
        }
    };
    let mut result: Vec<u8> = input
        .chunks(3)
        .map(|chunk| {
            // aaaaaaaabbbbbbbbcccccccc
            // aaaaaabbbbbbccccccdddddd
            let first = chunk[0] >> 2;
            let second = (chunk[0] & 0b00000011) << 4 | chunk[1] >> 4;
            let third = (chunk[1] & 0b00001111) << 2 | chunk[2] >> 6;
            let fourth = chunk[2] & 0b00111111;
            vec![first % 64, second % 64, third % 64, fourth % 64]
        })
        .flatten()
        .map(|n| {
            BASE64_TABLE[n as usize]
        })
        .collect();
    // Add padding to the end, where we had filled in zeroes.
    // Unintuitively, this is backwards, if we filled in two bytes of
    // padding, we write out one padding character.
    let l = result.len();
    if padding == 2 {
        result[l-1] = b'=';
    } else if padding == 1 {
        result[l-1] = b'=';
        result[l-2] = b'=';
    }
    String::from_utf8(result).unwrap()
}

#[test]
fn to_base64_test() {
    assert_eq!("YW55IGNhcm5hbCBwbGVhc3VyZS4=", to_base64(b"any carnal pleasure."));
    assert_eq!("YW55IGNhcm5hbCBwbGVhc3VyZQ==", to_base64(b"any carnal pleasure"));
    assert_eq!("YW55IGNhcm5hbCBwbGVhc3Vy", to_base64(b"any carnal pleasur"));
    assert_eq!("YW55IGNhcm5hbCBwbGVhc3U=", to_base64(b"any carnal pleasu"));
    assert_eq!("YW55IGNhcm5hbCBwbGVhcw==", to_base64(b"any carnal pleas"));
}

fn fixed_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter()
        .zip(b)
        .map(|(x, y)| x ^ y)
        .collect()
}

fn hamming_distance(s1: &[u8], s2: &[u8]) -> usize {
    s1.iter()
        .zip(s2)
        .map(|(a, b)| (*a ^ *b).count_ones())
        .sum::<u32>() as usize
}

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

fn transpose<T>(v: Vec<Vec<T>>, fill: T) -> Vec<Vec<T>> where T: Clone {
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter()
             .map(|inner| {
                 if i < inner.len() {
                     inner[i].clone()
                 } else {
                     fill.clone()
                 }
             })
             .collect::<Vec<T>>())
        .collect()
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
    let results = input
        .lines()
        .map(from_hex)
        .map(|line| {
            let mut decrypted = vec![];
            for k in b'0'..b'Z' {
                let key: Vec<u8> = repeat(k)
                    .take(line.len())
                    .collect();
                decrypted.push(fixed_xor(&line, &key));
            }
            decrypted
        })
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
    println!("1-6: 37? {}", hamming_distance(b"this is a test", b"wokka wokka!!!"));

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
    println!("{}", String::from_utf8(decrypted).unwrap());
}

fn main() {
    s1c1();
    s1c2();
    s1c3();
    s1c4();
    s1c5();
    s1c6();
}
