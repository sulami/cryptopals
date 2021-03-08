type Field = [u8; 16];

// fn expand_key() {
    
// }

// fn sub_bytes() {
    
// }

fn shift_rows(field: &mut Field) {
    let [
        a, b, c, d,
        e, f, g, h,
        i, j, k, l,
        m, n, o, p,
    ] = field;
    *field = [
        *a, *b, *c, *d,
        *f, *g, *h, *e,
        *k, *l, *i, *j,
        *p, *m, *n, *o,
    ]
}

#[test]
fn shift_rows_test() {
    let mut input = [
        1, 2, 3, 4,
        3, 4, 5, 6,
        7, 8, 9, 10,
        11, 12, 13, 14,
    ];
    let expected = [
        1, 2, 3, 4,
        4, 5, 6, 3,
        9, 10, 7, 8,
        14, 11, 12, 13,
    ];
    shift_rows(&mut input);
    assert_eq!(expected, input);
}

// fn mix_columns() {
    
// }

// fn add_round_key() {
    
// }
