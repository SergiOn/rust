## Tokio Tutorial (Mini-Redis)

https://tokio.rs/tokio/tutorial

#### About

This tutorial will take you step by step through the process of building a Redis client and server.
We will start with the basics of asynchronous programing with Rust and build up from there.
We will implement a subset of Redis commands but will get a comprehensive tour of Tokio.

#### Links

https://github.com/tokio-rs/tokio

https://redis.io

https://github.com/tokio-rs/mini-redis

https://crates.io

#### Commands

```bash
rustc --version
cargo install mini-redis
```
```bash
mini-redis-server
mini-redis-cli get foo
```
```bash
cargo new my-redis
cd my-redis
```

