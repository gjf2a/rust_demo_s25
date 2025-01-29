use std::env;

fn main() {
    let mut total = 0;
    for arg in env::args().skip(1) {
        let arg_val = arg.parse::<i64>().unwrap();
        total += arg_val;
    }
    println!("total of arguments: {total}");
}
