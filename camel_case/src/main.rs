fn main() {
    println!("Run Tests!");
}

#[allow(dead_code)]
fn camel_case(str: &str) -> String {
    let iter = str.split_whitespace();

    for word in iter {
        println!("{}", word);
        
        let mut chars: Vec<char> = word.chars().collect();
        chars[0] = chars[0].to_uppercase().;
    }

    str.to_string()
}

#[test]
fn codewars_tests() {
  assert_eq!(camel_case("test case"), "TestCase");
  assert_eq!(camel_case("camel case method"), "CamelCaseMethod");
  assert_eq!(camel_case("say hello "), "SayHello");
  assert_eq!(camel_case(" camel case word"), "CamelCaseWord");
  assert_eq!(camel_case(""), "");
}