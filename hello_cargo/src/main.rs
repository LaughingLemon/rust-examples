
fn main() {
    let v = vec!(1, 3, 7, 3, 12, 54, 34, 67, 2);

    let largest = largest_integer(&v);

    println!("largest is {}", largest);

    let v = vec!('e','G', 'b', 'L');

    let largest = largest_character(&v);

    println!("largest is {}", largest);
}

fn largest_integer(v: &[i32]) -> i32 {
    let mut largest = v[0];

    for &i in v.iter() {
        if i > largest {
            largest = i;
        }
    }
    largest
}


fn largest_character(v: &[char]) -> char {
    let mut largest = v[0];

    for &i in v.iter() {
        if i > largest {
            largest = i;
        }
    }
    largest
}