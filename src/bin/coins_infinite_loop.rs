use std::thread;

fn main() {
    let num_simulations = 8;
    let target_flips = 10;

    for _ in 0..num_simulations {
        thread::spawn(move || {
            simulate(target_flips);
        });
    }

    loop {}
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