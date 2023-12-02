Advent of Code 2023
===================

## Set up a Cargo project for a new day

- `cargo new dayX`
- Copy `main.rs.template` to `dayX/src/main.rs`.

## Run Cargo commands for a single day

Use the `-p` option to specify the package to use with cargo run/test:

```bash
cargo test -p day1
cargo run --release -p day1 -- day1/input.txt
```
