use std::{sync::{Arc, Mutex}, thread};

fn main() {
    let num_simulations = 8;
    let target_flips = 10;
    let all_iterations = Arc::new(Mutex::new(Vec::new()));

    for _ in 0..num_simulations {
        let all_iterations = all_iterations.clone();
        thread::spawn(move || {
            let iterations = simulate(target_flips);
            let mut all_iterations = all_iterations.lock().unwrap();
            all_iterations.push(iterations);
        });
    }

    let mut last_completed = 0;
    loop {
        let all_iterations = all_iterations.lock().unwrap();
        if all_iterations.len() == num_simulations {
            break;
        }

        if last_completed < all_iterations.len() {
            println!("{all_iterations:?}");
            last_completed += 1;
        }
    }
}

fn simulate(target_flips: u64) -> u64 {
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
    iterations
}