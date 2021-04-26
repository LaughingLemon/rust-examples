
fn main() {
    let v = vec!(1, 3, 7, 3, 12, 54, 34, 67, 2);

    let largest = find_largest(&v);

    println!("largest is {}", largest);

    let v = vec!(12, 65, 45, 87, 3);

    let largest = find_largest(&v);

    println!("largest is {}", largest);
}

fn find_largest(v: &[i32]) -> i32 {
    let mut largest = v[0];

    for &i in v.iter() {
        if i > largest {
            largest = i;
        }
    }
    largest
}
