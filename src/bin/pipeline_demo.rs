use std::ffi::CString;
use std::os::fd::AsRawFd;
use nix::unistd::{close, dup2, execvp, fork, pipe, ForkResult};
use nix::sys::wait::waitpid;

fn main() {
    println!("Executes ls -l | grep Cargo | wc");
    let ls = vec![CString::new("ls").unwrap(), CString::new("-l").unwrap()];
    let grep = vec![CString::new("grep").unwrap(), CString::new("Cargo").unwrap()];
    let wc = vec![CString::new("wc").unwrap()];

    match unsafe {fork()}.unwrap() {
        ForkResult::Parent { child } => {
            println!("first PID (wc) is {child}");
            waitpid(child, Option::None).unwrap();
            println!("Finished! Exited...");
        }
        ForkResult::Child => {
            let (wc_in, grep_out) = pipe().unwrap();

            match unsafe {fork()}.unwrap() {
                ForkResult::Parent { child } => {
                    close(grep_out.as_raw_fd()).unwrap();

                    println!("Second PID (grep) is {child}");
                    dup2(wc_in.as_raw_fd(), 0).unwrap();
                    execvp(&wc[0], &wc).unwrap();
                }
                ForkResult::Child => {
                    close(wc_in.as_raw_fd()).unwrap();
                    let (grep_in, ls_out) = pipe().unwrap();

                    match unsafe {fork()}.unwrap() {
                        ForkResult::Parent { child } => {
                            close(ls_out.as_raw_fd()).unwrap();

                            println!("Third PID (ls) is {child}");
                            dup2(grep_out.as_raw_fd(), 1).unwrap();
                            dup2(grep_in.as_raw_fd(), 0).unwrap();
                            execvp(&grep[0], &grep).unwrap();
                        }
                        ForkResult::Child => {
                            close(grep_in.as_raw_fd()).unwrap();

                            dup2(ls_out.as_raw_fd(), 1).unwrap();
                            execvp(&ls[0], &ls).unwrap();
                        }
                    }
                }
            }
        }
    }
}