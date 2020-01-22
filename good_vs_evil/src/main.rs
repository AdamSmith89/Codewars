fn main() {
    let s = good_vs_evil("2 25 3 56 1234 1000", "32 31 567 21 234 12 435");
    println!("{0}", s)
}

// Best practice example
// use std::cmp::Ordering::*;

// fn good_vs_evil(good: &str, evil: &str) -> String {
//   fn eval(group: &str, worth: &[u32]) -> u32 {
//     group.split_whitespace().enumerate()
//       .map(|(i, num)| worth[i] * num.parse::<u32>().unwrap())
//       .sum()
//   }

//   let good_worth = eval(good, &[ 1, 2, 3, 3, 4, 10 ]);
//   let evil_worth = eval(evil, &[ 1, 2, 2, 2, 3, 5, 10 ]);
//   String::from(
//     match good_worth.cmp(&evil_worth) {
//       Less => "Battle Result: Evil eradicates all trace of Good",
//       Equal => "Battle Result: No victor on this battle field",
//       Greater => "Battle Result: Good triumphs over Evil",
//     }
//   )
// }

fn good_vs_evil(good: &str, evil: &str) -> String {
    let good_total: i32 = get_army_value(good, [1,2,3,3,4,10].to_vec());
    let evil_total: i32 = get_army_value(evil, [1,2,2,2,3,5,10].to_vec());

    if good_total > evil_total {
        return String::from("Battle Result: Good triumphs over Evil");
    } else if evil_total > good_total {
        return String::from("Battle Result: Evil eradicates all trace of Good");
    }
    String::from("Battle Result: No victor on this battle field")
}

fn get_army_value(army: &str, unit_worth: Vec<i32>) -> i32 {
    let forces: Vec<i32> = army.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let forces_worth: Vec<i32> = forces.iter().zip(unit_worth).map(|(i1, i2)| i1 * i2).collect();
    forces_worth.iter().sum()
}

#[test]
fn returns_expected() {
    assert_eq!(
        good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"),
        "Battle Result: Good triumphs over Evil"
    );
    assert_eq!(
        good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"),
        "Battle Result: Evil eradicates all trace of Good"
    );
    assert_eq!(
        good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"),
        "Battle Result: No victor on this battle field"
    );
}
