use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1 .. 20 {
            println!("Hi! Number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(500));
        }
    });
    for i in 1 .. 5 {
        println!("Hi! Number {} from main thread", i);
        thread::sleep(Duration::from_secs(2));
    }

    handle.join().unwrap();
}

