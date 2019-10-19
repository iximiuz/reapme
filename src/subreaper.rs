use libc::{prctl, PR_SET_CHILD_SUBREAPER};

use nix::unistd::{getpid};

fn main() {
    println!("main - Hi there! My pid is {}", getpid());
    unsafe {
        prctl(PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0);
    }
}

