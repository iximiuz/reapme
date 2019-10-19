use std::ffi::CString;
use std::process;
use std::thread;
use std::time::Duration;

use libc::{prctl, PR_SET_CHILD_SUBREAPER};
use nix::sys::wait::waitpid;
use nix::unistd::{execv, fork, getpid, ForkResult, Pid};

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
            // child 1
            println!("child (1) - I'm alive!");

            match fork() {
                Ok(ForkResult::Parent { child, .. }) => {
                    println!("child (1) - forked new child with pid {}", child);
                }
                Ok(ForkResult::Child) => {
                    // child 2
                    println!("child (2) - I'm alive!");
                    exec_or_die("/home/vagrant/forkme/target/debug/sleep");
                }
                Err(err) => panic!("child (1) - fork failed: {}", err),
            }

            // child 1 - continued
            thread::sleep(Duration::from_secs(5));
            println!("child (1) - exiting...");
            process::exit(0);
        }
        Err(err) => panic!("main: fork failed: {}", err),
    };

    // main - continued
    match waitpid(Pid::from_raw(-1), None) {
        Ok(ok) => println!("waitpid() (1) ok: {:?}", ok),
        Err(err) => println!("waitpid() (1) failed: {}", err),
    }
    match waitpid(Pid::from_raw(-1), None) {
        Ok(ok) => println!("waitpid() (2) ok: {:?}", ok),
        Err(err) => println!("waitpid() (2) failed: {}", err),
    }
}

fn exec_or_die(name: &str) {
    let name_cstr = CString::new(name).unwrap();
    match execv(&name_cstr, &vec![name_cstr.clone()]) {
        Ok(_) => unreachable!("execv() succeed! Wait, what?!"),
        Err(err) => unreachable!("execv() failed: {}", err),
    }
}
