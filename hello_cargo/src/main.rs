fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("This is {}", i);
    }

    let four = &v[3];

    v.push(7);

    println!("This is {}", four);
}
