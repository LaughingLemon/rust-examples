use std::collections::HashMap;

fn main() {
    let mut h = HashMap::new();

    h.insert(String::from("key 1"), 1);
    h.insert(String::from("key 2"), 2);

    let k1 = String::from("key 3");
    let v1 = 3;

    let mut h1 = HashMap::new();

    h1.insert(k1, v1);

    println!("{:?}", h1);

    h1.insert("key 3".to_string(), 12);

    println!("{:?}", h1);

    h1.entry("key 3".to_string()).or_insert(13);
    h1.entry("key 4".to_string()).or_insert(10);

    println!("{:?}", h1);

    let v2 = h1.entry("key 4".to_string()).or_insert(10);
    *v2 += 5;

    println!("{:?}", h1);
}
