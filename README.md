# sifter

`sifter` is a command-line password wordlist generator. It takes a configuration file containing relevant information and generates a list of possible passwords.
This is a work-in-progress project and is mainly intended for me to learn Rust.

## Table of Contents
- [TODO](#todo)
- [Future Features](#future-features)
- [Building](#building)
- [Components](#components)

## TODO
- [ ] Unit tests
- [ ] Documentation

## Future Features
- [ ] Add the option to provide a hash, hash type, and salt and have `sifter` generate a wordlist and immediately attempt to crack the hash
- [x] Combine multiple wordlists into one

## Building
To build `sifter`, run `cargo build --release`. The binary will be located in `target/release/sifter`.
To install `sifter`, run `cargo install --path .`.
To uninstall `sifter`, run `cargo uninstall sifter`.

## Components
| File | Description |
| ---- | ----------- |
| `src/main.rs` | The main program. |
| `src/combine.rs` | The wordlist combination logic. |
| `src/generator.rs` | The password generator. |
