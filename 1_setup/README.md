# Environnement Setup

## :dart: Objective 

Make your Rust setup environnement ready


## :pencil: Step 1 - Setup 

### Create a new application

```bash
cargo new crabby
```

> :bulb: Notes 
>
> Cargo create a git repository with all structure by default  
> use `cargo new --help` for more options

### How to build application


Launch `cargo`:

```bash
cargo build
```

Look at newly created directories and files: 

```bash
ls ./target/
```

You can execute file `./target/debug/crabby` 


Launch `cargo`:

```bash
cargo build --release
```




> :bulb: Notes 
>
> Cargo have two predefined profiles
> * dev (used by default)
> * release 
> You can add one more profiles


### How to run application


Launch `cargo`:

```bash
cargo run 
```

:question:  Look at executed file 

`Running 'target/debug/crabby'`




> :bulb: Notes 
> 
> Cargo run builds app with dev profile and execute produced binary file


### How to test


Launch `cargo`:

```bash
cargo test 
```

:exclamation: No test are writtien yet...Coming soon


## :pencil: Step 2 - change output

change the programm output with : 

```bash
Hello, i am Crabby 🦀 !
```

> :bulb: Notes
>
> crab emoji is not an ASCII Char  
> String are stored Utf8 encoded in Rust


## :clap: Congrats, you have a working Rust setup !


## :memo: Summary 

What you have learned

* How to intialize project with cargo
* Build projet  
* Run project
* Test project


## :books: Resources 

* [Cargo profiles](https://doc.rust-lang.org/book/ch14-01-release-profiles.html)
* [Strings](https://doc.rust-lang.org/rust-by-example/std/str.html)
* [Rust string and UTF8](https://doc.rust-lang.org/book/ch08-02-strings.html)
