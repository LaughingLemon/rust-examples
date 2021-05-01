use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_name: String
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query parameter not defined")
        };

        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("File_name parameter not defined")
        };

        Ok(Config { query, file_name })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
            .filter(|line|
                line.to_lowercase()
                    .contains(&query.to_lowercase()))
            .collect()
}

#[cfg(test)]
mod tests {
    use crate::search;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust: \nproductive, safe\nconcurrent";

        assert_eq!(vec!["productive, safe"], search(query, contents));
    }

    #[test]
    fn more_results() {
        let query = "body";
        let contents = "Nobody, but\nsomebody but\nnot just\nanybody";

        assert_eq!(vec!["Nobody, but", "somebody but", "anybody"], search(query, contents));
    }

    #[test]
    fn no_results() {
        let query = "thing";
        let contents = "Nobody, but\nsomebody but\nnot just\nanybody";
        let empty: Vec<&str> = Vec::new();

        assert_eq!(empty, search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "Thing";
        let contents = "This Thing, but\nsomething but\nnot just\nanything";

        assert_eq!(vec!["This Thing, but", "something but", "anything"], search(query, contents));
    }
}