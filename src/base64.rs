const BASE64_TABLE: &[u8]
    = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn from_base64(s: &str) -> Vec<u8> {
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

pub fn to_base64(input: &[u8]) -> String {
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
