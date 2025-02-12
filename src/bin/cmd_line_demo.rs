use std::io;
use std::io::Write;

#[derive(Copy, Clone)]
enum Status {
    KeepGoing,
    Exit
}

fn main() {
    loop {
        match handle_one_input() {
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

fn handle_one_input() -> anyhow::Result<Status> {
    print!("$ ");
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    if line.trim() == "exit" {
        Ok(Status::Exit)
    } else {
        let mut longest = "";
        for word in line.split_whitespace() {
            if word.len() > longest.len() {
                longest = word;
            }
        }
        println!("Longest word: {longest}");
        Ok(Status::KeepGoing)
    }
}