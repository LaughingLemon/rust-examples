
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

fn longest<'a>(first: &'a str, next: &'a str) -> &'a str {
    if first.len() > next.len() {
        first
    } else {
        next
    }
}