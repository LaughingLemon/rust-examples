use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx);

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

    let handle2 = thread::spawn(move || {
        let vals = vec![
            String::from("send"),
            String::from("three"),
            String::from("and"),
            String::from("fourpence"),
        ];
        println!("send something {:?}", vals);

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    for rec in rx {
        println!("get something {}", rec);
    }

    handle.join().unwrap();
    handle2.join().unwrap();
}

