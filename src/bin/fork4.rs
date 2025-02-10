use nix::{fcntl::{open, OFlag}, libc::STDOUT_FILENO, sys::{stat::Mode, wait::waitpid}, unistd::{close, execvp, fork, getpid, ForkResult}};
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
                close(STDOUT_FILENO).unwrap();
                open("wc_out.txt", OFlag::O_CREAT.union(OFlag::O_WRONLY).union(OFlag::O_TRUNC), Mode::all()).unwrap();
                println!("child (pid: {})", getpid());
                let mut argv = Vec::new();
                for arg in ["wc", "src/bin/fork4.rs"] {
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