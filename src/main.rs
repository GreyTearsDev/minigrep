use std::{env, fs};

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(query: String, filename: String) -> Config {
        Config { query, filename }
    }
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config::new(query, filename)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("searching {}", config.query);
    println!("in {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text: \n{}", contents);
}
