use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("echo: {}", line.unwrap());
    }
}
