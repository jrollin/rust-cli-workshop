# Environnement Setup

## :dart: Objective 

Make your Rust setup environnement ready

* Initialize project
* compile and run 

## :pencil: Setup project with cargo

[Cargo](https://doc.rust-lang.org/cargo/) is the Rust Package Manager \
It manage dependencies, tasks, project creation, workspaces... (can be compare to Npm)

### Create a new application


```bash
cd /your/projects/directory/
cargo new crabby
```

Move to your application directory :

```bash
cd crabby
```

> :bulb: Notes 
>
> Cargo create a git repository with all structure by default  
> use `cargo new --help` for more options

### How to build application ?


Launch compilation with `cargo` command:

```bash
cargo build
```

Look at newly created directories and files: 

```bash
ls ./target/
```

A new `debug`directory have been created in `target`

You can execute built file :

```bash
./target/debug/crabby
```



Launch compilation with release profile:

```bash
cargo build --release
```

You have a new `release` directory in `./target`

```bash
ls ./target
.
.. 
debug
release
```

### Compile and Run

A common way to run code when developing is to use `cargo run`

```bash
cargo run

   Compiling crabby v0.1.0 (/your/project/crabby)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `/your/project/crabby/target/debug/crabby`

Hello, i am Crabby 🦀 !
```

cargo compiles with debug profile if needed and run compiled program

### How to test ?


Launch `cargo`:

```bash
cargo test 

    Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src/main.rs 

running 0 tests
```

:exclamation: No test are writtien yet...


## :pencil: Exercice

change the programm output with : 

```bash
Hello, i am Crabby 🦀 !
```

> :bulb: Notes
>
> * crab emoji is not an ASCII Char  
> * String are stored Utf8 encoded in Rust


## :clap: Congrats 

You have a working Rust setup !

Check the expected source code  [here](./solution/src/main.rs) 


## :memo: Summary 

What you have learned

* How to intialize project with cargo
* Build projet  
* Run project
* Test project


## :books: Additional resources 

* [Cargo profiles](https://doc.rust-lang.org/book/ch14-01-release-profiles.html)
* [Strings](https://doc.rust-lang.org/rust-by-example/std/str.html)
* [Rust string and UTF8](https://doc.rust-lang.org/book/ch08-02-strings.html)



### Next Part 
[:call_me_hand: Go to next part: Syntax](../2_syntax/README.md)
