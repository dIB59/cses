# 🦀 CSES Problem Set in Rust

This repository contains my solutions to problems from the [CSES Problem Set](https://cses.fi/problemset/), implemented in Rust.
I'm using this project as a learning exercise to improve my Rust and competitive programming skills.

## 📁 Project Structure
```
.
├── Cargo.toml
├── Cargo.lock
└── src
    ├── lib.rs
    ├── main.rs
    └── problems
        ├── mod.rs
        ├── mweird_algorithmod.rs
        └── etc
```

- All problems live in the `src/problems/` directory.
- `mod.rs` is used to register each problem module.
- `main.rs` is used to run the problems — edit it to call whichever solution you want to test.
- `lib.rs` can be used for shared utilities or reused logic across problems.

## ▶️ How to Run

Make sure Rust is installed. If not, install it from [rust-lang.org](https://www.rust-lang.org/tools/install).

Clone and run:

```bash
git clone https://github.com/your-username/cses-rust.git
cd cses-rust
cargo run
```
