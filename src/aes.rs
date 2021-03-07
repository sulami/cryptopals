type Field = Vec<Vec<u8>>;


// fn expand_key() {
    
// }

// fn sub_bytes() {
    
// }

fn shift_rows(f: &mut Field) {
    for row in 1..4 {
        for _ in 0..row {
            let elm = f[row].remove(0);
            f[row].push(elm);
        }
    }
}

#[test]
fn shift_rows_test() {
    let mut input = vec![
        vec![1, 2, 3, 4],
        vec![3, 4, 5, 6],
        vec![7, 8, 9, 10],
        vec![11, 12, 13, 14],
    ];
    let expected = vec![
        vec![1, 2, 3, 4],
        vec![4, 5, 6, 3],
        vec![9, 10, 7, 8],
        vec![14, 11, 12, 13],
    ];
    shift_rows(&mut input);
    assert_eq!(expected, input);
}

// fn mix_columns() {
    
// }

// fn add_round_key() {
    
// }
