fn main() {
    println!("Run tests!");
}

fn create_phone_number(numbers: &[u8]) -> String {
    // My answer
    format!(
        "({}) {}-{}",
        numbers[0..=2]
            .into_iter()
            .map(|x| x.to_string())
            .collect::<String>(),
        numbers[3..=5]
            .into_iter()
            .map(|x| x.to_string())
            .collect::<String>(),
        numbers[6..]
            .into_iter()
            .map(|x| x.to_string())
            .collect::<String>()
    )

    // A better answer
    // let s: String = numbers.into_iter().map(|x| x.to_string()).collect();
    // format!("({}) {}-{}", &s[..3], &s[3..6], &s[6..])
}

#[test]
fn returns_expected() {
    assert_eq!(
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
        "(123) 456-7890"
    );
    assert_eq!(
        create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
        "(111) 111-1111"
    );
    assert_eq!(
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]),
        "(123) 456-7899"
    );
}
