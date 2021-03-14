use std::convert::TryInto;

type State = [u8; 16];
type Key = [u32; 4];

const ROUND_CONSTANTS: [u32; 10] = [
    0x01000000,
    0x02000000,
    0x04000000,
    0x08000000,
    0x10000000,
    0x20000000,
    0x40000000,
    0x80000000,
    0x1B000000,
    0x36000000,
];

const S_BOX: [u8; 256] = [
    //         0     1     2     3     4     5     6     7     8     9     a     b     c     d     e     f
    /* 0 */ 0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    /* 1 */ 0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    /* 2 */ 0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    /* 3 */ 0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    /* 4 */ 0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    /* 5 */ 0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    /* 6 */ 0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    /* 7 */ 0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    /* 8 */ 0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    /* 9 */ 0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    /* a */ 0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    /* b */ 0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    /* c */ 0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    /* d */ 0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    /* e */ 0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    /* f */ 0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

fn sub_bytes(word: u32) -> u32 {
    // Each 32-bit word is broken up into 4 8-bit bytes.
    // For each byte, the four high bits determine the row, the low
    // four bits the column in the S-box above.
    let mut bs: [u8; 4] = word.to_be_bytes();
    for i in 0..4 {
        let first_four_bits = bs[i] >> 4;
        let last_four_bits = bs[i] & 0x0f;
        let idx = first_four_bits * 16 + last_four_bits;
        bs[i] = S_BOX[idx as usize];
    }
    u32::from_be_bytes(bs)
}

#[test]
fn sub_bytes_test() {
    assert_eq!(0xd4e0b81e, sub_bytes(0x19a09ae9));
}

fn expand_key(key: &Key) -> Vec<Key> {
    // the first key is the original key
    // from then for each key:
    //   for the first word:
    //   - rotate the previous word left by 1 byte
    //   - sub the it through the s-box
    //   - xor with the first word from the previous key
    //   - xor with the round constant for this round
    //   for the remaining three words:
    //   - xor the previous word with the corresponding word from the
    //     previous key (2-4)
    // repeat until you have 11 128-bit keys (4 * 32 bits each)
    let mut keys: Vec<Vec<u32>> = vec![key.to_vec()];
    for k in 1..11 {
        let first_word =
            sub_bytes(keys[k-1][3].rotate_left(8))
            ^ keys[k-1][0]
            ^ ROUND_CONSTANTS[k-1];
        keys.push(vec![first_word]);
        for i in 1..4 {
            let previous_word = keys[k][i-1];
            let previous_key_word = keys[k-1][i];
            keys[k].push(previous_word ^ previous_key_word);
        }
    }
    keys.iter()
        .map(|k| k.as_slice().try_into().expect("Invalid key") )
        .collect()
}

#[test]
fn expand_key_test() {
    let start_key: Key = [0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c];
    let keys = expand_key(&start_key);
    assert_eq!(11, keys.len());
    // Second key set.
    assert_eq!(0xa0fafe17, keys[1][0]);
    assert_eq!(0x88542cb1, keys[1][1]);
    assert_eq!(0x23a33939, keys[1][2]);
    assert_eq!(0x2a6c7605, keys[1][3]);
    // Very last one.
    assert_eq!(0xb6630ca6, keys[10][3]);
}

fn shift_rows(state: &mut State) {
    let [
        a, b, c, d,
        e, f, g, h,
        i, j, k, l,
        m, n, o, p,
    ] = state;
    *state = [
        *a, *b, *c, *d,
        *f, *g, *h, *e,
        *k, *l, *i, *j,
        *p, *m, *n, *o,
    ]
}

#[test]
fn shift_rows_test() {
    let mut input = [
        0xd4, 0xe0, 0xb8, 0x1e,
        0x27, 0xbf, 0xb4, 0x41,
        0x11, 0x98, 0x5d, 0x52,
        0xae, 0xf1, 0xe5, 0x30,
    ];
    let expected = [
        0xd4, 0xe0, 0xb8, 0x1e,
        0xbf, 0xb4, 0x41, 0x27,
        0x5d, 0x52, 0x11, 0x98,
        0x30, 0xae, 0xf1, 0xe5,
    ];
    shift_rows(&mut input);
    assert_eq!(expected, input);
}

fn mix_columns(state: &mut State) {
    for ncol in 0..4 {
        let column = [
            state[0*4+ncol],
            state[1*4+ncol],
            state[2*4+ncol],
            state[3*4+ncol],
        ];
        let mut double_column = column;
        for nrow in 0..4 {
            // Double each value, clever hack from Wikipedia.
            let h = (column[nrow] >> 7) & 0b00000001;
            double_column[nrow] = (column[nrow] << 1) ^ (h * 0x1b);
        }
        state[0*4+ncol] = double_column[0] ^ column[3] ^ column[2] ^ double_column[1] ^ column[1];
        state[1*4+ncol] = double_column[1] ^ column[0] ^ column[3] ^ double_column[2] ^ column[2];
        state[2*4+ncol] = double_column[2] ^ column[1] ^ column[0] ^ double_column[3] ^ column[3];
        state[3*4+ncol] = double_column[3] ^ column[2] ^ column[1] ^ double_column[0] ^ column[0];
    }
}

#[test]
fn mix_columns_test() {
    let mut input = [
        0xd4, 0xe0, 0xb8, 0x1e,
        0xbf, 0xb4, 0x41, 0x27,
        0x5d, 0x52, 0x11, 0x98,
        0x30, 0xae, 0xf1, 0xe5,
    ];
    let expected = [
        0x04, 0xe0, 0x48, 0x28,
        0x66, 0xcb, 0xf8, 0x06,
        0x81, 0x19, 0xd3, 0x26,
        0xe5, 0x9a, 0x7a, 0x4c,
    ];
    mix_columns(&mut input);
    assert_eq!(expected, input);
}

fn add_round_key(state: &mut State, key: Key) {
    for ncol in 0..4 {
        let key_words: [u8; 4] = key[ncol].to_be_bytes();
        let column = [
            state[0*4+ncol],
            state[1*4+ncol],
            state[2*4+ncol],
            state[3*4+ncol],
        ];
        (0..4).for_each(|i| state[i*4+ncol] = column[i] ^ key_words[i]);
    }
}

#[test]
fn add_round_key_test() {
    let mut input = [
        0x04, 0xe0, 0x48, 0x28,
        0x66, 0xcb, 0xf8, 0x06,
        0x81, 0x19, 0xd3, 0x26,
        0xe5, 0x9a, 0x7a, 0x4c,
    ];
    let key: Key = [0xa0fafe17, 0x88542cb1, 0x23a33939, 0x2a6c7605];
    let expected = [
        0xa4, 0x68, 0x6b, 0x02,
        0x9c, 0x9f, 0x5b, 0x6a,
        0x7f, 0x35, 0xea, 0x50,
        0xf2, 0x2b, 0x43, 0x49,
    ];
    add_round_key(&mut input, key);
    assert_eq!(expected, input);
}

#[allow(dead_code)]
pub fn encrypt(input: State, key: Key) -> State {
    let mut state = input;
    let keys = expand_key(&key);

    add_round_key(&mut state, keys[0]);

    for round in 1..10 {
        for column in 0..4 {
            let new_column = sub_bytes(
                u32::from_be_bytes([
                    state[0*4+column],
                    state[1*4+column],
                    state[2*4+column],
                    state[3*4+column],
                ])
            ).to_be_bytes();
            (0..4).for_each(|i| state[i*4+column] = new_column[i]);
        }
        shift_rows(&mut state);
        mix_columns(&mut state);
        add_round_key(&mut state, keys[round]);
    }

    for column in 0..4 {
        let new_column = sub_bytes(
            u32::from_be_bytes([
                state[0*4+column],
                state[1*4+column],
                state[2*4+column],
                state[3*4+column],
            ])
        ).to_be_bytes();
        (0..4).for_each(|i| state[i*4+column] = new_column[i]);
    }
    shift_rows(&mut state);
    add_round_key(&mut state, keys[10]);
    state
}

#[test]
fn encrypt_test() {
    let input = [
        0x32, 0x88, 0x31, 0xe0,
        0x43, 0x5a, 0x31, 0x37,
        0xf6, 0x30, 0x98, 0x07,
        0xa8, 0x8d, 0xa2, 0x34,
    ];
    let key: Key = [0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c];
    let expected = [
        0x39, 0x02, 0xdc, 0x19,
        0x25, 0xdc, 0x11, 0x6a,
        0x84, 0x09, 0x85, 0x0b,
        0x1d, 0xfb, 0x97, 0x32,
    ];
    assert_eq!(expected, encrypt(input, key));
}
