use std::collections::BTreeMap;
use std::env;
use std::fs::read_to_string;


fn main() {
    let mut args = env::args();
    let my_name = args.next().unwrap();
    for arg in args {
        match word_histogram(arg.as_str()) {
            Ok(histogram) => {
                show_histogram(&histogram);
            }
            Err(e) => {
                println!("{my_name}: {arg}: {e}");
            }
        }
    }
}

fn word_histogram(filename: &str) -> std::io::Result<BTreeMap<String,u64>> {
    let mut histogram = BTreeMap::new();
    let file_text = read_to_string(filename)?;
    for word in file_text.split_whitespace() {
        match histogram.get_mut(word) {
            Some(current_count) => {
                *current_count += 1;
            }
            None => {
                histogram.insert(word.to_string(), 1);
            }
        }
    }
    Ok(histogram)
}

fn show_histogram(histogram: &BTreeMap<String, u64>) {
    let mut kv = histogram.iter().collect::<Vec<_>>();
    kv.sort_by_key(|(_, v)| **v);

    for (key, value) in kv.iter().rev() {
        println!("{key}: {value}");
    }
}