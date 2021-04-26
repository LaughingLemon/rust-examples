use std::collections::HashMap;

fn main() {
    let mut h = HashMap::new();

    h.insert(String::from("key 1"), 1);
    h.insert(String::from("key 2"), 2);

    let k1 = String::from("key 3");
    let v1 = 3;

    let mut h1 = HashMap::new();

    h1.insert(k1, v1);

    println!("{}", k1);
}
