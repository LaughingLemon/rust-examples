use std::fs::File;
use std::io::{Error, ErrorKind};

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(er) => panic!("Problem creating the file: {:?}", er)
            },
            other_error => panic!("Problem opeining the file: {:?}", other_error)
        }
    };
}
