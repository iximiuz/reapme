use std::thread;
use std::time::Duration;

use nix::unistd::getppid;

fn main() {
    loop {
        let ppid = getppid();
        println!(
            "I'm a very sleepy program and my parent is {}. Zzzz...",
            ppid
        );
        thread::sleep(Duration::from_millis(2000));
    }
}
