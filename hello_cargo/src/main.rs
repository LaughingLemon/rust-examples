
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColour(i32, i32, i32)
}

fn main() {
    let msg = Message::ChangeColour(1, 2, 3);
    match msg {
        Message::Quit => (),
        Message::ChangeColour(r, g, b) => println!("msg: ChangeColour {}, {}, {}", r, g, b ),
        Message::Move {x, y} => println!("msg: Move {}, {}", x, y),
        Message::Write(txt) => println!("msg: Write {}", txt)
    }
}

