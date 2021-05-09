use std::ops::Deref;

struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x : T) -> MyBox<T> {
        MyBox(x)
    }
}

impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name : &str) {
    println!("hello {}", name);
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    println!("x is {}", x);
    println!("y is {}", *y);

    let my_name = MyBox::new(String::from("Shaun"));
    hello(&my_name);
}

