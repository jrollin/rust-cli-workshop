# Add help and usage

What about features not in standard Rust Library ?\
Our cli can manage many subcommands but usage is not documented\
How to build a cli with nice usage help  ?


## :dart: Objectives

* add external libraries from crates.io
* use Clap as command line argument parser


## Import Modules 

Rust provides many libraries out of the box but at a time, we need specifics features

[Crates.io](https://crates.io/) is the defaut public repository where to search modules

:bulb: Notes 

What about playing chifoumi with computer as opponent ?

### Add a lib to generate random number

2 ways to add dependency

### Option 1: Use cargo cli 

```bash
cargo add rand

Updating crates.io index
      Adding rand v0.8.5 to dependencies.
             Features:
             + alloc
             + getrandom
             + libc
             + rand_chacha
             + std
             + std_rng
             - log
             - min_const_gen
             - nightly
             - packed_simd
             - serde
             - serde1
             - simd_support
             - small_rng

```

:books: Additional resources

* [cargo add documentation](https://doc.rust-lang.org/cargo/commands/cargo-add.html) (ex: specify version, features,etc)


### Option 2: Add dependency manually 

Add dependency in `cargo.toml`
```toml
[dependencies]
rand = "0.8.5"
```


Example of usage

 ```rust
 use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    println!("Random u8: {}", n1);

    println!("Integer de 0 to 3 included: {}", rng.gen_range(0..=3));
}

 ```

:books: Additional resources

* [rand documentation](https://docs.rs/crate/rand/latest)

## Use Clap to parse our cli arguements

[Clap](https://docs.rs/clap/latest/clap/) a commonly used crate for Cli application in Rust

With Derive feature, we can setup a Cli in few lines 


```rust
// from clap doc.rs
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[clap(short, long, value_parser)]
   name: String,

   /// Number of times to greet
   #[clap(short, long, value_parser, default_value_t = 1)]
   count: u8,
}

fn main() {
   let args = Args::parse();

   for _ in 0..args.count {
       println!("Hello {}!", args.name)
   }
}
```



```bash
$ demo --help
clap [..]
A simple to use, efficient, and full-featured Command Line Argument Parser

USAGE:
    demo[EXE] [OPTIONS] --name <NAME>

OPTIONS:
    -c, --count <COUNT>    Number of times to greet [default: 1]
    -h, --help             Print help information
    -n, --name <NAME>      Name of the person to greet
    -V, --version          Print version information

```

```bash
$ demo --name Me
Hello Me!
```

:bulb: Notes

* You can describe commands without annotation if you want


:books: Additional resources

* [Clap doc](https://docs.rs/clap/latest/clap/)
* [Clap github](https://github.com/clap-rs/clap)

### Focus on crates features

Cargo "features" provide a mechanism to express conditional compilation and optional dependencies.

```bash
cargo add clap

Updating crates.io index
Adding clap v4.2.1 to dependencies.
     Features:
     + color
     + error-context
     + help
     + std
     + suggestions
     + usage
     - cargo
     - debug
     - deprecated
     - derive
     - env
     - string
     - unicode
     - unstable-doc
     - unstable-v5
     - wrap_help
```
Note that `Derive` Feature is no enabled

:books: Additonal resource

* [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html)

#### Option 1: use cargo add 

use cargo add option -F (or --features)

```bash
cargo add clap -F derive
```


#### Option 2: Edit Cargo.toml

Update dependencies

```toml
[dependencies]
clap = { version = "4.2.1", features = ["derive"] }
```


## Clap provides subcommand 

We want to have distinct options for each cli functionality (greets and chifoumi)\
Clap provides functionality to create subcommand with struct and enum

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    author = "Julien Rollin",
    version = "1.0.0",
    about = "Crabby cli",
    long_about = None
)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Add { name: Option<String> },
    /// Doc about other command
    Greets {
        /// Name of the person to greet
        #[clap(short, long, value_parser)]
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Add { name } => {
            println!("'myapp add' was used, name is: {:?}", name)
        }
        Commands::Greets { name } => {
            println!("Name: {:?}", name)
        }
    }
}
```

Try this command by yourself 

```bash 
cargo run 

cargo run -- -V

cargo run -- greets --help
```

## :pencil: Exercice 1: Refactor your cli code with Clap

Refactor your app code with Clap and Subcommand

you shoud see beautiful cli usage when running your app

```bash
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

## :pencil: Exercice 2:  Play Chifoumi against computer

If no player two is provided use a random game value for player 2

```bash 
cargo run -- chifoumi -a paper

p1: Paper vs p2: Scissors => Lost

```


## :clap: Congrats

you can imagine building any cli now !

Check a solution with unit tests [here](./solution/src/main.rs) 

## :pencil: Summary

What you learned 

* add crate with features
* parse and document usage of cli app with clap

## Next Part

[:call_me_hand: Go to next part: API](../6_api/README.md)


