[![Rust](https://github.com/jrollin/rust-cli-workshop/actions/workflows/rust.yml/badge.svg)](https://github.com/jrollin/rust-cli-workshop/actions/workflows/rust.yml)

![screenshot](app.png)

# Learn Rust by building Command Line Application

Repository used for workshop


## :dart:  What we will build

🦀 Crabby command line !

```
$ crabby --help

crabby 0.1.0
I am the crabby help usage.

USAGE:
    crabby <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    chifoumi    chifoumi with players
    greets      Greets with name
    help        Print this message or the help of the given subcommand(s)

```

## :pencil: Requirements

You'll need to install:

- [Rust installation instructions](https://www.rust-lang.org/tools/install)

TLDR;

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

:warning: Note on Windows

Rust requires certain C++ build tools.
You can either download the [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/), or (recommended) you might prefer just to install [Microsoft Visual Studio](https://visualstudio.microsoft.com/downloads/).

[More about Installation setup on windows](https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup)

### Verify your toolchain version

Minimum Version : 1.64+ 

```bash
rustc --version
```

```bash
rustc 1.64.0 (a55dd71d5 2022-09-19)
```

Keep your rust up-to-date with the latest release of rust, type:

```bash
rustup update
```


### Choose your IDE


* VS Code: 
    * [Rust language](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
    * [Rust Analyzer Language Server Protocol](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
    * [TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml)
* Jetbrains Rust: 
    * https://www.jetbrains.com/fr-fr/rust/
* Vim plugin : 
    * https://github.com/simrat39/rust-tools.nvim

[More tools on Rust website](https://www.rust-lang.org/tools)

## :pencil: Workshop instructions 


* [Part 1 - Env setup](./1_setup) 
* [Part 2 - Syntax](./2_syntax)
* [Part 3 - Command args and options](./3_args)
* [Part 4 - Modules and conversion](./4_mod)
* [Part 5 - Better command help documenter son application](./5_cli)
* [Part 6 - Api call with Json parsin (wip)](./6_api)


### :eyes: Solutions

> :exclamation: Try your solution first. Errors help to learn with Rust


Each part have a working solution


```bash
// inside each directory 
cd <partx>/solutions
cargo run 
```

Run solutions :


```bash
// from git root directory
cargo run --bin crabby_setup
cargo run --bin crabby_syntax
cargo run --bin crabby_args
cargo run --bin crabby_cli
```

> bin name is defined in related `<partx>/solutions/Cargo.toml` files


### :zap: Tests

> You are not required to write test during workshop but it always a good pratice so have a look !

```bash
// all
cargo test

//specific
cargo test --bin crabby_setup
cargo test --bin crabby_syntax
cargo test --bin crabby_args
cargo test --bin crabby_mod
cargo test --bin crabby_cli
```


## :books: Additional resources 

* [Rust Book](https://doc.rust-lang.org/book/)
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/) 

