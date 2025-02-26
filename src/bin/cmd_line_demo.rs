use std::io;
use std::io::Write;

#[derive(Copy, Clone)]
enum Status {
    KeepGoing,
    Exit
}

pub struct WordStats {
    total_word_characters: usize,
    total_words: usize,
    longest: String,
    shortest: String,
}

impl WordStats {
    pub fn new() -> Self {
        Self {
            total_word_characters: 0,
            total_words: 0,
            longest: String::new(),
            shortest: String::new(),
        }
    }

    pub fn add_word(&mut self, word: &str) {
        self.total_words += 1;
        self.total_word_characters += word.len();
        if word.len() > self.longest.len() {
            self.longest = word.to_string();
        }
        if self.shortest.len() == 0 || word.len() < self.shortest.len() {
            self.shortest = word.to_string();
        }
    }

    pub fn average_word_length(&self) -> Option<f64> {
        if self.total_words == 0 {
            None
        } else {
            Some(self.total_word_characters as f64 / self.total_words as f64)
        }
    }

    pub fn longest_word(&self) -> &str {
        self.longest.as_str()
    }

    pub fn shortest_word(&self) -> &str {
        self.shortest.as_str()
    }
}

fn main() {
    let mut stats = WordStats::new();
    loop {
        match handle_one_input(&mut stats) {
            Ok(status) => {
                match status {
                    Status::KeepGoing => {}
                    Status::Exit => break,
                }
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
    }

    println!("Fare thee well!");
}

fn handle_one_input(stats: &mut WordStats) -> anyhow::Result<Status> {
    print!("$ ");
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    if line.trim() == "exit" {
        Ok(Status::Exit)
    } else {
        for word in line.split_whitespace() {
            stats.add_word(word);
        }
        match stats.average_word_length() {
            None => {
                println!("No words yet entered; average is undefined.");
            }
            Some(average) => {
                println!("Average word length: {average:.2}");
                println!("Longest word: {}", stats.longest_word());
                println!("Shortest word: {}", stats.shortest_word());
            }
        }
        
        Ok(Status::KeepGoing)
    }
}