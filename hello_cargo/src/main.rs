enum Colour {
    Rgb(i32, i32, i32),
    Hsl(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColour(Colour)
}

fn main() {
    let msg = Message::ChangeColour(Colour::Rgb(1, 2, 4));
    match msg {
        Message::ChangeColour(Colour::Rgb(r, g, b)) =>
            println!("msg: ChangeColour, RGB {}, {}, {}", r, g, b ),
        Message::ChangeColour(Colour::Hsl(h, s, l)) =>
            println!("msg: ChangeColour, HSL {}, {}, {}", h, s, l ),
        _ => ()
    }
}

