use std::thread;
use std::time::Duration;

use libc::{prctl, PR_SET_PDEATHSIG};
use nix::sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet, SIGUSR1};
use nix::unistd::{getpid, getppid};

extern "C" fn handle_sigint(_: libc::c_int) {
    println!("sleepy program - parent died");
}

fn main() {
    println!("sleepy program - Hi there! My pid is {}", getpid());

    unsafe {
        prctl(PR_SET_PDEATHSIG, SIGUSR1);
    }
    let sig_action = SigAction::new(
        SigHandler::Handler(handle_sigint),
        SaFlags::empty(),
        SigSet::empty(),
    );

    if let Err(err) = unsafe { sigaction(SIGUSR1, &sig_action) } {
        println!("sigaction() failed: {}", err);
    };

    loop {
        let ppid = getppid();
        println!(
            "I'm a very sleepy program and my parent is {}. Zzzz...",
            ppid
        );
        thread::sleep(Duration::from_millis(2000));
    }
}
