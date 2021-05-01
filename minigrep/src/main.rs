use std::{env, process};

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem passing args {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Problem opening file {}", e);
        process::exit(1);
    };
}
