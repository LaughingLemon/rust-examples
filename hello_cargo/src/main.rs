use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let vals = vec![
            String::from("something"),
            String::from("something"),
            String::from("oranges"),
            String::from("something"),
        ];
        println!("send something {:?}", vals);

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    for rec in rx {
        println!("get something {}", rec);
    }

    handle.join().unwrap();
}

