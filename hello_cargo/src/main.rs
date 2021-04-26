fn main() {
    let mut s1 = String::from("hello ");

    let s2= "world";
    s1.push_str(s2);

    println!("This is {}, {}", s2, s1);
}
