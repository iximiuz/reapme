use std::ffi::c_void;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use libc::{_exit, write, STDOUT_FILENO};
use nix::sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet, SIGCHLD};
use nix::sys::wait::waitpid;
use nix::unistd::{fork, getpid, getppid, ForkResult, Pid};

extern "C" fn handle_sigchld(_: libc::c_int) {
    print_signal_safe("[main] What a surprise! Got SIGCHLD!\n");
    match waitpid(Pid::from_raw(-1), None) {
        Ok(_) => {
            print_signal_safe("[main] Child exited.\n");
            print_signal_safe("[main] Bye Bye!\n");
            exit_signal_safe(0);
        }
        Err(_) => {
            print_signal_safe("[main] waitpid() failed.\n");
            exit_signal_safe(1);
        }
    }
}

fn main() {
    println!("[main] Hi there! My PID is {}.", getpid());

    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            //////////////////////
            //      child       //
            //////////////////////
            println!(
                "[child] I'm alive! My PID is {} and PPID is {}.",
                getpid(),
                getppid()
            );

            println!("[child] I'm gonna sleep for a while and then just exit...");
            sleep(Duration::from_secs(2));
            exit(0);
        }

        Ok(ForkResult::Parent { child, .. }) => {
            println!("[main] I forked a child with PID {}.", child);
        }

        Err(err) => {
            panic!("[main] fork() failed: {}", err);
        }
    };

    let sig_action = SigAction::new(
        SigHandler::Handler(handle_sigchld),
        SaFlags::empty(),
        SigSet::empty(),
    );

    if let Err(err) = unsafe { sigaction(SIGCHLD, &sig_action) } {
        panic!("[main] sigaction() failed: {}", err);
    };

    println!("[main] I'll be doing my own stuff...");
    loop {
        println!("[main] Do my own stuff.");
        // ... replace sleep with the payload
        sleep(Duration::from_millis(500));
    }
}

fn print_signal_safe(s: &str) {
    unsafe {
        write(STDOUT_FILENO, s.as_ptr() as (*const c_void), s.len());
    }
}

fn exit_signal_safe(status: i32) {
    unsafe {
        _exit(status);
    }
}
