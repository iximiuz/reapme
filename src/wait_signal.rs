use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use nix::sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet, SIGCHLD};
use nix::sys::wait::waitpid;
use nix::unistd::{fork, getpid, getppid, ForkResult, Pid};

extern "C" fn handle_sigchld(_: libc::c_int) {
    println!("[main process] What a surprise! Got SIGCHLD!");
    match waitpid(Pid::from_raw(-1), None) {
        Ok(status) => println!("[main process] Child exited with status {:?}", status),
        Err(err) => panic!("[main process] waitpid() failed: {}", err),
    }
    println!("[main process] Bye Bye!");
    exit(0);
}

fn main() {
    println!("[main process] Hi there! My PID is {}.", getpid());

    match fork() {
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
        }

        Err(err) => {
            panic!("[main process] fork() failed: {}", err);
        }
    };

    let sig_action = SigAction::new(
        SigHandler::Handler(handle_sigchld),
        SaFlags::empty(),
        SigSet::empty(),
    );

    if let Err(err) = unsafe { sigaction(SIGCHLD, &sig_action) } {
        panic!("[main process] sigaction() failed: {}", err);
    };

    println!("[main process] I'll be doing my own stuff...");
    loop {
        println!("[main process] Do my own stuff.");
        // ... replace sleep with the payload
        sleep(Duration::from_millis(500));
    }
}
