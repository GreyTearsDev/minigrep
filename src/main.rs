use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];
    println!("searching {}", query);
    println!("in {}", filename);

    let contents = fs::read_to_string("poem.txt").expect("Something went wrong reading the file");

    println!("With text: \n{}", contents);
}
