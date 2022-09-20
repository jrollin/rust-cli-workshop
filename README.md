![](app.png)

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

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    greets    Say my name !
    help      Prints this message or the help of the given subcommand(s)
    learn     I can learn things

```

## :pencil: Requirements

You'll need to install:

- [Rust](https://www.rust-lang.org/tools/install)

TLDR 

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Verify your toolchain version

Minimum Version : 1.63+

```bash
rustc --version
```

Keep your rust up-to-date with the latest release of rust, type:

```bash
rustup update
```


### Choose your IDE


* VS Code: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust
* Jetbrains Rust: https://www.jetbrains.com/fr-fr/rust/
* Vim plugin : https://github.com/simrat39/rust-tools.nvim

[More tools on Rust website](https://www.rust-lang.org/tools)

## :pencil: Workshop instructions 


* [Part 1 - Env setup](./1_setup) 
* [Part 2 - Syntax](./2_syntax)
* [Part 3 - Command args and options](./3_args)
* [Part 4 - Better command help documenter son application](./4_cli)
* [Part 5 - Api call with Json parsing](./5_api)


### :eyes: Solutions

> :exclamation: Try your solution first. Errors help to learn with Rust


Each part have a working solution


```bash
// inside each directory 
cd solutions
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

> bin name is defined in related `Cargo.toml` files

Run tests:


```bash
// all
cargo test

//specific
cargo test --bin crabby_setup
cargo test --bin crabby_syntax
cargo test --bin crabby_args
cargo test --bin crabby_cli
```

