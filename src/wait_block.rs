use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use nix::sys::wait::waitpid;
use nix::unistd::{fork, getpid, getppid, ForkResult};

fn main() {
    println!("[main process] Hi there! My PID is {}.", getpid());

    let child_pid = match fork() {
        Ok(ForkResult::Child) => {
            //////////////////////
            //      child       //
            //////////////////////
            println!(
                "[child process] I'm alive! My PID is {} and PPID is {}.",
                getpid(),
                getppid()
            );

            println!("[child process] I'm gonna sleep for a while and then just exit...");
            sleep(Duration::from_secs(2));
            exit(0);
        }

        Ok(ForkResult::Parent { child, .. }) => {
            println!("[main process] I forked a child with PID {}.", child);
            child
        }

        Err(err) => {
            panic!("[main process] fork() failed: {}", err);
        }
    };

    println!("[main process] I'll be waiting for the child termination...");
    match waitpid(child_pid, None) {
        Ok(status) => println!("[main process] Child exited with status {:?}", status),
        Err(err) => panic!("[main process] waitpid() failed: {}", err),
    }
    println!("[main process] Bye Bye!");
}
