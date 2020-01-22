fn main() {
    println!("Run the tests!");
}

#[allow(dead_code)]
fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    // Both are the same length so both must be empty, shortcut rest of the logic
    if a.is_empty() {
        return true;
    }

    let mut actual_squares: Vec<_> = a.iter().map(|x| x*x).collect();
    actual_squares.sort();
    
    let mut expected_squares = b;
    expected_squares.sort();

    actual_squares == expected_squares
}

#[test]
fn different_lengths_returns_false() {
    let a1 = vec![];
    let a2 = vec![1, 2, 3, 4];
    assert_eq!(comp(a1, a2), false);

    let a1 = vec![1, 2, 3, 4];
    let a2 = vec![];
    assert_eq!(comp(a1, a2), false);
}

#[test]
fn both_empty_returns_true() {
    let a1 = vec![];
    let a2 = vec![];
    assert_eq!(comp(a1, a2), true);
}

#[test]
fn basic_success() {
    let a1 = vec![2, 3, 4];
    let a2 = vec![2 * 2, 3 * 3, 4 * 4];

    assert_eq!(comp(a1, a2), true);
}

#[test]
fn basic_out_of_order_success() {
    let a1 = vec![2, 3, 4];
    let a2 = vec![3 * 3, 2 * 2, 4 * 4];

    assert_eq!(comp(a1, a2), true);
}

#[test]
fn basic_fail() {
    let a1 = vec![2, 3, 4];
    let a2 = vec![5, 6, 7];

    assert_eq!(comp(a1, a2), false);
}

#[test]
fn massive_number_success() {
    let a1: Vec<i64> = [3032000498, 1032070398, 303200].to_vec();
    let a2: Vec<i64> = [
        3032000498 * 3032000498,
        1032070398 * 1032070398,
        303200 * 303200,
    ]
    .to_vec();

    assert_eq!(comp(a1, a2), true);
}

#[test]
fn codewars_tests() {
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![
        11 * 11,
        121 * 121,
        144 * 144,
        19 * 19,
        161 * 161,
        19 * 19,
        144 * 144,
        19 * 19,
    ];
    assert_eq!(comp(a1, a2), true);

    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![
        11 * 21,
        121 * 121,
        144 * 144,
        19 * 19,
        161 * 161,
        19 * 19,
        144 * 144,
        19 * 19,
    ];
    assert_eq!(comp(a1, a2), false);

    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![121, 14641, 20736, 36100, 25921, 361, 20736, 361];
    assert_eq!(comp(a1, a2), false);

    let a1 = vec![2, 2, 3];
    let a2 = vec![4, 9, 9];
    assert_eq!(comp(a1, a2), false);
}
