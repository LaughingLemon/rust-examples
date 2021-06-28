use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::{fs, thread};
use std::time::Duration;

use hello_server::ThreadPool;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:8085").unwrap();

    for stream in listner.incoming() {
        let stream = stream.unwrap();
        let pool = ThreadPool::new(4);

        pool.execute(|| {
            handle_connection(stream);
        });
    }

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    println!("Request {}", String::from_utf8_lossy(&buffer[..]));

    let get_text = b"GET / HTTP/1.1\r\n";
    let sleep_text = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, contents) = if buffer.starts_with(get_text) {
        ("HTTP/1.1 200 OK\r\n\r\n", fs::read_to_string("hello.html").unwrap())
    } else if buffer.starts_with(sleep_text) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK\r\n\r\n", fs::read_to_string("hello.html").unwrap())
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", fs::read_to_string("404.html").unwrap())
    };
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
