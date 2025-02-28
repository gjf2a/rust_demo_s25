use std::{sync::{Arc, Mutex}, thread};

fn main() {
    let num_simulations = 8;
    let target_flips = 10;
    let num_completed = Arc::new(Mutex::new(0));
    let min_iterations = Arc::new(Mutex::new(u64::max_value()));

    for _ in 0..num_simulations {
        let num_completed = num_completed.clone();
        let min_iterations = min_iterations.clone();
        thread::spawn(move || {
            let iterations = simulate(target_flips);
            println!("iterations: {iterations}");
            {
                let mut min_iterations = min_iterations.lock().unwrap();
                if *min_iterations > iterations {
                    *min_iterations = iterations;
                }
            }

            let mut num_completed = num_completed.lock().unwrap();
            *num_completed += 1;
        });
    }

    loop {
        let num_completed = num_completed.lock().unwrap();
        if *num_completed == num_simulations {
            break;
        }
    }

    let min_iterations = min_iterations.lock().unwrap();
    println!("min iterations: {}", *min_iterations);
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