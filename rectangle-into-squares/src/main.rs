fn main() {
    println!("Run the tests!");
}

fn sq_in_rect(length: i32, width: i32) -> Option<Vec<i32>> {
    if length == width {
        return None;
    }

    let mut side1 = length;
    let mut side2 = width;
    let mut squares: Vec<i32> = Vec::new();

    while side1 != side2 {
        let min = std::cmp::min(side1, side2);
        squares.push(min);
        side1 = (side1 - side2).abs();
        side2 = min;
    }

    squares.push(side1);

    return Some(squares);
}

fn testing(lng: i32, wdth: i32, exp: Option<Vec<i32>>) -> () {
    assert_eq!(sq_in_rect(lng, wdth), exp)
}

#[test]
fn squares_return_none() {
    assert_eq!(sq_in_rect(1, 1), None);
    assert_eq!(sq_in_rect(5, 5), None);
    assert_eq!(sq_in_rect(25, 25), None);
}

#[test]
fn rectangles_return_some() {

    testing(2, 1, Some(vec![1, 1]));
    testing(1, 2, Some(vec![1, 1]));
    testing(5, 3, Some(vec![3, 2, 1, 1]));
    testing(3, 5, Some(vec![3, 2, 1, 1]));
    testing(5, 4, Some(vec![4, 1, 1, 1, 1]));
    testing(7, 4, Some(vec![4, 3, 1, 1, 1]));
    testing(8, 6, Some(vec![6, 2, 2, 2]));
    testing(10, 4, Some(vec![4, 4, 2, 2]));
    testing(20, 14, Some(vec![14, 6, 6, 2, 2, 2]));
    // 20, 14  : push(14); 20 - 14 = 6
    // 14, 6   : push(6);  14 - 6  = 8
    // 6, 8    : push(6);  8 - 6   = 2
    // 6, 2    : push(2);  6 - 2   = 4
    // 2, 4    : push(2);  4 - 2   = 2
    // 2, 2    : push(2);  end

    // 5, 3    : push(3);  5 - 3   = 2
    // 3, 2    : push(2);  3 - 2   = 1
    // 2, 1    : push(1);  2 - 1   = 1
    // 1, 1    : push(1);  end
}
