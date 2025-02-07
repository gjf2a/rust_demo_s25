use nix::{sys::wait::waitpid, unistd::{execvp, fork, getpid, ForkResult}};
use std::{ffi::CString, process::exit};

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
                let mut argv = Vec::new();
                for arg in ["wc", "src/bin/fork3.rs"] {
                    argv.push(CString::new(arg).unwrap());
                }
                match execvp(&argv[0], &argv) {
                    Ok(_) => {
                        println!("This will never print!");
                    }
                    Err(e) => {
                        println!("Error: {e}");
                        exit(1);
                    }
                }
            }
        }
    }
}