
fn main() {
    let num_list = vec!(1, 3, 7, 3, 12, 54, 34, 67, 2);

    let result = largest(&num_list);

    println!("largest is {}", result);

    let char_list = vec!('e','G', 'b', 'L');

    let result = largest(&char_list);

    println!("largest is {}", result);
}

fn largest<T: PartialOrd>(v: &[T]) -> &T {
    let mut largest = &v[0];

    for i in v.iter() {
        if i > largest {
            largest = i;
        }
    }
    largest
}
