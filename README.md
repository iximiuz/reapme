# reapme - playing with Linux processes termination in Rust

The project covers the following scenarios:

- awaiting a child process termination;
- awaiting a grandchild process termination;
- catching the parent process termination.

Read more about it in my <a href="https://iximiuz.com/en/posts/dealing-with-processes-termination-in-Linux/">blog</a>.

## Usage
```bash
cargo build

# wait for the child termination
cargo run --bin wait_block

# wait for the child termination while busy looping
cargo run --bin wait_busy

# wait for the child termination signaled to you
cargo run --bin wait_signal

# test Linux CHILD_SUBREAPER feature
cargo run --bin subreaper

# put all things together
cargo run --bin combined
```

