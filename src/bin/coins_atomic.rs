use std::{sync::Arc, thread};
use crossbeam::atomic::AtomicCell;

fn main() {
    let num_simulations = 8;
    let target_flips = 10;
    let num_completed = Arc::new(AtomicCell::new(0));
    let min_iterations = Arc::new(AtomicCell::new(u64::max_value()));

    for _ in 0..num_simulations {
        let num_completed = num_completed.clone();
        let min_iterations = min_iterations.clone();
        thread::spawn(move || {
            let iterations = simulate(target_flips);
            println!("iterations: {iterations}");
            let old_min = min_iterations.load();
            if old_min > iterations {
                loop {
                    let result = min_iterations.compare_exchange(old_min, iterations);
                    if result.is_ok() {break;}
                }
            }

            let last_completed = num_completed.load();
            loop {
                let result = num_completed.compare_exchange(last_completed, last_completed + 1);
                if result.is_ok() {break;}
            }
        });
    }

    loop {
        if num_completed.load() == num_simulations {
            break;
        }
    }

    println!("min iterations: {}", min_iterations.load());
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