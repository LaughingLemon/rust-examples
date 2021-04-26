use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::io;

fn main() {
    match read_user_name() {
        Ok(user_name) => println!("{}", user_name),
        Err(err) => panic!("Unable to read username, {}", err)
    };
}

fn read_user_name() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
