use std::process::{exit};
use std::thread::{sleep};
use std::time::Duration;

use libc::{prctl, PR_SET_CHILD_SUBREAPER};
use nix::sys::wait::waitpid;
use nix::unistd::{fork, ForkResult, getpid, getppid, Pid};

fn main() {
    println!("[main] Hi there! My PID is {}.", getpid());
    println!("[main] Making myself a child subreaper.");
    unsafe {
        prctl(PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0);
    }

    match fork() {
        Ok(ForkResult::Child) => {
            //////////////////////
            //      child 1     //
            //////////////////////
            println!("[child 1] I'm alive! My PID is {} and PPID is {}.", getpid(), getppid());

            match fork() {
                Ok(ForkResult::Child) => {
                    //////////////////////
                    //      child 2     //
                    //////////////////////
                    for _ in 0..6 {
                        println!("[child 2] I'm alive! My PID is {} and PPID is {}.", getpid(), getppid());
                        sleep(Duration::from_millis(500));
                    }
                    println!("[child 2] Bye Bye");
                    exit(0);
                }

                Ok(ForkResult::Parent { child, .. }) => {
                    println!("[child 1] I forked a grandchild with PID {}.", child);
                }

                Err(err) => panic!("[child 1] fork() failed: {}", err),
            };

            println!("[child 1] I'm gonna sleep for a while and then just exit...");
            sleep(Duration::from_millis(1500));
            exit(0);
        }

        Ok(ForkResult::Parent { child, .. }) => {
            println!("[main] I forked a child with PID {}.", child);
        }

        Err(err) => panic!("[main] fork() failed: {}", err),
    };

    println!("[main] I'll be waiting for the child termination...");
    match waitpid(Pid::from_raw(-1), None) {
        Ok(status) => println!("[main] Child exited with status {:?}", status),
        Err(err) => println!("[main] waitpid() failed: {}", err),
    }

    println!("[main] I'll be waiting for the grandchild termination as well...");
    sleep(Duration::from_millis(500));  // just in case
    match waitpid(Pid::from_raw(-1), None) {
        Ok(status) => println!("[main] Grandchild exited with status {:?}", status),
        Err(err) => println!("[main] waitpid() failed: {}", err),
    }
    println!("[main] Bye Bye!");
}

