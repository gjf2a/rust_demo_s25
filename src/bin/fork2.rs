use nix::{sys::wait::waitpid, unistd::{fork, getpid, ForkResult}};

fn main() {
    println!("hello {}", getpid());
    match unsafe {fork()} {
        Err(e) => {
            println!("Fork failed: {e}");
        }
        Ok(fork_value) => match fork_value {
            ForkResult::Parent { child } => {
                match waitpid(child, None) {
                    Ok(status) => {
                        println!("parent of {child} ({}) status: {status:?}", getpid());
                    }
                    Err(e) => {
                        println!("I can't wait! {e}");
                    }
                }
            }
            ForkResult::Child => {
                println!("child (pid: {})", getpid());
            }
        }
    }
}