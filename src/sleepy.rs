use std::thread;
use std::time::Duration;

use libc::{prctl, PR_SET_PDEATHSIG};
use nix::sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet, SIGUSR1};
use nix::unistd::{getpid, getppid, Pid};

extern "C" fn handle_sigint(_: libc::c_int) {
    println!("[sleepy] Parent died!");
}

fn main() {
    println!("[sleepy] Hi there! My pid is {}", getpid());
    println!("[sleepy] Binding SIGUSR1 to the parent termination event");
    unsafe {
        prctl(PR_SET_PDEATHSIG, SIGUSR1);
    }

    let sig_action = SigAction::new(
        SigHandler::Handler(handle_sigint),
        SaFlags::empty(),
        SigSet::empty(),
    );

    if let Err(err) = unsafe { sigaction(SIGUSR1, &sig_action) } {
        println!("[sleepy] sigaction() failed: {}", err);
    };

    loop {
        let ppid = getppid();
        println!("[sleepy] My parent is {}. Zzzz...", ppid);
        thread::sleep(Duration::from_millis(500));
        if ppid == Pid::from_raw(1) {
            println!("[sleepy] My parent is init process. I don't like it much...");
            break;
        }
    }

    println!("[sleepy] Bye Bye!");
}
