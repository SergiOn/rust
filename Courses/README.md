# Rust courses

## The Rust Programming Language

https://www.udemy.com/course/rust-lang

#### About

Learn a modern, powerful yet safe systems programming language!

#### Links

https://www.rust-lang.org

https://www.rust-lang.org/learn/get-started

https://www.rust-lang.org/tools/install

#### Commands

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
```bash
info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

It will add the cargo, rustc, rustup and other commands to
Cargo's bin directory, located at:

  /Users/serhii/.cargo/bin

This can be modified with the CARGO_HOME environment variable.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  /Users/serhii/.rustup

This can be modified with the RUSTUP_HOME environment variable.

This path will then be added to your PATH environment variable by
modifying the profile files located at:

  /Users/serhii/.profile
/Users/serhii/.bash_profile

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: x86_64-apple-darwin
     default toolchain: stable
               profile: default
  modify PATH variable: yes
```
```bash
info: profile set to 'default'
info: default host triple is x86_64-apple-darwin
info: syncing channel updates for 'stable-x86_64-apple-darwin'
info: latest update on 2020-01-30, rust version 1.41.0 (5e1a79984 2020-01-27)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
 12.0 MiB /  12.0 MiB (100 %)   4.7 MiB/s in  2s ETA:  0s
info: downloading component 'rust-std'
 15.5 MiB /  15.5 MiB (100 %)   4.7 MiB/s in  3s ETA:  0s
info: downloading component 'rustc'
 54.4 MiB /  54.4 MiB (100 %)   4.7 MiB/s in 11s ETA:  0s
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 12.0 MiB /  12.0 MiB (100 %)   4.7 MiB/s in  2s ETA:  0s
info: installing component 'rust-std'
info: installing component 'rustc'
 54.4 MiB /  54.4 MiB (100 %)  14.9 MiB/s in  3s ETA:  0s
info: installing component 'rustfmt'
info: default toolchain set to 'stable'

  stable installed - rustc 1.41.0 (5e1a79984 2020-01-27)


Rust is installed now. Great!

To get started you need Cargo's bin directory ($HOME/.cargo/bin) in your PATH
environment variable. Next time you log in this will be done
automatically.

To configure your current shell run source $HOME/.cargo/env
```
```bash
rustup
cargo
```

```bash
rustc hello.rs
./hello
```
```bash
cargo build
cargo run
cargo new hello_world --bin
```


## Network Programming with Rust

https://www.udemy.com/course/draft/1842780/learn/lecture/11319368

#### About

Build fast and resilient network servers and clients by leveraging Rust's memory-safety and concurrency features

