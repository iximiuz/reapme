use std::ffi::{c_void};
use std::thread;
use std::time::Duration;

use libc::{prctl, PR_SET_PDEATHSIG, STDOUT_FILENO, write};
use nix::sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet, SIGUSR1};
use nix::unistd::{getpid, getppid, Pid};

extern "C" fn handle_sigusr1(_: libc::c_int) {
    print_signal_safe("[sleepy] Parent died!\n");
}

fn main() {
    println!("[sleepy] Hi there! My pid is {}", getpid());
    println!("[sleepy] Binding SIGUSR1 to the parent termination event");
    unsafe {
        prctl(PR_SET_PDEATHSIG, SIGUSR1);
    }

    let sig_action = SigAction::new(
        SigHandler::Handler(handle_sigusr1),
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

fn print_signal_safe(s: &str) {
    unsafe {
        write(STDOUT_FILENO, s.as_ptr() as (* const c_void), s.len());
    }
}
