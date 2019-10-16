use std::ffi::CString;
use std::process;
use std::thread;
use std::time::Duration;

use libc::{prctl, PR_SET_CHILD_SUBREAPER};
use nix::unistd::{execv, fork, ForkResult, getpid};

fn main() {
    println!("main - Hi there! My pid is {}", getpid());

    unsafe {
        prctl(PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0);
    }

    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("main - forked new child with pid {}", child);
        }
        Ok(ForkResult::Child) => {
            println!("child (1) - I'm alive!");

            match fork() {
                Ok(ForkResult::Parent { child, .. }) => {
                    println!("child (1) - forked new child with pid {}", child);
                    println!("child (1) - exiting...");
                    process::exit(0);
                }
                Ok(ForkResult::Child) => {
                    println!("child (2) - I'm alive!");
                    exec_or_die("/home/vagrant/conman/target/debug/sleep");
                }
                Err(err) => panic!("child (1) - fork failed: {}", err),
            }
        }
        Err(err) => panic!("main: fork failed: {}", err),
    };

    thread::sleep(Duration::from_millis(9999999));
}

fn exec_or_die(name: &str) {
    let name_cstr = CString::new(name).unwrap();
    match execv(&name_cstr, &vec![name_cstr.clone()]) {
        Ok(_) => unreachable!("execv() succeed! Whait, what?!"),
        Err(err) => unreachable!("execv() faield: {}", err),
    }
}
