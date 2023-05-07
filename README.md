# Rust Experiment
*First attempt at Rust*

### Goal
Try to build a [LSD](https://github.com/lsd-rs/lsd) clone with as few bells or whistles as possible.
Replicate the output of `lsd -lA --group-directories-first` as best as possible with no configurability.

### Run
`cargo install --path .`
`cargo run`

### Build
`cargo build -r`
Output binary will by default be in `./target/release/rust-ls`

#### Test
`./target/release/rust-ls`