use std::{sync::Arc, thread};

fn main() {
    let num_simulations = 8;
    let target_flips = 10;
    let mut num_completed = Arc::new(0);

    for _ in 0..num_simulations {
        thread::spawn(move || {
            simulate(target_flips);
            num_completed += 1;
        });
    }

    while num_completed < num_simulations {}
}

fn simulate(target_flips: u64) {
    let mut consecutive = 0;
    let mut iterations = 0;
    while consecutive < target_flips {
        iterations += 1;
        if rand::random_bool(0.5) {
            consecutive += 1;
        } else {
            consecutive = 0;
        }
    }
    println!("iterations: {iterations}");
}