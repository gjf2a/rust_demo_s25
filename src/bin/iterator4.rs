fn main() {
    let iter = CoinIterator {consecutive_flips: 0};
    let counts = iter
        .take(100000)
        .filter(|c| *c == 10)
        .count();
    println!("Number of 10 in first 100000: {}", counts);
}

struct CoinIterator {
    consecutive_flips: u64
}

impl Iterator for CoinIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.consecutive_flips;
        if rand::random_bool(0.5) {
            self.consecutive_flips += 1;
        } else {
            self.consecutive_flips = 0;
        }
        Some(current)
    }
}