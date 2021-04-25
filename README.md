# Guessing game in Rust!
This example is on [the Rust Book, chapter 2](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).

## Usage
To run the unoptimized version:
```bash
cargo run
```

To build the relase version:
```bash
cargo build --release
```

To check local documentation:
```bash
cargo doc --open
```

## Notes
- Rust doc comments use `///` and have Markdown support.
- `match` and `if` are expressions!
- In order to install dependencies, the official way is modifying `Cargo.toml` and manually adding dependencies.
- It is possible to use `cargo add <package's name>` with [`cargo-edit`](). I hope in the future, this subcommands will be available _out-of-the-box_ in Cargo. The command to install is:
```bash
cargo install cargo-edit

# Then, e.g.
cargo add rand
```
