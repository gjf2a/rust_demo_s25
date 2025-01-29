use std::env;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let filename = args[0].as_str();
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename).unwrap();
    for i in 1..args.len() {
        writeln!(file, "{}", args[i].as_str()).unwrap();
    }
}
