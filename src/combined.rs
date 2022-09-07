use std::ffi::CString;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use libc::{prctl, PR_SET_CHILD_SUBREAPER};
use nix::sys::wait::waitpid;
use nix::unistd::{execv, fork, getpid, getppid, ForkResult, Pid};

fn main() {
    println!("[main] Hi there! My pid is {}", getpid());
    println!("[main] Making myself a child subreaper.");
    unsafe {
        prctl(PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0);
    }

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("[main] Forked new child with pid {}", child);
        }
        Ok(ForkResult::Child) => {
            //////////////////////
            //      child 1     //
            //////////////////////
            println!(
                "[child 1] I'm alive! My PID is {} and PPID is {}.",
                getpid(),
                getppid()
            );

            match unsafe { fork() } {
                Ok(ForkResult::Child) => {
                    //////////////////////
                    //      child 2     //
                    //////////////////////
                    println!(
                        "[child 2] I'm alive! My PID is {} and PPID is {}.",
                        getpid(),
                        getppid()
                    );
                    println!("[child 2] Exec-ing...");
                    exec_or_die("target/debug/sleepy");
                }

                Ok(ForkResult::Parent { child, .. }) => {
                    println!("[child 1] I forked a child with PID {}.", child);
                }
                Err(err) => panic!("[child 1] fork failed: {}", err),
            }

            println!("[child 1] I'm gonna sleep for a while and then just exit...");
            sleep(Duration::from_millis(1500));
            exit(0);
        }
        Err(err) => panic!("main: fork failed: {}", err),
    };

    println!("[main] I'll be waiting for the child termination...");
    match waitpid(Pid::from_raw(-1), None) {
        Ok(status) => println!("[main] Child exited with status {:?}", status),
        Err(err) => println!("[main] waitpid() failed: {}", err),
    }

    println!("[main] I'll not be waiting for the grandchild though.");
    sleep(Duration::from_millis(1000));
    println!("[main] Bye Bye!");
}

fn exec_or_die(name: &str) {
    let name_cstr = CString::new(name).unwrap();
    match execv(&name_cstr, &vec![name_cstr.clone()]) {
        Ok(_) => unreachable!("execv() succeed! Wait, what?!"),
        Err(err) => unreachable!("execv() failed: {}", err),
    }
}
