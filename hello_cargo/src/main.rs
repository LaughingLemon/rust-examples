
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let x = Some(6);
    let y = 10;

    match x {
        Some(50) => println!("Some(50)"),
        Some(y) => println!("This is y {}", y),
        _ => println!("This is the default {:?}", x)
    }
    println!("Some(x): {:?}, y: {}", x, y);

    let x = 1;
    match x {
        1 | 2 => println!("One or two"),
        3 => println!("three"),
        _ => println!("something else")
    }

    let x = 3;
    match x {
        1..=5 => println!("One to five"),
        _ => println!("something else")
    }

    let p = Point { x: 0, y: 12 };

    match p {
        Point { x, y: 0 } => println!("On the y axis, x: {}", x),
        Point {x: 0, y} => println!("On the x axis, y: {}", y),
        Point { x, y} => println!("x:{}, y: {}", x, y)
    }
}

