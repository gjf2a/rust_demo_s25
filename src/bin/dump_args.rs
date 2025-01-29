use std::env;
use std::fs::write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    for i in 2..args.len() {
        write(filename, args[i].as_str()).unwrap();
    }
}
