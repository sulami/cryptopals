pub fn fixed_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter()
        .zip(b)
        .map(|(x, y)| x ^ y)
        .collect()
}

pub fn hamming_distance(s1: &[u8], s2: &[u8]) -> usize {
    s1.iter()
        .zip(s2)
        .map(|(a, b)| (*a ^ *b).count_ones())
        .sum::<u32>() as usize
}

#[test]
fn hamming_distance_test() {
    assert_eq!(37, hamming_distance(b"this is a test", b"wokka wokka!!!"));
}

pub fn transpose<T>(v: Vec<Vec<T>>, fill: T) -> Vec<Vec<T>> where T: Clone {
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
