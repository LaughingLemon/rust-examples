use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let val = String::from("something");
        println!("send something {}", val);
        tx.send(val).unwrap();
    });

    let val = rx.recv().unwrap();
    println!("get something {}", val);

    handle.join().unwrap();
}

