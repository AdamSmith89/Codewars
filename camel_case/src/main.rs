fn main() {
    println!("Run Tests!");

    //use cargo::ops::new;
    cargo::ops::new()
}

#[allow(dead_code)]
fn camel_case(str: &str) -> String {
    let iter = str.split_whitespace();
    let mut capitalised_words: Vec<String> = Vec::new();

    for word in iter {        
        let mut chars: Vec<char> = word.chars().collect();
        chars[0] = chars[0].to_uppercase().nth(0).unwrap();

        capitalised_words.push(chars.into_iter().collect());
    }

    capitalised_words.join("")
}

// Best-practice / clever from Codewars
// fn camel_case(str: &str) -> String {
//     str.split_whitespace()
//         .map(|s| [&s[..1].to_uppercase(), &s[1..]].join(""))
//         .collect()
// }

#[test]
fn codewars_tests() {
  assert_eq!(camel_case("test case"), "TestCase");
  assert_eq!(camel_case("camel case method"), "CamelCaseMethod");
  assert_eq!(camel_case("say hello "), "SayHello");
  assert_eq!(camel_case(" camel case word"), "CamelCaseWord");
  assert_eq!(camel_case(""), "");
}