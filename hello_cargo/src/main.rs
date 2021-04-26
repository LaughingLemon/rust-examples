fn main() {
    let s1 = String::from("hello ");

    let s2= String::from("world");
    let s3 = s1 + &s2;

    println!("This is {}, {}", s2, s3);

    let s4 = format!("{}, {}", s2, s3);
    println!("{}", s4);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
