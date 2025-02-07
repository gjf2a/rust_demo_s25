use nix::unistd::{fork, getpid, ForkResult};

fn main() {
    println!("hello {}", getpid());
    match unsafe {fork()} {
        Err(e) => {
            println!("Fork failed: {e}");
        }
        Ok(fork_value) => match fork_value {
            ForkResult::Parent { child } => {
                println!("parent of {child} ({})", getpid());
            }
            ForkResult::Child => {
                println!("child (pid: {})", getpid());
            }
        }
    }
}