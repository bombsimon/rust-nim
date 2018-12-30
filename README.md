# Rust Nim

Another implementation like the one in
[Elixir](https://github.com/bombsimon/elixir-nm) or
[Java](https://github.com/bombsimon/Java-Example/tree/master/Nm) to try out
Rust.

## Installation

Install Rust and Cargo:

```sh
curl https://sh.rustup.rs -sSf | sh
```

Install rust-nim:

```sh
cargo install --path .
```

## Usage

Assuming you've `~/.cargo/bin` in your `$PATH`:

```sh
rust-nim 10

There are 10 objects left in the stack
Draw some: 4
User removed 4 from stack.
Computer removed 2 from stack.
There are 4 objects left in the stack
Draw some: 1
User removed 1 from stack.
There are 2 objects left in the stack
Draw some: 1
User removed 1 from stack.
Player wins!
```
