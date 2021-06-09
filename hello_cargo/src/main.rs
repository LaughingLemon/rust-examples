use hello_cargo::{Draw, Screen};

pub struct Button {
    pub width: i32,
    pub height: i32,
    pub label_text: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw Button, w: {}, h: {}, labelText: {}",
                 self.width, self.height, self.label_text);
    }
}

fn main() {
    let screen = Screen {
        components: vec![Box::new(Button{
            width: 10,
            height: 50,
            label_text: "Something".to_string()
        })]
    };

    screen.run();
}

