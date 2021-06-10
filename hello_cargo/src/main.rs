
fn main() {
    let x = Some(6);
    let y = 10;

    match x {
        Some(50) => println!("Some(50)"),
        Some(n) if n == y => println!("This is y {}", y),
        _ => println!("This is the default {:?}", x)
    }
    println!("Some(x): {:?}, y: {}", x, y);
}

