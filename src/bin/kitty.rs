use std::env;
use std::fs::read_to_string;

fn main() {
    for arg in env::args().skip(1) {
        let file_str = read_to_string(arg.as_str()).unwrap();
        println!("Contents of {arg}:");
        println!("{file_str}\n");
    }
}
