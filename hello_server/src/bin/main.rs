use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::{fs, thread};
use std::time::Duration;

use hello_server::ThreadPool;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:8085").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listner.incoming().take(2) {
        let stream = stream.unwrap();

        println!("New Stream");

        pool.execute(|| {
            handle_connection(stream);
        });
    }

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get_text = b"GET / HTTP/1.1\r\n";
    let sleep_text = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, file_name) = if buffer.starts_with(get_text) {
        println!("GET /");
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep_text) {
        println!("GET /sleep");
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK\r\n\r\n", "sleeping.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(file_name).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
